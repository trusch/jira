use reqwest::Client;
use serde_json;
use serde_yaml;
use types::*;

mod credentials;
mod error;
mod flags;
mod types;

fn main() -> Result<()> {
    let matches = flags::parse();
    let pass_bin = String::from(matches.value_of("pass").unwrap());
    let pass_key = String::from(matches.value_of("pass-key").unwrap());
    match matches.subcommand() {
        ("show", Some(cmd)) => {
            let id = String::from(cmd.value_of("id").unwrap());
            match show_issue(&id, &pass_bin, &pass_key) {
                Ok(()) => {}
                Err(err) => exit(1, Some(err.description())),
            }
        }
        ("details", Some(cmd)) => {
            let id = String::from(cmd.value_of("id").unwrap());
            let format = String::from(cmd.value_of("format").unwrap());
            match show_issue_details(&id, &pass_bin, &pass_key, &format) {
                Ok(()) => {}
                Err(err) => exit(1, Some(err.description())),
            }
        }
        ("list", Some(_)) | (_, _) => {
            let assignee = String::from(matches.value_of("assignee").unwrap());
            match list_issues(&assignee, &pass_bin, &pass_key) {
                Ok(()) => {}
                Err(err) => exit(1, Some(err.description())),
            }
        }
    }
    Ok(())
}

fn list_issues(assignee: &String, pass: &String, key: &String) -> Result<()> {
    let (name, pw) = credentials::load(pass, key)?;
    let request_url = format!(
        "https://contiamo.atlassian.net/rest/api/2/search?jql=status in (\"In Progress\", \"In Review\", \"Selected for Development\") AND assignee in ({assignee}) order by created DESC",
        assignee = assignee,
    );
    let mut response = Client::new()
        .get(&request_url)
        .basic_auth(name, Some(pw))
        .send()?;
    let resp: types::Resp = response.json()?;
    print!("{}", resp);
    Ok(())
}

fn show_issue(id: &String, pass: &String, key: &String) -> Result<()> {
    let mut res = get_issue(id, pass, key)?;
    let issue: Issue = res.json()?;
    print!("{}", issue);
    Ok(())
}

fn show_issue_details(id: &String, pass: &String, key: &String, format: &String) -> Result<()> {
    let mut res = get_issue(id, pass, key)?;
    let full_issue: serde_json::Value = res.json()?;
    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&full_issue);
            println!("{}", json.unwrap());
        }
        "yaml" => {
            let yaml = serde_yaml::to_string(&full_issue);
            println!("{}", yaml.unwrap());
        }
        _ => {
            let msg = format!("unknown format '{}'", format);
            return Err(error::new_box(msg.as_str()));
        }
    }
    Ok(())
}

fn get_issue(id: &String, pass: &String, key: &String) -> Result<reqwest::Response> {
    let (name, pw) = credentials::load(pass, key)?;
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

fn exit(code: i32, msg: Option<&str>) {
    match msg {
        Some(msg) => eprintln!("{}", msg),
        None => {}
    }
    std::process::exit(code);
}
