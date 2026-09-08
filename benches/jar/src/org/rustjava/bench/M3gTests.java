package org.rustjava.bench;

import javax.microedition.m3g.Transform;

public class M3gTests implements BenchCase {
    public void run(Runner runner) {
        runner.runTimed(256, new TimedWork() {
            public int run(int sink, int batch) {
                Transform a = new Transform();
                Transform b = new Transform();
                float[] out = new float[16];
                float[] point = new float[] {1, 2, 3, 1};
                int s = sink;
                for (int i = 0; i < batch; i++) {
                    a.setIdentity();
                    a.postRotate(i, 0.1f, 0.8f, 0.3f);
                    a.postTranslate(i * 0.01f, 1, 2);
                    a.postScale(1.05f, 0.95f, 1.1f);
                    b.setIdentity();
                    b.postRotate(i * 0.5f, 0, 1, 0);
                    a.postMultiply(b);
                    a.transform(point);
                    a.get(out);
                    s ^= (int) (out[0] * 1000) ^ (int) (point[0] * 1000);
                    a.invert();
                }
                return s;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("m3g", "transform", ops, "ops", sink, ms);
            }
        });
    }
}
