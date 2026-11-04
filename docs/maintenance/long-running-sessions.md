# Reproducible long-running firmware sessions

The user-visible chat is not a source repository or an artifact store. Preserve
all useful work as small linear commits and **verified Git bundles**. A frontend
freeze or a reset execution container must not erase the only working copy.

## What the previous conversation export can establish

The Oct 31 conversation export contained 4,147 messages, including 1,807 tool
messages, in a 16.7 MB JSON file. It does **not** include browser memory,
render timings, transport errors, or the backend lifecycle. Large conversations,
large emitted tool results, or execution-session changes are plausible causes
of observed freezes, but none is proven by the export alone.

## File and output limits

- Read large inputs *inside the container* using streaming parsers and bounded
  chunks. Do not paste multi-megabyte source, firmware, logs, or conversation
  exports into an assistant/tool response.
- Summarize counts and hash results on disk. Limit displayed diagnostics with
  explicit line ranges or `tail -n 10` and short excerpts.
- Use single-job offline Rust compilation under the 4 GiB limit:
  `./tools/low-memory-build.sh test` and `./tools/low-memory-build.sh release`.
  Avoid concurrent compiler jobs and keep their full logs on disk.
- Confirm reference image size and pinned SHA-256 before comparisons; compare
  generated spans by streaming over their exact source offsets.
- Publish complete generated images only after the pinned hashes match, using
  atomic replacement in the target directory.

## Recovery steps

1. Verify that every uploaded file is readable. Stop immediately if the client
   reports inaccessible files, rather than claiming missing inputs were used.
2. Reassemble split archives in their specified order; validate compressed
   streams and extract into separate directories.
3. Import the latest **available** verified checkpoint bundle. Do not assume
   that earlier generated bundle links are available in a new container.
4. Check out its recorded branch/HEAD, check `git status`, run the local
   quality gate, and continue in short independent commits.
5. Regularly create `git bundle create CHECKPOINT.bundle --all`, verify by
   cloning into a fresh directory, compare HEAD hashes, and run `git fsck`.
6. Include the exact verified checkpoint link and HEAD in the visible response.
   A new chat with that bundle is safer than relying on an enormous prior chat.

Source coverage includes both modeled metadata and executable spans. A match
that depends on copying external OEM payload bytes or signatures is **not**
independent decompilation or a verified hardware boot.
