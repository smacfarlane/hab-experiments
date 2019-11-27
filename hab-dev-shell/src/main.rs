use std::collections::HashMap;
use std::vec::Vec;

use habitat_core::package::{metadata::MetaFile, PackageIdent, PackageInstall};

use clap::Clap;
use itertools::Itertools;

// fn resolve_dependencies(declared: Vec<PackageIdent>) -> Vec<PackageInstall> {
// let mut ui = UI::default_with_env();
// let mut resolved: Vec<PackageInstall> = Vec::new();
// let fs_root_path = Path::new("/tmp/unified-build-workspace");
//
// for ident in &declared {
// let installed = install::start(
// &mut ui,
// &default_bldr_url(),
// &ChannelIdent::from(
// habitat_core::env::var("HAB_BLDR_CHANNEL").unwrap_or("stable".to_string()),
// ),
// &(ident.clone(), PackageTarget::active_target()).into(),
// "unified-pkg-build",
// "0.0.1",
// fs_root_path,
// &habitat_core::fs::cache_artifact_path(None::<String>),
// None,
// &install::InstallMode::default(),
// &install::LocalPackageUsage::default(),
// install::InstallHookMode::Ignore,
// );
//
// if let Ok(pkg) = installed {
// let installed_package = PackageInstall::load(pkg.ident(), Some(&fs_root_path)).unwrap();
// resolved.push(installed_package);
// }
// }
// let mut resolved_deps: Vec<PackageInstall> = Vec::new();
// for installed_package in &resolved {
// let mut installed_package_tdeps = installed_package
// .tdeps()
// .and_then(|tdeps| {
// tdeps
// .iter()
// .map(|ident| PackageInstall::load(ident, Some(&fs_root_path)))
// .collect()
// })
// .unwrap();
// resolved_deps.append(&mut installed_package_tdeps);
// }
// resolved.append(&mut resolved_deps);
// resolved
// }

fn resolve_environment(packages: Vec<PackageIdent>) -> HashMap<String, String> {
    // Path is a special case
    let mut env: HashMap<String, String> = HashMap::new();
    let env_vars = vec![
        ("CFLAGS", MetaFile::CFlags, " "),
        ("CPPFLAGS", MetaFile::CppFlags, " "),
        ("CXXFLAGS", MetaFile::CxxFlags, " "),
        ("LD_RUN_PATH", MetaFile::LdRunPath, ":"),
        ("PKG_CONFIG_PATH", MetaFile::PkgConfigPath, ":"),
        ("LDFLAGS", MetaFile::LdFlags, " "),
    ];

    for pkg in packages.iter().map(|p| PackageInstall::load(p, None)) {
        match pkg {
            Ok(pkg) => {
                for (var, metafile, seperator) in &env_vars {
                    match pkg.read_metafile(*metafile) {
                        Ok(contents) => {
                            env.entry(var.to_string())
                                .and_modify(|v| v.push_str(&format!("{}{}", seperator, &contents)))
                                .or_insert(contents);
                        }
                        Err(habitat_core::Error::MetaFileNotFound(_metafile)) => {}
                        Err(_) => unimplemented!(),
                    }
                }
            }
            Err(e) => {
                println!("Unable to load package: {:?}", e);
            }
        }
    }

    env.insert("PATH".to_string(), resolve_build_path(&packages));

    env
}

fn resolve_build_path(packages: &Vec<PackageIdent>) -> String {
    let mut build_path = Vec::new();
    let mut tdep_paths = Vec::new();
    for pkg in packages
        .iter()
        .map(|p| PackageInstall::load(p, None).expect("Unable to load package"))
    {
        if let Ok(mut path) = pkg.paths() {
            build_path.append(&mut path);
        }

        for tdep in pkg
            .tdeps()
            .unwrap()
            .iter()
            .map(|tdep| PackageInstall::load(tdep, None).expect("Unable to load package"))
        {
            if let Ok(mut path) = tdep.paths() {
                tdep_paths.append(&mut path);
            }
        }
    }

    build_path.append(&mut tdep_paths);

    let build_path: Vec<String> = build_path
        .into_iter()
        .unique()
        .map(|path| path.to_str().unwrap().to_string())
        .collect();

    build_path.join(":")
}

#[derive(Clap, Debug)]
struct Opt {
    #[clap(name = "PACKAGES")]
    packages: Vec<PackageIdent>,
}

fn main() {
    let opts = Opt::parse();

    let mut build_environment = resolve_environment(opts.packages);

    let mut cmd = std::process::Command::new("bash");

    if let Ok(current_path) = std::env::var("PATH") {
        build_environment
            .entry("PATH".to_string())
            .and_modify(|path| path.push_str(&format!(":{}", current_path)))
            .or_insert(current_path);
    }
    dbg!(&build_environment);
    cmd.envs(build_environment);

    let mut child = cmd.spawn().expect("Unable to start shell");
    child.wait().expect("failed to wait on child");
}
