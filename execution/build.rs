fn main() {
    // CPU feature detection at compile time
    println!("cargo:rerun-if-changed=build.rs");
    
    // Detect target features
    let target = std::env::var("TARGET").unwrap();
    
    if target.contains("x86_64") {
        // Enable optimizations for x86_64
        println!("cargo:rustc-cfg=has_x86_64");
        
        // Check for AVX2 support (most modern CPUs)
        if cfg!(target_feature = "avx2") {
            println!("cargo:rustc-cfg=has_avx2");
        }
        
        // Check for SSE4.2 support
        if cfg!(target_feature = "sse4.2") {
            println!("cargo:rustc-cfg=has_sse42");
        }
        
        // Check for FMA support
        if cfg!(target_feature = "fma") {
            println!("cargo:rustc-cfg=has_fma");
        }
    }
    
    // Set global allocator
    println!("cargo:rustc-env=MALLOC_CONF=background_thread:true,metadata_thp:auto,dirty_decay_ms:30000,muzzy_decay_ms:30000");
}
