use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use clap::Parser;
use url::Url;

pub fn run() -> anyhow::Result<()> {
    let stdin_reader = BufReader::new(std::io::stdin());
    for line in stdin_reader.lines() {
        let url = Url::parse(&line?)?;
        let query = url
            .query_pairs()
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect::<HashMap<String, String>>();
        if let Some(q) = query.get("q") {
            println!("{}", q);
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {}
