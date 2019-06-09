use deploy::cfg;

use serde::ser::Serialize;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Build and install testlib
    #[structopt(long = "enable-testlib")]
    testlib: bool,
    /// Build and install manual
    #[structopt(long = "enable-man")]
    man: bool,
    /// Generate tarball
    #[structopt(long = "enable-archive")]
    archive: bool,
    /// Cargo path
    #[structopt(long = "with-cargo")]
    cargo: Option<String>,
    /// CMake path
    #[structopt(long = "with-cmake")]
    cmake: Option<String>,
    /// Target triple
    #[structopt(long = "target", short = "T")]
    target: Option<String>,
    /// Optimization
    #[structopt(long = "optimize", short = "O")]
    optimize: bool,
    /// Debug symbols
    #[structopt(long = "dbg-dym", short = "D")]
    dbg_sym: bool,
    /// Prefix
    #[structopt(long = "prefix", short = "P")]
    install_prefix: Option<String>,
}

static MAKE_SCRIPT_TPL: &str = include_str!("../make-tpl.sh");
static MAKEFILE_TPL: &str = include_str!("../makefile.tpl");

fn generate_make_script(src: &str, build: &str) {
    let mut substitutions = HashMap::new();
    substitutions.insert("BUILD_DIR", build.to_string());
    substitutions.insert("SRC_DIR", src.to_string());
    let mut subst_text = String::new();
    for (k, v) in substitutions {
        let v_esc = shell_escape::escape(v.into());
        let line = format!("export JJS_{}=\"{}\"\n", k, &v_esc);
        subst_text.push_str(&line);
    }
    let script = MAKE_SCRIPT_TPL.replace("$SUBST$", &subst_text);
    let script_path = format!("{}/make", &build);
    std::fs::write(&script_path, script).unwrap();
    let full_script_path = std::fs::canonicalize(&script_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let makefile = MAKEFILE_TPL
        .replace("___SCRIPT_PATH___", &full_script_path)
        .replace("    ", "\t");
    let makefile_path = format!("{}/Makefile", &build);
    std::fs::write(&makefile_path, makefile).unwrap();
    //println!("To trigger build, run {}", &script_path);
}

fn check_build_dir(_src: &str, build: &str) {
    if deploy::util::create_or_empty(build).is_ok() {
        return;
    }
    let dot_build_file = format!("{}/.jjsbuild", build);
    if std::path::PathBuf::from(&dot_build_file).exists() {
        return;
    }
    eprintln!("maybe, assumed build dir ({}) contains some important files. If you are sure, add .jjsbuild in this dir", build);
    std::process::exit(1);
}

fn main() {
    let jjs_path = std::env::var("JJS_CFGR_SOURCE_DIR").unwrap();
    let build_dir_path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    check_build_dir(&jjs_path, &build_dir_path);
    //.unwrap();
    let opt: Opt = Opt::from_args();
    let tool_info = cfg::ToolInfo {
        cargo: opt
            .cargo
            .as_ref()
            .map(String::as_str)
            .unwrap_or_else(|| "cargo")
            .to_string(),
        cmake: opt
            .cmake
            .as_ref()
            .map(String::as_str)
            .unwrap_or_else(|| "cmake")
            .to_string(),
    };
    let profile = match (opt.optimize, opt.dbg_sym) {
        (true, false) => cfg::BuildProfile::Release,
        (true, true) => cfg::BuildProfile::RelWithDebInfo,
        _ => cfg::BuildProfile::Debug,
    };
    let build_config = cfg::Config {
        prefix: opt.install_prefix.clone(),
        target: match &opt.target {
            Some(t) => t.clone(),
            None => deploy::util::get_current_target(),
        },
        profile,
        man: opt.man,
        testlib: opt.testlib,
        tool_info,
        archive: opt.archive,
    };
    let manifest_path = format!("{}/jjs-build-config.json", &build_dir_path);
    println!("Configuration: {}", &build_config);
    println!("Emitting JJS build config: {}", &manifest_path);
    let out_file = std::fs::File::create(&manifest_path).unwrap();
    let mut ser = serde_json::Serializer::pretty(out_file);
    build_config.serialize(&mut ser).unwrap();
    generate_make_script(&jjs_path, &build_dir_path);
}