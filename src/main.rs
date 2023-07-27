use std::io;
use std::io::prelude::*;
use std::fs::{self};
use std::path::Path;
use regex::Regex;
use url::Url;
use colored::Colorize;

#[derive(Debug, Clone)]
struct Account {
    url: String,
    username: String,
    password: String,
    application: String,
}

fn read_clouds(dir: &Path, accounts: &mut Vec<Account>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                read_clouds(&path, accounts)?;
            } else {
                if let Some(ext) = path.extension() {
                    if ext == "txt" {
                        if let Ok(file) = fs::File::open(&path) {
                            let reader = io::BufReader::new(file);
                            let mut account = Account {
                                url: String::new(),
                                username: String::new(),
                                password: String::new(),
                                application: String::new()
                            };
                            for line in reader.lines() {
                                let line = line?;

                                if line.starts_with("URL: ") {
                                    account.url = line.replace("URL: ", "");
                                } else if line.starts_with("Username: ") {
                                    account.username = line.replace("Username: ", "");
                                } else if line.starts_with("Password: ") {
                                    account.password = line.replace("Password: ", "");
                                } else if line.starts_with("Application: ") {
                                    account.application = line.replace("Application: ", "");

                                    accounts.push(account);
                                    account = Account {
                                        url: String::new(),
                                        username: String::new(),
                                        password: String::new(),
                                        application: String::new()
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() -> io::Result<()> {
    let directory = "./databases";
    let mut logins: Vec<Account> = Vec::new();
    let mut input = String::new();

    read_clouds(Path::new(directory), &mut logins)?;
    clear();
    println!("Input a url to filter:");
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_string();
    let url = Regex::new(r"^(https?://)[a-zA-Z0-9.-]+(\.[a-zA-Z]{2,})").unwrap();
    if url.is_match(&input) {
        let domain = Url::parse(&input).map_err(|err| {
            io::Error::new(io::ErrorKind::Other, format!("Error parsing URL: {}", err))
        })?;
        let result: Vec<Account> = logins.iter().filter(|acc| acc.url.contains(domain.as_str())).cloned().collect();
        if result.len() < 1 {
            println!("No results!");
        } else {
            for account in result {
                println!("\n");
                println!("{}: {}", "URL".red().bold(), account.url);
                println!("{}: {}", "Username".red().bold(), account.username);
                println!("{}: {}", "Password".red().bold(), account.password);
                println!("{}: {}", "Application".red().bold(), account.application);
                println!("{}", "______________________________________".red());
            }
        }
    } else {
        println!("Invalid URL!")
    }
    Ok(())
}