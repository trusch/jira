extern crate clap;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;

use clap::{App, Arg};
use reqwest::{Client};
use serde::Deserialize;
use std::process::Command;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = get_flags();
    let assignee = String::from(matches.value_of("assignee").unwrap());
    let pass_bin = String::from(matches.value_of("pass").unwrap());
    let pass_key = String::from(matches.value_of("pass-key").unwrap());
    list_issues(assignee, pass_bin, pass_key)
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
        .get_matches()
}

fn list_issues(assignee: String, pass: String, key: String) -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://contiamo.atlassian.net/rest/api/2/search?jql=status in (\"In Progress\", \"In Review\", \"Selected for Development\") AND assignee in ({assignee}) order by created DESC",
        assignee = assignee,
    );

    let (name, pw) = get_credentials(pass, key)?;
    let mut response = Client::new()
        .get(&request_url)
        .basic_auth(name, Some(pw))
        .send()?;
    let resp: Resp = response.json()?;
    print_issues(&resp.issues);
    Ok(())
}

fn get_credentials(pass: String, key: String) -> Result<(String, String), Box<dyn std::error::Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", pass, key))
        .output()?;
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

fn format_link(url: &String, text: &String) -> String {
    format!("\x1b]8;;{url}\x07{text}\x1b]8;;\x07", url = url, text = text)
}
