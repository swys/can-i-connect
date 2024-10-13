use lazy_static::lazy_static;

lazy_static! {
		pub static ref VERSION: String = {
				let cargo_version = env!("CARGO_PKG_VERSION");
				let git_hash = option_env!("GIT_HASH").unwrap_or("unknown");
				format!("{}-{}", cargo_version, git_hash)  // Now we can use format!
		};
}
