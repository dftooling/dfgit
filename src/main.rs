use core::time;
use std::thread::sleep;
use clap::{Parser, Subcommand};

mod ccapi;
mod template;
mod repo;



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Clone {
        repo: String,

        #[arg(long)]
        dir: Option<String>
    },
    Commit {
        repo: String,
        message: String,

        #[arg(long)]
        dir: Option<String>
    }
}



fn push(repo: String, message: String, dir_override: Option<String>) {
    println!("Cloning...");
    let mut repo = repo::Repo::clone(&repo);
    let mut cc = ccapi::CCAPI::connect();

    match dir_override {
        Some(o) => repo.set_dir_override(o),
        None => {}
    }

    println!("You will be prompted in-game to run /auth. Please do so to continue.");
    cc.wait_for_auth();

    println!("Scanning...");
    let templates = cc.scan();
    repo.write_templates(templates);

    println!("Pushing...");
    repo.push(message);

    println!("Cleaning up...");
    repo.delete();
    //cc.close();
}

fn clone(repo: String, dir_override: Option<String>) {
    println!("Cloning...");
    let mut repo = repo::Repo::clone(&repo);
    let mut cc = ccapi::CCAPI::connect();

    match dir_override {
        Some(o) => repo.set_dir_override(o),
        None => {}
    }

    println!("You will be prompted in-game to run /auth. Please do so to continue.");
    cc.wait_for_auth();
    cc.clear();
    sleep(time::Duration::from_secs(1)); // Sleep to give time for the plot to clear

    println!("Placing...");
    cc.place(repo.read_templates());

    println!("Cleaning up...");
    repo.delete();
    //cc.close();
}

fn main() {
    //push("https://github.com/dfgit-tool/test-repo".to_owned(), String::from("doing some testing again"));
    //clone("https://github.com/dfgit-tool/test-repo".to_owned());
    let cli = Cli::parse();

    match &cli.command {
        Commands::Clone { repo, dir } => {
            clone(repo.to_string(), dir.clone());
        }
        Commands::Commit { repo, message, dir } => {
            push(repo.to_string(), message.to_string(), dir.clone());
        }
    }
}
