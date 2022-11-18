use anyhow::Result;
#[cfg(feature = "build-faiss")]
use cmake;

use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
#[cfg(feature = "build-tensorflow")]
use std::process::Command;
use std::{env, fs};

// fn cp_r(from: impl AsRef<Path>, to: impl AsRef<Path>) {
//   for e in from.as_ref().read_dir().unwrap() {
//       let e = e.unwrap();
//       let from = e.path();
//       let to = to.as_ref().join(e.file_name());
//       if e.file_type().unwrap().is_dir() {
//           fs::create_dir_all(&to).unwrap();
//           cp_r(&from, &to);
//       } else {
//           println!("{} => {}", from.display(), to.display());
//           fs::copy(&from, &to).unwrap();
//       }
//   }
// }
fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn toolchain_dir() -> PathBuf {
    let target = env::var("TARGET").unwrap();
    let toolchain_dir = manifest_dir().join("prebuilt").join(target);
    PathBuf::from(toolchain_dir)
}

// fn recursive_find()
// let library = std::fs::read_dir(&path)
//     .unwrap_or_else(|_| panic!("Cannot find dir: {}", &path.display()))
//     .filter_map(|de| Some(de.ok()?.path().join(format!("lib{}.dylib", name))))
//     .find(|p| {
//         println!("P: {}", p.display());
//         p.exists()
//     })
//     .unwrap_or_else(|| {
//         panic!(
//             "Cannot find file: {}",
//             path.join(format!("lib{}.dylib", name)).display()
//         )
//     });
fn set_w_permission(path: PathBuf) -> Result<()> {
    let mut perms = fs::metadata(&path)?.permissions();
    // perms.set_readonly(false);
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms)?;

    Ok(())
}

fn install_lib(path: PathBuf, name: &str) -> PathBuf {
    let libname = format!("lib{}.dylib", name);
    let src = path.join(&libname);
    let dst = toolchain_dir().join(&libname);

    println!("install_lib: {} - {}", src.display(), dst.display());

    if src.exists() == false {
        panic!("Cannot find: {}", src.display());
    }
    std::fs::copy(&src, &dst).unwrap();

    dst
}

#[cfg(feature = "build-faiss")]
fn build_faiss() {
    let faiss_src_dir = manifest_dir().join("src/faiss");

    println!("Building faiss");

    let build_config = &mut cmake::Config::new(faiss_src_dir);
    build_config.very_verbose(true);

    build_config.define("CMAKE_BUILD_TYPE", "Release");
    build_config.define("FAISS_ENABLE_C_API", "ON");
    build_config.define("FAISS_ENABLE_GPU", "OFF");
    build_config.define("BUILD_SHARED_LIBS", "ON");
    build_config.define("FAISS_ENABLE_PYTHON", "OFF");
    build_config.define("BUILD_TESTING", "OFF");

    let dst = build_config.build();

    install_lib(dst.join("build/faiss"), "faiss");
    install_lib(dst.join("build/c_api"), "faiss_c");
}

#[cfg(feature = "build-tensorflow")]
fn build_tensorflow() {
    let tensorflow_src_dir = manifest_dir().join("src/tensorflow");
    // let dst = out_dir().join("tf-build");
    Command::new("bazel")
        .current_dir(&tensorflow_src_dir)
        .args([
            // format!("--output_base={}", dst.display()).as_str(),
            "build",
            "//tensorflow/lite:libtensorflowlite.dylib",
        ])
        .spawn()
        .unwrap();
    let lib = install_lib(
        tensorflow_src_dir.join("bazel-bin/tensorflow/lite"),
        "tensorflowlite",
    );
    set_w_permission(lib).unwrap();

    // bazel --output_user_root=/tmp/bazel build x/y:z
}

fn main() {
    // Command::new("env").spawn().unwrap();
    println!("building jots toolchain");

    fs::create_dir_all(toolchain_dir()).unwrap();

    #[cfg(feature = "build-faiss")]
    build_faiss();

    #[cfg(feature = "build-tensorflow")]
    build_tensorflow();

    println!(
        "cargo:rustc-link-search=native={}",
        toolchain_dir().display()
    );
}
