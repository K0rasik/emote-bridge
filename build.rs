use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap(); // Корень проекта

    let obj_file = format!("{}/avif_decoder.o", out_dir);
    let lib_file = format!("{}/libavif_decoder.a", out_dir);

    // Компилируем avif_decoder.c в объектный файл
    let status = Command::new("cc")
        .args(["-c", "avif_decoder.c", "-o", &obj_file])
        .status()
        .expect("Failed to compile avif_decoder.c");

    if !status.success() {
        panic!("cc command failed");
    }

    // Создаем статическую библиотеку
    let status = Command::new("ar")
        .args(["rcs", &lib_file, &obj_file])
        .status()
        .expect("Failed to create static library");

    if !status.success() {
        panic!("ar command failed");
    }

    // Указываем Cargo, где искать библиотеку
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=avif_decoder");
    println!("cargo:rustc-link-lib=dylib=avif"); // Подключаем динамическую libavif
}
