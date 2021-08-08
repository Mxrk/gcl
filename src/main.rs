use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = match env::var("gcl") {
        Ok(path) => path,
        Err(error) => panic!("Couldn't load path from env 'gcl': {}", error),
    };

    if args.len() <= 1 {
        println!("Usage: {} <url>", &args[0]);
        return;
    }

    let mut url = args[1].to_owned();
    // ./gcl, url -> len 2
    // ./gcl, -p, language, url -> len  4
    if args.len() == 4 {
        path = [path, args[2].to_owned()].join("/");
        url = args[3].to_owned();
    } else if args.len() != 2 {
        println!("Usage: {} -p <project> <url>", &args[0]);
        return;
    }

    // 0 https:// - 1 empty, 2 github.com, 3 user, 4 project
    let split: Vec<&str> = url.split("/").collect();
    // create folder structure
    let project_path = [&path, split[2], split[3], split[4]].join("/");

    create_dir(&project_path);

    clone(&url, &project_path);
}

// create_dir creates all folders for a given path
fn create_dir(project_path: &str) {
    match fs::create_dir_all(project_path) {
        Ok(()) => println!("Creating project path: {}", &project_path),
        Err(e) => panic!(
            "Failed to create project path: {} because {}",
            &project_path, e
        ),
    }
}

// clone is a function that clones a git repository into a given path
fn clone(url: &str, project_path: &str) {
    // building the following string
    // git clone url dest
    let mut cmd = Command::new("git");
    let status = cmd
        .arg("clone")
        .arg(url)
        .arg(project_path)
        .status()
        .expect("git command failed to start");

    if status.success() {
        println!("Sucessfully cloned");
    } else {
        panic!("failed to clone, err: {}", status);
    }
}
