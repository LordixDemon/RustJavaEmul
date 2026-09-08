use std::{
    path::Path,
    time::{Duration, Instant},
};

use rust_java::{StartType, create_jvm_with_screen, invoke_entrypoint};

#[tokio::test]
async fn test_profile_pop2008() {
    let jar = Path::new("apk/spaces-java/all-games/41720016-Prince_of_Persia_2008_RU.jar");
    if !jar.exists() {
        println!("JAR not found, skipping");
        return;
    }

    println!("=== PROFILING PRINCE OF PERSIA 2008 ===");
    let t0 = Instant::now();

    jvm_rust::set_profile_enabled(true);
    jvm_rust::reset_profile();

    let (jvm, runtime) = create_jvm_with_screen(std::io::sink(), &StartType::Jar(jar), &[Path::new(".")], None)
        .await
        .expect("create_jvm failed");

    let t_jvm = Instant::now();
    println!("1. JVM & Classloader initialized in {:.2} ms", (t_jvm - t0).as_secs_f64() * 1000.0);

    let midlet = invoke_entrypoint(&jvm, &StartType::Jar(jar), &[] as &[&str])
        .await
        .expect("invoke_entrypoint failed");

    let t_entry = Instant::now();
    println!("2. Entrypoint (startApp) executed in {:.2} ms", (t_entry - t_jvm).as_secs_f64() * 1000.0);

    // Let it run for 10 seconds while measuring
    println!("3. Running game thread for 10 seconds...");
    for sec in 1..=10 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let snap = jvm_rust::profile_snapshot();
        println!(
            "[{:2}s] opcodes={:<10} fast={:<10} slow={:<8} runs={:<6} jumps={:<8} ret={:<6}",
            sec, snap.opcodes, snap.fast_opcodes, snap.slow_opcodes, snap.interpreter_runs, snap.jumps, snap.returns
        );
    }

    let top_methods = jvm_rust::method_opcode_report_and_reset(25);
    println!("\n=== TOP 25 HOTTEST METHODS ===");
    for method in top_methods.split_whitespace() {
        println!("  {method}");
    }

    let snap = jvm_rust::profile_snapshot();
    println!("\n=== PROFILE SUMMARY ===");
    println!("Total opcodes:         {}", snap.opcodes);
    println!(
        "Fast opcodes:          {} ({:.1}%)",
        snap.fast_opcodes,
        (snap.fast_opcodes as f64 / snap.opcodes.max(1) as f64) * 100.0
    );
    println!(
        "Slow opcodes:          {} ({:.1}%)",
        snap.slow_opcodes,
        (snap.slow_opcodes as f64 / snap.opcodes.max(1) as f64) * 100.0
    );
    println!("Interpreter runs:      {}", snap.interpreter_runs);
    println!("Invoke virtual:        {}", snap.invoke_virtual);
    println!("Invoke static:         {}", snap.invoke_static);
    println!("Invoke special:        {}", snap.invoke_special);
    println!("Invoke interface:      {}", snap.invoke_interface);
    println!("Array load (one):      {}", snap.array_load_one);
    println!("Array store (one):     {}", snap.array_store_one);
    println!("Array load (bulk):     {}", snap.array_load_bulk);
    println!("Array store (bulk):    {}", snap.array_store_bulk);
    println!("Array raw read:        {}", snap.array_raw_read);
    println!("Array raw write:       {}", snap.array_raw_write);

    runtime.abort_spawned();
    let _ = midlet;
}
