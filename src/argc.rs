use clap::{crate_version, Arg, Command};

pub fn argc_app() -> Command {
	Command::new("can-i-connect")
		.version(crate_version!())
		.about("tool to check connectivity to various hosts using HTTP or TCP")
		.arg(
			Arg::new("http-hosts")
				.help("comma seperated list of http hosts to attempt to connect to")
				.long("http-hosts")
        .value_name("https://example.com"),
		)
		.arg(
			Arg::new("tcp-hosts")
				.help("comma seperated list of tcp hosts to attempt to connect to. Required format: <dns name or ip address>:<port>")
				.long("tcp-hosts")
        .value_name("example.com:80"),
		)
		.arg(
			Arg::new("timeout")
				.help("how much time in seconds to wait while connecting to a host before giving up")
				.long("timeout")
        .value_name("5"),
		)
		.arg(
			Arg::new("log-level")
				.help("set the log level {info|error|debug}")
				.long("log-level")
        .value_name("debug")
				.default_value("info"),
		)
		.arg(
			Arg::new("no-color")
				.help("remove color from log output")
				.long("no-color")
        .action(clap::ArgAction::SetFalse)
		)
    .arg(
      Arg::new("listen")
        .help("run in Server Mode by binding to <ip address>:<port> e.g. 127.0.0.1:8000 or [::1]:8000")
        .long("listen")
        .value_name("127.0.0.1:8000")
    )
}
