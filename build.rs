#[allow(unused)]
use {
    jlogger_tracing::{jdebug, jerror, jinfo, jwarn, JloggerBuilder, LevelFilter, LogTimeFormat},
    std::{
        env,
        fs::{self, canonicalize, create_dir_all, remove_dir_all, remove_file, File},
        os::unix::fs::symlink,
        path::{Path, PathBuf},
        process::{Command, Stdio},
    },
};

fn main() {
    JloggerBuilder::new()
        .max_level(LevelFilter::DEBUG)
        .log_file(Some(("/tmp/wamr-runner-rs.log", false)))
        .log_console(false)
        .log_time(LogTimeFormat::TimeNone)
        .build();

    let target = env::var("TARGET").unwrap();
    jinfo!(target = target);

    let current_dir_path = env::current_dir().unwrap();
    let current_dir_str = current_dir_path.as_path().to_str().unwrap();
    jinfo!(current_dir = current_dir_str);

    let out_dir = env::var("OUT_DIR").unwrap();
    let work_dir = format!("{}/wamr", out_dir);
    let wasm_dir = format!("{}/wasm-micro-runtime", current_dir_str);
    jinfo!("work_dir={}", work_dir);

    let wasm_log_file = "/tmp/wamr.log";
    let output = File::create(wasm_log_file).unwrap();

    let _ = remove_dir_all(&work_dir);
    Command::new("mkdir").arg(&work_dir).status().unwrap();
    jinfo!("Build wamr.");
    Command::new("cmake")
        .current_dir(&work_dir)
        .arg(&wasm_dir)
        .arg("-DWAMR_BUILD_PLATFORM=linux")
        .arg("-DWAMR_BUILD_AOT=1")
        .arg("-DWAMR_BUILD_JIT=0")
        .arg("-DWAMR_BUILD_LIBC_BUILTIN=1")
        .arg("-DWAMR_BUILD_FAST_INTERP=1")
        .arg("-DWAMR_BUILD_LIBC_WASI=1")
        .arg("-DWAMR_BUILD_LIB_WASI_THREADS=1")
        .stdout(Stdio::from(output.try_clone().unwrap()))
        .stderr(Stdio::from(output.try_clone().unwrap()))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    Command::new("make")
        .current_dir(&work_dir)
        .arg("-j10")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Wformat")
        .arg("-Wformat-security")
        .stdout(Stdio::from(output.try_clone().unwrap()))
        .stderr(Stdio::from(output.try_clone().unwrap()))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    let log = fs::read_to_string(wasm_log_file).unwrap();
    jinfo!("{}", log);
    let _ = remove_file(wasm_log_file);

    let wasm_export = format!("{wasm_dir}/core/iwasm/include/wasm_export.h");
    jinfo!(wasm_export = wasm_export);
    let binding = bindgen::Builder::default()
        .header(&wasm_export)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let wasm_export_rs = format!("{out_dir}/wasm_export.rs");
    binding
        .write_to_file(&wasm_export_rs)
        .expect("Failed to create binding file for wasm_export.h");
    jinfo!("Created binding: {}", wasm_export_rs);

    println!("cargo:rerun-if-changed=wamr-1.2.2");
    println!("cargo:rustc-link-search={}", work_dir);
    //println!("cargo:rustc-link-arg=-liwasm");
    println!("cargo:rustc-link-arg=-lvmlib");
    println!("cargo:rustc-link-arg=-lm");
    println!("cargo:rustc-link-arg=-lc");
}
