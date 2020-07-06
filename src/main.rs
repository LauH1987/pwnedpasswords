use reqwest::Result;
use std::collections::HashMap;
use str_ext::Sha1Hash;

mod str_ext;

async fn request_pwd(pwd: &str) -> Result<Option<String>> {
    let hash_result = pwd.sha1_hash();
    let (first_five, rest) = hash_result.split_at(5);

    let req_string = format!("{}{}", "https://api.pwnedpasswords.com/range/", first_five);
    let body = reqwest::get(&req_string).await?.text().await?;

    let pwd_map: HashMap<&str, &str> = body
        .lines()
        .map(|line| line.split(':'))
        .map(|mut iter| (iter.next().unwrap(), iter.next().unwrap()))
        .collect();

    let number_of_matches = pwd_map.get(rest).map(|s| s.to_string());
    Ok(number_of_matches)
}

#[tokio::main]
async fn main() {
    let pwd = rpassword::prompt_password_stdout("Password: ").unwrap();

    match request_pwd(&pwd).await {
        Ok(Some(matches)) => println!("Password matched {} times", matches),
        Ok(None) => println!("No matching password found"),
        _error => println!("Error trying to request server"),
    }
}
