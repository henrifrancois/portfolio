
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate error_chain;

#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate rocket_contrib;

use rocket::response::NamedFile;
use rocket::Config;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
struct GithubRepository {
    pub name: String,
    pub html_url: String,
}

    
#[derive(Serialize, Deserialize, Debug)]
struct Readme {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    name: String,
    url: String,
    content: String,
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
fn get_github_repos() -> Json<Vec<Repository>> {
    let req_url = format!("https://api.github.com/users/nehri97/repos");
    let mut response = reqwest::get(&req_url).unwrap();
    let repos: Vec<GithubRepository> = response.json().unwrap();
    let mut final_repos: Vec<Repository> = Vec::new();
    for i in repos {
        let readme_url = format!("https://api.github.com/repos/{owner}/{name}/readme", name = i.name, owner = "nehri97");
        let mut rm_response = reqwest::get(&readme_url).unwrap();
        let readme: Readme = rm_response.json().unwrap();
        let final_repo = Repository {
            name: i.name,
            url: i.html_url,
            content: readme.content
        };
        final_repos.push(final_repo);
    }
    Json(final_repos)
}

fn configure() -> rocket::Config {
    let mut config = Config::active().expect("Could not load configuration.");
    if let Ok(port_str) = std::env::var("PORT") {
        let port = port_str.parse().expect("Could not parse PORT.");
        config.set_port(port);
    }
    config
}


fn rocket() -> rocket::Rocket {
    rocket::custom(configure())
        .mount("/", routes![index, get_github_repos])
        .mount("/public/", StaticFiles::from("./static"))
}

fn main() {
        rocket().launch();
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};

    #[test]
    fn index() {
        let client = Client::new(rocket()).expect("Valid Rocket Instance");
        let content_type = ContentType::HTML;
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(content_type));
    }
}
