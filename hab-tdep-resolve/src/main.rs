use std::collections::BTreeSet;

use clap::Clap;
use lazy_static::lazy_static;

use habitat_api_client::{self, error::Result};
use habitat_core::{
    package::{Identifiable, PackageIdent, PackageTarget},
    ChannelIdent,
};

#[derive(Clap, Debug)]
struct Opt {
    #[clap(name = "DEPS")]
    deps: Vec<PackageIdent>,
}
lazy_static! {
    static ref HAB_BLDR_CHANNEL: ChannelIdent = ChannelIdent::stable();
}

const BLDR_URL: &str = "https://bldr.acceptance.habitat.sh";

enum Dependency {
    Dep,
    BuildDep,
    TDep,
    BuildTDep,
}

fn main() -> Result<()> {
    let target = PackageTarget::active_target();
    let opts = Opt::parse();
    // TODO: This probably belongs in a graph
    let mut resolved: BTreeSet<PackageIdent> = BTreeSet::new();

    let api_client = habitat_api_client::Client::new(BLDR_URL, "hab", "0.0.0", None)?;

    for ident in opts.deps {
        println!("Fetching expanded dependencies for {}", ident);
        let resolved_package: Option<habitat_api_client::Package> =
            match api_client.show_package_metadata((&ident, target), &HAB_BLDR_CHANNEL, None) {
                Ok(fqpi) => dbg!(Some(fqpi)),
                _ => None,
            };

        if let Some(resolved_package) = resolved_package {
            resolved.insert(resolved_package.ident.clone());
            for package in resolved_package.tdeps {
                resolved.insert(package);
            }
        }
        //TODO: I think we're missing a trait on Package to allow us to use
        // either Package and PackageInstall
        //TODO: Handle locally installed case later
        // match PackageInstall::load(ident, None) { */
        // Ok(installed_package) => {
        //   if let Some(builder_package) {
        //
        //   } else {
        //     installed_package
        //   }
        //
        // }
        // Err(_) => {
        //
        // }
        // }

        // TODO: Check installed packages unless IGNORE_LOCAL is set
    }

    dbg!(resolved);

    Ok(())
}
