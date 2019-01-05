
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate error_chain;

#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate rocket_contrib;

use rocket::response::NamedFile;
use rocket::Config;
use rocket_contrib::serve::StaticFiles;

#[derive(Deserialize, Debug)]
struct GithubRepository {
    name: String,
    html_url: String,
}

#[derive(Deserialize, Debug)]
struct GitLabRepository {
    name: String,
    html_url_to_repo: String,
}


error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
    }
}


#[get("/")]
fn index() -> NamedFile{
    let file = NamedFile::open("src/pages/repository.html");
    file.unwrap()
}

#[get("/api/v0/github")]
fn get_repository() -> &'static str{
    "This will be an api endpoint"
}


fn get_github_repos() -> Result<Vec<GithubRepository>> {
    let req_url = format!("https://api.github.com/users/nehri97/repos");
    println!("[request url]: {}", req_url);
    let mut response = reqwest::get(&req_url)?;
    let repos: Vec<GithubRepository> = response.json()?;
    for repo in &repos {
        println!("{}", repo.name);
    }
    Ok(repos)
}

fn configure() -> rocket::Config {
    let mut config = Config::active().expect("Could not load configuration.");
    if let Ok(port_str) = std::env::var("PORT") {
        let port = port_str.parse().expect("Could not parse PORT.");
        config.set_port(port);
    }
    config
}


fn main() {
    rocket::custom(configure())
        .mount("/", routes![index])
        .mount("/public/", StaticFiles::from("./static"))
        .launch();
}
