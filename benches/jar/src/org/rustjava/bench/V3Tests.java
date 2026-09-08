package org.rustjava.bench;

import com.mascotcapsule.micro3d.v3.AffineTrans;

public class V3Tests implements BenchCase {
    public void run(Runner runner) {
        runner.runTimed(256, new TimedWork() {
            public int run(int sink, int batch) {
                AffineTrans a = new AffineTrans();
                AffineTrans b = new AffineTrans();
                int s = sink;
                for (int i = 0; i < batch; i++) {
                    a.setIdentity();
                    a.rotationX(i * 17);
                    b.setIdentity();
                    b.rotationY(i * 13);
                    a.mul(b);
                    b.rotationZ(i * 11);
                    a.multiply(b);
                    s ^= i;
                }
                return s ^ a.hashCode();
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("v3", "affine", ops, "ops", sink, ms);
            }
        });
    }
}
