extern crate clap;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;

use clap::{App, Arg, SubCommand};
use reqwest::{Client};
use serde::Deserialize;
use std::process::Command;

mod error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
struct Issue {
    key: String,
    fields: Fields,
    #[serde(rename="self")]
    link: String,
}

#[derive(Deserialize, Debug)]
struct Fields {
    summary: String,
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Resp {
    issues: Vec<Issue>,
}

fn main() -> Result<()> {
    let matches = get_flags();
    let pass_bin = String::from(matches.value_of("pass").unwrap());
    let pass_key = String::from(matches.value_of("pass-key").unwrap());

    match matches.subcommand() {
        ("show", Some(cmd)) => {
            let id = String::from(cmd.value_of("id").unwrap());
            match show_issue(&id, &pass_bin, &pass_key) {
                Ok(()) => {},
                Err(err) => exit(1, Some(err.description())),
            }
        },
        ("details", Some(cmd)) => {
            let id = String::from(cmd.value_of("id").unwrap());
            let format = String::from(cmd.value_of("format").unwrap());
            match show_issue_all(&id, &pass_bin, &pass_key, &format) {
                Ok(()) => {},
                Err(err) => exit(1, Some(err.description())),
            }
        },
        _ => { // list issues by default
            let assignee = String::from(matches.value_of("assignee").unwrap());
            match list_issues(&assignee, &pass_bin, &pass_key) {
                Ok(()) => {},
                Err(err) => exit(1, Some(err.description())),
            }
        },
    }

    Ok(())
}

fn get_flags() -> clap::ArgMatches<'static> {
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
        .subcommand(SubCommand::with_name("show")
            .about("show details of an issue")
            .arg(Arg::with_name("id")
                .help("id of the issue")
                .required(true)
            ),
        )
        .subcommand(SubCommand::with_name("details")
            .about("show all details of an issue")
            .arg(Arg::with_name("id")
                .help("id of the issue")
                .required(true)
            )
            .arg(Arg::with_name("format")
                .help("format to use")
                .long("format")
                .short("f")
                .default_value("json")
            ),
        )
        .get_matches()
}

fn list_issues(assignee: &String, pass: &String, key: &String) -> Result<()> {
    let (name, pw) = get_credentials(pass, key)?;

    let request_url = format!(
        "https://contiamo.atlassian.net/rest/api/2/search?jql=status in (\"In Progress\", \"In Review\", \"Selected for Development\") AND assignee in ({assignee}) order by created DESC",
        assignee = assignee,
    );

    let mut response = Client::new()
        .get(&request_url)
        .basic_auth(name, Some(pw))
        .send()?;
    let resp: Resp = response.json()?;

    print_issues(&resp.issues);
    Ok(())
}

fn get_issue(id: &String, pass: &String, key: &String) -> Result<reqwest::Response> {
    let (name, pw) = get_credentials(pass, key)?;
    let request_url = format!(
        "https://contiamo.atlassian.net/rest/api/2/issue/{id}",
        id = id,
    );
    let response = Client::new()
        .get(&request_url)
        .basic_auth(name, Some(pw))
        .send()?;
    Ok(response)
}

fn show_issue(id: &String, pass: &String, key: &String) -> Result<()> {
    let mut res = get_issue(id, pass, key)?;
    let issue: Issue = res.json()?;
    print_issue_details(&issue);
    Ok(())
}

fn show_issue_all(id: &String, pass: &String, key: &String, format: &String) -> Result<()> {
    let mut res = get_issue(id, pass, key)?;
    let full_issue: serde_json::Value = res.json()?;
    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&full_issue);
            println!("{}",  json.unwrap());
        },
        "yaml" => {
            let yaml = serde_yaml::to_string(&full_issue);
            println!("{}",  yaml.unwrap());
        },
        _ => {
            let msg = format!("unknown format '{}'", format);
            return Err(error::new_box(msg.as_str()));
        }

    }
    Ok(())
}

fn get_credentials(pass: &String, key: &String) -> Result<(String, String)> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", pass, key))
        .output()?;
    if output.status.code().unwrap() != 0 {
        return Err(error::new_box("failed to get credentials"));
    }
    let str = String::from_utf8(output.stdout).unwrap();
    let parts: Vec<&str> = str.split("\n").collect();
    let pw = String::from(parts[0]);
    let name_line = String::from(parts[1]);
    let name_parts: Vec<&str> = name_line.split(" ").collect();
    let name = String::from(name_parts[1]);
    Ok((name, pw))
}

fn print_issues(issues: &Vec<Issue>) {
    for issue in issues.iter() {
        let link = format_link(&format!("https://contiamo.atlassian.net/browse/{}", &issue.key), &issue.key);
        println!(
            "{id}: {summary}",
            id = link,
            summary = issue.fields.summary
        )
    }
}

fn print_issue_details(issue: &Issue) {
    let link = format_link(&format!("https://contiamo.atlassian.net/browse/{}", &issue.key), &issue.key);
    let header = format!("{}: {}", link, issue.fields.summary);
    let underline = (0..header.len()).map(|_| "=").collect::<String>();
    println!("{}", header);
    println!("{}\n", underline);
    match &issue.fields.description {
        Some(val) => println!("{}", val),
        None => {},
    }
}

fn format_link(url: &String, text: &String) -> String {
    format!("\x1b]8;;{url}\x07{text}\x1b]8;;\x07", url = url, text = text)
}

fn exit(code: i32, msg: Option<&str>) {
    match msg {
        Some(msg) => eprintln!("{}", msg),
        None => {},
    }
    std::process::exit(code);
}
