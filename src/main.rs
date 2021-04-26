use std::io::prelude::BufRead;

const CONFIG_FILEPATH: &str = "/home/fosseddy/.config/create-project.config";
const GITHUB_CREATE_REPO_URL: &str = "https://api.github.com/user/repos";

#[derive(Debug, Default)]
struct Config {
    projects_dir: String,
    github_api_key: String,
    github_username: String,
    allowed_languages: Vec<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Not enough arguments provided.");
        println!("Example: create-project <programming_language> <project_name>");
        std::process::exit(0);
    }

    // LOAD CONFIG
    let file = match std::fs::File::open(CONFIG_FILEPATH) {
        Ok(file) => file,
        Err(_) => {
            println!("Config file does not exist.");
            std::process::exit(0);
        }
    };
    let reader = std::io::BufReader::new(file);
    let mut config = Config::default();

    for line in reader.lines().map(|l| l.unwrap()) {
        let line = line.replace(" ", "");
        let pair: Vec<&str> = line.split('=').collect();
        let key = pair[0];
        let value = pair[1].to_string();

        match key {
            "githubAPIKey" => config.github_api_key = value,
            "githubUsername" => config.github_username = value,
            "projectsDir" => config.projects_dir = value,
            "allowedLanguages" => {
                config.allowed_languages = value.split(',').map(|v| v.to_string()).collect();
            }
            _ => {
                println!("Unknown config param: {}", key);
                std::process::exit(0);
            }
        };
    }

    // CREATE PROJECT
    let programming_language = &args[1];
    let project_name = &args[2];

    if !config.allowed_languages.contains(programming_language) {
        println!("Unknown programming language: {}", programming_language);
        println!("Allowed languages: {}", config.allowed_languages.join(", "));
        std::process::exit(0);
    }

    println!("Creating github repository...");

    let mut body = std::collections::HashMap::new();
    body.insert("name", project_name);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(GITHUB_CREATE_REPO_URL)
        .header(reqwest::header::AUTHORIZATION, format!("token {}", config.github_api_key))
        .header(reqwest::header::USER_AGENT, "reqwest")
        .json(&body)
        .send()
        .expect("Error sending http request");

    if !res.status().is_success() {
        println!("There was an error during repository creation.");
        println!("{}", res.text().unwrap());
        std::process::exit(0);
    }

    println!("Github repository successfully created.\n");

    // SHELL COMMANDS
    let project_path = format!("{}/{}", config.projects_dir, programming_language);
    let project_root = format!("{}/{}", project_path, project_name);

    println!("Cloning repository into {}...", project_root);

    if !std::path::Path::new(&project_path).exists() {
        std::fs::create_dir(&project_path).expect("Error creating project directory");
    }

    std::process::Command::new("git")
        .current_dir(&project_path)
        .arg("clone")
        .arg(format!("git@github.com:{}/{}.git", config.github_username, project_name))
        .status()
        .expect("Error executing shell command");

    println!("\nCreating README.md and .gitignore...\n");

    std::process::Command::new("touch")
        .current_dir(&project_root)
        .args(&["README.md", ".gitignore"])
        .status()
        .expect("Error executing shell command");

    println!("Pushing changes into the repository...");

    std::process::Command::new("git")
        .current_dir(&project_root)
        .args(&["add", "."])
        .status()
        .expect("Error executing shell command");

    std::process::Command::new("git")
        .current_dir(&project_root)
        .args(&["commit", "-m", "Initial commit"])
        .status()
        .expect("Error executing shell command");

    std::process::Command::new("git")
        .current_dir(&project_root)
        .args(&["push", "origin", "master"])
        .status()
        .expect("Error executing shell command");

    println!("\nSuccess.");
}
