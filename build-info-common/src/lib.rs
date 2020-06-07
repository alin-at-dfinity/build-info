/*!
Common types used by the `build-info` and `build-info-build` crates.

User code should not depend on this crate directly, but rather depend on `build-info` (as a `[dependency]`) and `build-info-build` (as a `[build-dependency]`).
The types provided herein are reexported by `build-info` and should be used that way.
For example, `build_info_common::BuildInfo` should be used as `build_info::BuildInfo` instead.
*/

#![forbid(unsafe_code)]

use derive_more::Display;

pub use chrono::{DateTime, TimeZone, Utc};
pub use semver::{Identifier, Version};

#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "enable-serde")]
mod versioned_string;
#[cfg(feature = "enable-serde")]
pub use versioned_string::VersionedString;

pub mod display;

/// Gets the version of the `build-info-common` crate (this crate)
pub fn crate_version() -> Version {
	Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

/// Information about the current build
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BuildInfo {
	/// Updated whenever `build.rs` is rerun.
	pub timestamp: DateTime<Utc>,

	/// Cargo currently supports two different build types: `"Release"` and `"Debug"`
	pub profile: String,

	/// Information about the current crate
	pub crate_info: CrateInfo,

	/// Information about the compiler used.
	pub compiler: CompilerInfo,

	/// `Some` if the project is inside a check-out of a supported version control system.
	pub version_control: Option<VersionControl>,
}

/// Convert a UNIX timestamp in nanoseconds to DateTime<Utc>
pub fn nanos_to_utc(nanos: i64) -> DateTime<Utc> {
	Utc.timestamp_nanos(nanos)
}

/// Convert a UNIX timestamp in seconds to DateTime<Utc>
pub fn epoch_to_utc(secs: i64) -> DateTime<Utc> {
	Utc.timestamp(secs, 0)
}

/// Information about the current crate (i.e., the crate for which build information has been generated)
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CrateInfo {
	/// The name, as defined in `Cargo.toml`.
	pub name: String,

	/// The version, as defined in `Cargo.toml`.
	pub version: Version,

	/// The authors, as defined in `Cargo.toml`.
	pub authors: Vec<String>,
}

/// `rustc` version and configuration
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompilerInfo {
	/// Version of the current `rustc`
	pub version: Version,

	/// Commit hash from which `rustc` was built
	pub commit_id: Option<String>,

	/// Date on which `rustc` was built
	pub commit_date: Option<String>,

	/// Channel which was configured for this version of `rustc`
	pub channel: CompilerChannel,

	/// Identifies the host on which `rustc` was running
	pub host_triple: String,

	/// Identifies the target architecture for which the crate is being compiled
	pub target_triple: String,
}

/// `rustc` distribution channel (some compiler features are only available on specific channels)
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Display, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}

/// Support for different version control systems
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionControl {
	Git(GitInfo),
}

/**
Information about a git repository

If a git repository is detected (and, thereby, this information included), the build script will be rerun whenever the
currently checked out commit changes.
*/
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitInfo {
	/// Currently checked out git commit hash
	pub commit_id: String,

	/// Currently checked out short git commit hash
	pub commit_short_id: Option<String>,

	/// Currently checked out commit date
	pub commit_date: String,

	/// `true` iff the repository had uncommitted changes when building the project.
	pub dirty: bool,

	/// Names the branch that is currently checked out, if any
	pub branch: Option<String>,

	/// All tags that point to the current commit (e.g., `["refs/tags/v0.0.10", "refs/tags/sample@v0.0.10"]`)
	pub tags: Vec<String>,
}
