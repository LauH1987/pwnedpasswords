#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate sha1;

use std::io::Read;
use std::env;
use std::collections::HashMap;
use sha1::{Sha1, Digest};


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn request_pwd(pwd: &str) -> Result<Option<String>> {
    let mut hasher = Sha1::new();
    hasher.input(pwd);
    let hash_result = hasher.result().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
    let (first_five, rest) = hash_result.split_at(5);

    let req_string = format!("{}{}", "https://api.pwnedpasswords.com/range/", first_five);

    let mut res = reqwest::get(&req_string)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let pwd_map: HashMap<&str, &str> = body.lines()
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .map(|vec| (vec[0], vec[1])).collect();

    let number_of_matches = pwd_map.get(rest).map(|s| s.to_string());

    Ok(number_of_matches)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pwd = match args.get(1) {
        Some(str) => str,
        _ => panic!("No password given"),
    };

    match request_pwd(pwd) {
        Ok(Some(matches)) => println!("Password matched {} times", matches),
        Ok(None) => println!("No matching password found"),
        _error => println!("Error trying to request server"),
    }
}