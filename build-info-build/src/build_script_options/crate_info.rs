use pretty_assertions::assert_eq;

use build_info_common::semver::Version;
use build_info_common::CrateInfo;
use cargo_metadata::*;

use std::collections::HashMap;
use std::path::Path;

pub(crate) fn read_manifest() -> CrateInfo {
	let meta = MetadataCommand::new()
		.cargo_path(std::env::var_os("CARGO").unwrap())
		.manifest_path(Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml"))
		.exec()
		.unwrap();
	let root = make_crate_info(&meta);

	assert_eq!(root.name, std::env::var("CARGO_PKG_NAME").unwrap()); // sanity check...
	assert_eq!(root.version.to_string(), std::env::var("CARGO_PKG_VERSION").unwrap()); // sanity check...
	assert_eq!(root.authors.join(":"), std::env::var("CARGO_PKG_AUTHORS").unwrap()); // sanity check...

	root
}

fn make_crate_info(meta: &Metadata) -> CrateInfo {
	let resolve = meta.resolve.as_ref().unwrap();
	let root_id = resolve.root.as_ref().unwrap();
	let dependencies: HashMap<&PackageId, &Node> = resolve.nodes.iter().map(|node| (&node.id, node)).collect();

	to_crate_info(dependencies[&root_id], &dependencies, meta)
}

fn to_crate_info(node: &Node, dependencies: &HashMap<&PackageId, &Node>, meta: &Metadata) -> CrateInfo {
	let pkg = &meta[&node.id];
	let name = pkg.name.clone();
	let version = Version::parse(&pkg.version.to_string()).unwrap();
	let authors = pkg.authors.clone();
	let license = pkg.license.clone();
	let dependencies = node
		.deps
		.iter()
		.map(|dep| to_crate_info(dependencies[&dep.pkg], dependencies, meta))
		.collect();

	CrateInfo {
		name,
		version,
		authors,
		license,
		dependencies,
	}
}
