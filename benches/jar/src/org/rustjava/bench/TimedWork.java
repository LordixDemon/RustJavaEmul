package org.rustjava.bench;

public interface TimedWork {
    int run(int sink, int batch);

    void finish(Runner runner, long ops, int sink, long ms);
}
