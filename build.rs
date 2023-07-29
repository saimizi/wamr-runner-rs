//cspell:word canonicalize rnetmon aarch rustc

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
    let wasm_dir = format!("{}/wamr-1.2.2", current_dir_str);
    jinfo!("work_dir={}", work_dir);

    let wasm_log_file = "/tmp/wamr.log";
    let output = File::create(wasm_log_file).unwrap();

    let _ = remove_dir_all(&work_dir);
    Command::new("mkdir").arg(&work_dir).status().unwrap();
    jinfo!("Build wamr.");
    Command::new("cmake")
        .current_dir(&work_dir)
        .arg(&wasm_dir)
        .arg("-DWAMR_BUILD_INTERP=1")
        .arg("-DWAMR_BUILD_AOT=1")
        .arg("-DWAMR_BUILD_LIBC_WASI=1")
        .arg("-DWAMR_BUILD_DUMP_CALL_STACK=1")
        .arg("-DWAMR_BUILD_PLATFORM=linux")
        .stdout(Stdio::from(output.try_clone().unwrap()))
        .stderr(Stdio::from(output.try_clone().unwrap()))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    Command::new("make")
        .current_dir(&work_dir)
        .arg("-j10")
        .stdout(Stdio::from(output.try_clone().unwrap()))
        .stderr(Stdio::from(output.try_clone().unwrap()))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    let log = fs::read_to_string(wasm_log_file).unwrap();
    jinfo!("{}", log);
    let _ = remove_file(wasm_log_file);

    jinfo!(wasm_dir = work_dir);

    println!("cargo:rerun-if-changed=wamr-1.2.2");
    println!("cargo:rustc-link-search={}", work_dir);
    println!("cargo:rustc-link-arg=-liwasm");
    println!("cargo:rustc-link-arg=-lm");
}
