use std::str::FromStr;
use std::env;
use git2::{Repository, StatusOptions};

// Basically stolen from the rust book and sample CLI repo, see here: https://github.com/rust-cli/cli-template/
// Ideally, we would use this struct to hold all arguments passed from the user, 
// The crate StructOpt will handle creating the struct from the cmd line args, handles error messages too, 
// We can work off this for quick results
// edit: I will not use StructOpt as I wanna do something else invloving some enum tomfoolery
#[derive(Debug)]
struct Cli {
    cmd: Command
}

impl Cli {
    fn from_args(args: &[String]) -> Cli {
        let query = args[1].to_lowercase();
        let cmd = match Command::from_str(&query) {
            Ok(cmd) => cmd,
            Err(_e) => panic!("{} command! WHHAAATT!", &query),
        };

        Cli { cmd }
    }
}

// I think having an enum to handle command control flow is handy and fairly useful, 
// Buut, i am not very familiar with how they are used and haven't done it well yet
// At least for now, with them we have a clear path to completeion
#[derive(Debug, PartialEq)]
enum Command {
    Push,
    Pull,
    Fetch,
    Status,
    Diff,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "push"  => Ok(Command::Push),
            "pull"  => Ok(Command::Pull),
            "fetch" => Ok(Command::Fetch),
            "status"=> Ok(Command::Status),
            "diff"  => Ok(Command::Diff),
            _       => Err(()),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_args = Cli::from_args(&args);

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let result = repo.statuses(Some(&mut StatusOptions::new())).unwrap();

    println!("result: {:?}", result.get(0));
}