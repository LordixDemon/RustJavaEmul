package org.rustjava.bench;

import java.util.Hashtable;
import java.util.Vector;

public class CpuTests {
    public static void run(Runner runner) {
        intAdd(runner);
        longMul(runner);
        floatMath(runner);
        objectAlloc(runner);
        stringOps(runner);
        hashtable(runner);
        vector(runner);
        arrays(runner);
    }

    private static void intAdd(final Runner runner) {
        runner.runTimed(4096, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    a += i;
                    a ^= i * 3;
                    a = (a << 1) | (a >>> 31);
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("cpu", "int_add", ops, "ops", sink, ms);
            }
        });
    }

    private static void longMul(final Runner runner) {
        runner.runTimed(2048, new TimedWork() {
            private long last;

            public int run(int sink, int batch) {
                long a = sink + last;
                for (int i = 0; i < batch; i++) {
                    a += i;
                    a *= 6364136223846793005L;
                    a ^= a >>> 17;
                }
                last = a;
                return (int) (a ^ (a >>> 32));
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("cpu", "long_mul", ops, "ops", sink, ms);
            }
        });
    }

    private static void floatMath(final Runner runner) {
        runner.runTimed(256, new TimedWork() {
            public int run(int sink, int batch) {
                double a = sink * 0.0001;
                for (int i = 0; i < batch; i++) {
                    double x = a + i * 0.017;
                    a += Math.sin(x) + Math.cos(x) + Math.sqrt(x * x + 1.0);
                }
                return (int) (a * 1000.0);
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("cpu", "float_math", ops, "ops", sink, ms);
            }
        });
    }

    private static void objectAlloc(final Runner runner) {
        runner.runTimed(512, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    Integer boxed = new Integer(a + i);
                    a ^= boxed.intValue();
                    a ^= boxed.hashCode();
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("cpu", "object_alloc", ops, "ops", sink, ms);
            }
        });
    }

    private static void stringOps(final Runner runner) {
        runner.runTimed(128, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                String base = "RustJavaBench-";
                for (int i = 0; i < batch; i++) {
                    StringBuffer built = new StringBuffer(base);
                    built.append(a + i);
                    String s = built.toString();
                    a ^= s.hashCode();
                    a ^= s.indexOf("Bench");
                    a ^= s.length();
                    String upper = s.toUpperCase();
                    a ^= upper.charAt(0);
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("cpu", "string", ops, "ops", sink, ms);
            }
        });
    }

    private static void hashtable(final Runner runner) {
        runner.runTimed(64, new TimedWork() {
            public int run(int sink, int batch) {
                Hashtable table = new Hashtable();
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    String key = Integer.toString(i);
                    table.put(key, Integer.toString(a + i));
                }
                for (int i = 0; i < batch; i++) {
                    Object value = table.get(Integer.toString(i));
                    if (value != null) {
                        a ^= value.hashCode();
                    }
                }
                return a ^ table.size();
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("cpu", "hashtable", ops, "ops", sink, ms);
            }
        });
    }

    private static void vector(final Runner runner) {
        runner.runTimed(128, new TimedWork() {
            public int run(int sink, int batch) {
                Vector vector = new Vector();
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    vector.addElement(Integer.toString(a + i));
                }
                for (int i = 0; i < batch; i++) {
                    Object value = vector.elementAt(i);
                    a ^= value.hashCode();
                }
                return a ^ vector.size();
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("cpu", "vector", ops, "ops", sink, ms);
            }
        });
    }

    private static void arrays(final Runner runner) {
        runner.runTimed(4, new TimedWork() {
            public int run(int sink, int batch) {
                int n = 256 * batch;
                int[] src = new int[n];
                int[] dst = new int[n];
                int a = sink;
                for (int i = 0; i < n; i++) {
                    src[i] = a + i;
                }
                System.arraycopy(src, 0, dst, 0, n);
                for (int i = 0; i < n; i += 16) {
                    a ^= dst[i];
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("cpu", "arraycopy", ops * 256L, "elements", sink, ms);
            }
        });
    }
}
