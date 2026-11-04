use std::collections::BTreeSet;

const CANONICAL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../reference/xiaomi-nabu/coverage.toml"
));

#[derive(Debug, PartialEq, Eq)]
pub struct ImageIdentity<'a> {
    pub name: &'a str,
    pub size: usize,
    pub sha256: &'a str,
}

pub fn canonical_image(name: &str) -> Result<ImageIdentity<'static>, String> {
    parse_manifest_identities(CANONICAL)?
        .into_iter()
        .find(|image| image.name == name)
        .ok_or_else(|| format!("missing canonical firmware image: {name}"))
}

fn string(value: &str) -> Result<&str, String> {
    let text = value
        .strip_prefix('"')
        .and_then(|v| v.strip_suffix('"'))
        .ok_or("expected a plain double-quoted string")?;
    if text
        .chars()
        .any(|c| c == '"' || c == '\\' || c.is_control())
    {
        return Err("unsupported string syntax in narrow firmware manifest".into());
    }
    Ok(text)
}

fn integer(value: &str) -> Result<usize, String> {
    let (digits, radix) = value.strip_prefix("0x").map_or((value, 10), |v| (v, 16));
    if digits.is_empty()
        || !digits.bytes().all(|b| match radix {
            16 => b.is_ascii_hexdigit(),
            _ => b.is_ascii_digit(),
        })
        || (radix == 10 && digits.len() > 1 && digits.starts_with('0'))
    {
        return Err("expected an unsigned manifest integer".into());
    }
    usize::from_str_radix(digits, radix).map_err(|e| e.to_string())
}

pub fn parse_manifest_identities(input: &str) -> Result<Vec<ImageIdentity<'_>>, String> {
    let mut images = Vec::new();
    let mut names = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut tables = BTreeSet::new();
    let mut section = "";
    let mut schema = false;
    let mut in_image = false;
    let mut name = None;
    let mut size = None;
    let mut sha256 = None;
    for raw in input.lines().chain(std::iter::once("[[images]]")) {
        let mut quoted = false;
        let mut end = raw.len();
        for (index, c) in raw.char_indices() {
            if c == '"' {
                quoted = !quoted;
            }
            if c == '#' && !quoted {
                end = index;
                break;
            }
        }
        let line = raw[..end].trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            match line {
                "[[images]]" => {
                    if in_image {
                        let image_name = name.take().ok_or("missing image name")?;
                        if !names.insert(image_name) {
                            return Err("duplicate image name".into());
                        }
                        images.push(ImageIdentity {
                            name: image_name,
                            size: size.take().ok_or("missing image size")?,
                            sha256: sha256.take().ok_or("missing image SHA-256")?,
                        });
                    }
                    in_image = true;
                    tables.clear();
                    section = "images";
                }
                "[images.executable_load]" | "[[images.regions]]" if in_image => {
                    if line == "[images.executable_load]" && !tables.insert(line) {
                        return Err("duplicate executable load table".into());
                    }
                    section = line;
                }
                _ => {
                    return Err("unsupported or misplaced table in narrow firmware manifest".into())
                }
            }
            keys.clear();
            continue;
        }
        let (key, value) = line.split_once('=').ok_or("expected manifest key/value")?;
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || !key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
            return Err("unsupported manifest key syntax".into());
        }
        if !keys.insert(key) {
            return Err(format!("duplicate manifest key: {key}"));
        }
        if matches!(key, "name" | "size" | "sha256") && section != "images" {
            return Err(format!("misplaced image identity field: {key}"));
        }
        if key == "schema_version" {
            if section != "" || value != "1" {
                return Err("unsupported manifest schema".into());
            }
            schema = true;
        } else if section == "images" && key == "name" {
            let text = string(value)?;
            if text.is_empty() || matches!(text, "." | "..") || text.contains('/') {
                return Err("invalid image name".into());
            }
            name = Some(text);
        } else if section == "images" && key == "size" {
            let count = integer(value)?;
            if count == 0 {
                return Err("image size must be positive".into());
            }
            size = Some(count);
        } else if section == "images" && key == "sha256" {
            let digest = string(value)?;
            if digest.len() != 64
                || !digest
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err("invalid image SHA-256".into());
            }
            sha256 = Some(digest);
        } else if value.starts_with('"') {
            string(value)?;
        } else if !matches!(value, "true" | "false") {
            integer(value)?;
        }
    }
    if !schema {
        return Err("missing manifest schema_version".into());
    }
    if images.is_empty() {
        return Err("firmware image table is empty".into());
    }
    Ok(images)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> String {
        format!(
            "schema_version = 1\n[[images]]\nname = \"venus.mbn\"\nsize = 12\nsha256 = \"{}\"\n",
            "a".repeat(64)
        )
    }

    #[test]
    fn canonical_identity_and_narrow_syntax() {
        let venus = canonical_image("venus.mbn").unwrap();
        assert_eq!(venus.size, 1159200);
        assert_eq!(
            venus.sha256,
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085"
        );
        assert!(canonical_image("missing").is_err());
        let text = manifest().replace("size = 12", "size = 0xc # comment");
        assert_eq!(parse_manifest_identities(&text).unwrap()[0].size, 12);
    }

    #[test]
    fn rejects_duplicate_names_and_keys_throughout_manifest() {
        let text = manifest();
        let duplicate = format!(
            "{text}[[images]]{}",
            text.split_once("[[images]]").unwrap().1
        );
        assert!(parse_manifest_identities(&duplicate).is_err());
        for suffix in [
            "name = \"other\"",
            "size = 13",
            "sha256 = \"bad\"",
            "extra = 1\nextra = 2",
            "[images.executable_load]\nfile_base = 0\nfile_base = 1",
            "[images.executable_load]\n[images.executable_load]",
        ] {
            assert!(
                parse_manifest_identities(&(text.clone() + suffix)).is_err(),
                "{suffix}"
            );
        }
        assert!(parse_manifest_identities(&text.replace(
            "schema_version = 1",
            "schema_version = 1\nschema_version = 1"
        ))
        .is_err());
    }

    #[test]
    fn rejects_missing_invalid_and_misplaced_identity() {
        let text = manifest();
        for (old, new) in [
            ("schema_version = 1", "schema_version = true"),
            ("schema_version = 1", "schema_version = 2"),
            ("schema_version = 1", ""),
            ("name = \"venus.mbn\"", ""),
            ("name = \"venus.mbn\"", "name = \"../venus.mbn\""),
            ("name = \"venus.mbn\"", "name = \"..\""),
            ("name = \"venus.mbn\"", "name = \"\""),
            ("size = 12", "size = 0"),
            ("size = 12", "size = -1"),
            ("size = 12", "size = true"),
            ("size = 12", "size = \"12\""),
            ("size = 12", "size = 012"),
            ("size = 12", "size = 999999999999999999999999999"),
            ("size = 12", ""),
            ("size = 12", "size.bytes = 12"),
        ] {
            assert!(
                parse_manifest_identities(&text.replace(old, new)).is_err(),
                "{new}"
            );
        }
        for digest in ["a".repeat(63), "A".repeat(64), "g".repeat(64)] {
            assert!(parse_manifest_identities(&text.replace(&"a".repeat(64), &digest)).is_err());
        }
        for header in [
            "[images.executable_load]",
            "[[images.regions]]",
            "[images.identity]",
            "[other]",
        ] {
            for key in ["name", "size", "sha256"] {
                let misplaced = format!("{text}{header}\n{key} = 1\n");
                assert!(
                    parse_manifest_identities(&misplaced).is_err(),
                    "{misplaced}"
                );
            }
        }
        for key in ["name", "size", "sha256"] {
            assert!(parse_manifest_identities(&format!("{key} = 1\n{text}")).is_err());
        }
        let sha_line = format!("sha256 = \"{}\"", "a".repeat(64));
        assert!(parse_manifest_identities(&text.replace(&sha_line, "")).is_err());
        for replacement in [
            "sha256 = 123",
            "sha256 = 'abc'",
            "sha256 = \"abc\" trailing",
        ] {
            assert!(parse_manifest_identities(&text.replace(&sha_line, replacement)).is_err());
        }
        assert!(parse_manifest_identities("schema_version = 1").is_err());
        assert!(parse_manifest_identities(&(text + "[[images]]\nname = \"broken\"\n")).is_err());
    }
}
