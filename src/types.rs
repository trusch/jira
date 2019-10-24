use serde::Deserialize;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub key: String,
    pub fields: Fields,
    #[serde(rename = "self")]
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct Fields {
    pub summary: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Resp {
    pub issues: Vec<Issue>,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let link = format_link(
            &format!("https://contiamo.atlassian.net/browse/{}", &self.key),
            &self.key,
        );
        let header = format!("{}: {}", link, &self.fields.summary);
        let underline = (0..header.len()).map(|_| "=").collect::<String>();
        write!(f, "{}\n", header)?;
        write!(f, "{}\n\n", underline)?;
        match &self.fields.description {
            Some(val) => write!(f, "{}\n", val),
            None => Ok(()),
        }
    }
}

impl fmt::Display for Resp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for issue in self.issues.iter() {
            let link = format_link(
                &format!("https://contiamo.atlassian.net/browse/{}", &issue.key),
                &issue.key,
            );
            write!(
                f,
                "{id}: {summary}\n",
                id = link,
                summary = issue.fields.summary
            )?;
        }
        Ok(())
    }
}

fn format_link(url: &String, text: &String) -> String {
    format!(
        "\x1b]8;;{url}\x07{text}\x1b]8;;\x07",
        url = url,
        text = text
    )
}
