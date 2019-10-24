use clap::{App, Arg, SubCommand};

pub fn parse() -> clap::ArgMatches<'static> {
    App::new("jira")
        .version("1.0")
        .author("Tino Rusch <tino.rusch@gmail.com>")
        .about("List your current tasks")
        .arg(
            Arg::with_name("assignee")
                .short("a")
                .long("assignee")
                .default_value("tinorusch1")
                .help("assignee of the issues (should be you)"),
        )
        .arg(
            Arg::with_name("pass-key")
                .long("pass-key")
                .default_value("misc/jira")
                .help("credential key"),
        )
        .arg(
            Arg::with_name("pass")
                .long("pass")
                .default_value("gopass")
                .help("pass binary to use"),
        )
        .subcommand(SubCommand::with_name("list").about("list issues"))
        .subcommand(
            SubCommand::with_name("show")
                .about("show details of an issue")
                .arg(Arg::with_name("id").help("id of the issue").required(true)),
        )
        .subcommand(
            SubCommand::with_name("details")
                .about("show all details of an issue")
                .arg(Arg::with_name("id").help("id of the issue").required(true))
                .arg(
                    Arg::with_name("format")
                        .help("format to use")
                        .long("format")
                        .short("f")
                        .default_value("json"),
                ),
        )
        .get_matches()
}
