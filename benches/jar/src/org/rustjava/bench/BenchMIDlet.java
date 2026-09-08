package org.rustjava.bench;

import javax.microedition.lcdui.Display;
import javax.microedition.midlet.MIDlet;

public class BenchMIDlet extends MIDlet {
    private BenchCanvas canvas;
    private boolean started;

    public BenchMIDlet() {
        canvas = new BenchCanvas();
        started = false;
    }

    public void startApp() {
        if (started) {
            return;
        }
        started = true;
        try {
            Display.getDisplay(this).setCurrent(canvas);
        } catch (Throwable ignored) {
        }
        canvas.status = "running";
        canvas.repaint();
        runSuite();
        canvas.status = "done";
        canvas.repaint();
        String auto = System.getProperty("rustjava.bench.autoexit");
        if (auto != null && auto.length() > 0 && !"0".equals(auto)) {
            notifyDestroyed();
        }
    }

    public void pauseApp() {
    }

    public void destroyApp(boolean unconditional) {
    }

    private void runSuite() {
        int ms = readMs();
        int width = 240;
        int height = 320;
        try {
            width = canvas.getWidth();
            height = canvas.getHeight();
        } catch (Throwable ignored) {
        }
        Runner runner = new Runner(ms, width, height);
        runner.header();
        CpuTests.run(runner);
        canvas.lastScore = runner.lastLine;
        canvas.repaint();
        GfxTests.run(runner);
        canvas.lastScore = runner.lastLine;
        canvas.repaint();
        GameTests.run(runner);
        canvas.lastScore = runner.lastLine;
        canvas.repaint();
        runOptional(runner, "org.rustjava.bench.M3gTests", "m3g", "transform");
        runOptional(runner, "org.rustjava.bench.V3Tests", "v3", "affine");
        canvas.lastScore = runner.lastLine;
        runner.close();
    }

    private void runOptional(Runner runner, String className, String suite, String phase) {
        try {
            Class cls = Class.forName(className);
            BenchCase tests = (BenchCase) cls.newInstance();
            tests.run(runner);
        } catch (Throwable error) {
            runner.skip(suite, phase, error);
        }
    }

    private int readMs() {
        int ms = parseMs(System.getProperty("rustjava.bench.ms"), 0);
        if (ms <= 0) {
            ms = parseMs(getAppProperty("Bench-Ms"), 400);
        }
        if (ms < 50) {
            ms = 50;
        }
        if (ms > 10000) {
            ms = 10000;
        }
        return ms;
    }

    private static int parseMs(String text, int fallback) {
        if (text == null || text.length() == 0) {
            return fallback;
        }
        try {
            return Integer.parseInt(text);
        } catch (Throwable ignored) {
            return fallback;
        }
    }
}
