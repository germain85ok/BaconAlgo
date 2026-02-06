// BaconAlgo 2040 Quantum Edition - Build Script
// Détecte les features CPU et configure les optimisations

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Détection de l'architecture cible
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    println!("cargo:warning=🚀 Building BaconAlgo 2040 Quantum Edition");
    println!("cargo:warning=   Architecture: {}", target_arch);
    println!("cargo:warning=   OS: {}", target_os);
    
    // Configuration CPU-spécifique pour x86_64
    if target_arch == "x86_64" {
        // Détection des features CPU disponibles
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                println!("cargo:warning=   ✅ AVX2 disponible");
                println!("cargo:rustc-cfg=feature=\"avx2\"");
            }
            if is_x86_feature_detected!("avx") {
                println!("cargo:warning=   ✅ AVX disponible");
                println!("cargo:rustc-cfg=feature=\"avx\"");
            }
            if is_x86_feature_detected!("sse4.2") {
                println!("cargo:warning=   ✅ SSE4.2 disponible");
                println!("cargo:rustc-cfg=feature=\"sse42\"");
            }
            if is_x86_feature_detected!("fma") {
                println!("cargo:warning=   ✅ FMA disponible");
                println!("cargo:rustc-cfg=feature=\"fma\"");
            }
        }
    }
    
    // Configuration pour ARM/AArch64
    if target_arch == "aarch64" {
        println!("cargo:warning=   📱 ARM64/AArch64 detected - NEON enabled");
        println!("cargo:rustc-cfg=feature=\"neon\"");
    }
    
    // Recommandations de compilation
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile == "release" {
        println!("cargo:warning=");
        println!("cargo:warning=⚡ OPTIMISATIONS ACTIVÉES:");
        println!("cargo:warning=   • LTO: fat");
        println!("cargo:warning=   • opt-level: 3");
        println!("cargo:warning=   • codegen-units: 1");
        println!("cargo:warning=   • target-cpu: native");
        println!("cargo:warning=");
        println!("cargo:warning=🎯 PERFORMANCE CIBLE:");
        println!("cargo:warning=   • Latence: < 10μs");
        println!("cargo:warning=   • Scan: 10K instruments < 100ms");
        println!("cargo:warning=   • Throughput: 1M+ msg/s");
        println!("cargo:warning=");
    }
}
