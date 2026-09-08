package org.rustjava.bench;

import java.io.FileOutputStream;
import java.io.PrintStream;

public class Runner {
    public int durationMs;
    public int screenW;
    public int screenH;
    public String lastLine;
    private final StringBuffer log;
    private PrintStream fileOut;

    public Runner(int durationMs, int screenW, int screenH) {
        this.durationMs = durationMs;
        this.screenW = screenW;
        this.screenH = screenH;
        this.log = new StringBuffer();
    }

    public int mix(int state, int value) {
        return state ^ (value * 1664525 + 1013904223);
    }

    public void header() {
        StringBuffer row = new StringBuffer("RUSTJAVA_BENCH\tmeta\tversion=1\tscreen=");
        row.append(screenW);
        row.append('x');
        row.append(screenH);
        row.append("\tms=");
        row.append(durationMs);
        line(row.toString());
    }

    public void record(String suite, String phase, long ops, String unit, int checksum, long ms) {
        long opsPerSec = 0;
        long msPerOpX10000 = 0;
        if (ms > 0) {
            opsPerSec = (ops * 1000L) / ms;
        }
        if (ops > 0) {
            msPerOpX10000 = (ms * 10000L) / ops;
        }
        StringBuffer row = new StringBuffer("RUSTJAVA_BENCH\t");
        row.append(suite);
        row.append('\t');
        row.append(phase);
        row.append('\t');
        row.append(ops);
        row.append('\t');
        row.append(unit);
        row.append('\t');
        row.append(hex8(checksum));
        row.append('\t');
        row.append(ms);
        row.append('\t');
        row.append(msPerOpX10000 / 10000);
        row.append('.');
        row.append(pad4((int) (msPerOpX10000 % 10000)));
        row.append('\t');
        row.append(opsPerSec);
        line(row.toString());
    }

    public void skip(String suite, String phase, Throwable error) {
        String why = "skip";
        if (error != null && error.getMessage() != null) {
            why = error.getClass().getName();
        } else if (error != null) {
            why = error.getClass().getName();
        }
        record(suite, phase, 0, why, 0, 0);
    }

    public long runTimed(int batch, TimedWork work) {
        int sink = 1;
        long ops = 0;
        long start = System.currentTimeMillis();
        long end = start + durationMs;
        while (System.currentTimeMillis() < end) {
            sink = work.run(sink, batch);
            ops += batch;
        }
        long ms = System.currentTimeMillis() - start;
        if (ms < 1) {
            ms = 1;
        }
        work.finish(this, ops, sink, ms);
        return ops;
    }

    public String text() {
        return log.toString();
    }

    public void close() {
        if (fileOut != null) {
            fileOut.flush();
            fileOut.close();
            fileOut = null;
        }
    }

    private void line(String row) {
        lastLine = row;
        log.append(row);
        log.append('\n');
        System.out.println(row);
        System.out.flush();
        if (fileOut == null) {
            try {
                String path = System.getProperty("rustjava.bench.out");
                if (path != null && path.length() > 0) {
                    fileOut = new PrintStream(new FileOutputStream(path));
                }
            } catch (Throwable ignored) {
            }
        }
        if (fileOut != null) {
            fileOut.println(row);
            fileOut.flush();
        }
    }

    private static String hex8(int value) {
        char[] digits = "0123456789abcdef".toCharArray();
        char[] out = new char[16];
        for (int i = 0; i < 8; i++) {
            out[i] = '0';
        }
        int v = value;
        for (int i = 15; i >= 8; i--) {
            out[i] = digits[v & 15];
            v >>>= 4;
        }
        return new String(out);
    }

    private static String pad4(int value) {
        char[] out = new char[] {'0', '0', '0', '0'};
        int v = value;
        for (int i = 3; i >= 0; i--) {
            out[i] = (char) ('0' + (v % 10));
            v /= 10;
        }
        return new String(out);
    }
}
