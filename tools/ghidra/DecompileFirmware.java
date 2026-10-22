// SPDX-License-Identifier: GPL-3.0-only
// Script for Ghidra headless analysis of locally supplied ELF images.
// Generated outputs are binary-derived analysis data; NOT GPL-covered Rust code.
import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionIterator;
import java.io.BufferedWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashSet;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class DecompileFirmware extends GhidraScript {
    @Override
    protected void run() throws Exception {
        String[] a = getScriptArgs();
        if (a.length < 1) throw new IllegalArgumentException("Output path is required");
        Path out = Path.of(a[0]);
        int maxFns = a.length > 1 ? Integer.parseInt(a[1]) : 256;
        int runSecs = a.length > 2 ? Integer.parseInt(a[2]) : 40;
        long limit = System.nanoTime() + (long)runSecs * 1000000000L;
        Set<String> previouslyAttempted = new HashSet<>();
        if (a.length > 3 && Files.exists(Path.of(a[3]))) {
            Pattern marker = Pattern.compile("^// ===== .* @ ([0-9a-fA-F]+) =====$");
            try (java.util.stream.Stream<String> lines = Files.lines(Path.of(a[3]), StandardCharsets.UTF_8)) {
                lines.forEach(line -> {
                    Matcher m = marker.matcher(line);
                    if (m.matches()) previouslyAttempted.add(m.group(1).toLowerCase());
                });
            }
        }
        DecompInterface d = new DecompInterface();
        d.openProgram(currentProgram);
        long all = currentProgram.getFunctionManager().getFunctionCount();
        int ok = 0, error = 0, attempted = 0, skipped = 0;
        try (BufferedWriter w = Files.newBufferedWriter(out, StandardCharsets.UTF_8)) {
            w.write("// Source: " + currentProgram.getName() + "\n");
            w.write("// ISA: " + currentProgram.getLanguage().getLanguageID() + "\n");
            w.write("// Ghidra discovered function count: " + all + "\n");
            w.write("// Decompiler output is an *approximation*, not original source.\n");
            FunctionIterator funcs = currentProgram.getFunctionManager().getFunctions(true);
            while (funcs.hasNext() && !monitor.isCancelled() && attempted < maxFns && System.nanoTime() < limit) {
                Function fn = funcs.next();
                if (previouslyAttempted.contains(fn.getEntryPoint().toString().toLowerCase())) {
                    skipped++;
                    continue;
                }
                attempted++;
                w.write("\n// ===== " + fn.getName() + " @ " + fn.getEntryPoint() + " =====\n");
                try {
                    DecompileResults result = d.decompileFunction(fn, 3, monitor);
                    if (result.decompileCompleted() && result.getDecompiledFunction() != null) {
                        String ps = result.getDecompiledFunction().getC();
                        if (ps.length() > 24000) ps = ps.substring(0, 24000) + "\n// TRUNCATED\n";
                        w.write(ps + "\n");
                        ok++;
                    } else {
                        w.write("// ERROR: " + result.getErrorMessage() + "\n"); error++;
                    }
                } catch (Exception ex) {
                    w.write("// DECOMPILE FAILURE " + ex + "\n"); error++;
                }
                if (attempted % 20 == 0) w.flush();
            }
            w.write("\n// SUMMARY discovered=" + all + " attempted=" + attempted + " decompiled=" + ok + " errors=" + error + " skipped=" + skipped + " capped=" + (attempted + skipped < all) + "\n");
        } finally {d.dispose();}
        println("DECOMP_SUMMARY " + currentProgram.getName() + " discovered=" + all + " attempted=" + attempted + " success=" + ok + " failed=" + error + " skipped=" + skipped);
    }
}
