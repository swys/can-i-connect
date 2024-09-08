use clap::{crate_version, Arg, Command};

pub fn argc_app() -> Command {
	Command::new("can-i-connect")
		.version(crate_version!())
		.about("tool to check connectivity to various hosts using HTTP or TCP")
		.arg(
			Arg::new("http-hosts")
				.help("comma seperated list of http hosts to attempt to connect to")
				.long("http-hosts"),
		)
		.arg(
			Arg::new("tcp-hosts")
				.help("comma seperated list of tcp hosts to attempt to connect to. Required format: <dns name or ip address>:<port>")
				.long("tcp-hosts"),
		)
		.arg(
			Arg::new("timeout")
				.help("how much time in seconds to wait while connecting to a host before giving up")
				.long("timeout"),
		)
		.arg(
			Arg::new("log-level")
				.help("set the log level {info|error|debug}")
				.long("log-level")
				.default_value("info"),
		)
		.arg(
			Arg::new("no-color")
				.help("remove color from log output")
				.long("no-color")
        .action(clap::ArgAction::SetFalse)
		)
}
