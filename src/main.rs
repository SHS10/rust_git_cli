use std::env;
use std::str::FromStr;

// Basically stolen from the rust book and sample CLI repo, see here: https://github.com/rust-cli/cli-template/
// Ideally, we would use this struct to hold all arguments passed from the user,
// The crate StructOpt will handle creating the struct from the cmd line args, handles error messages too,
// We can work off this for quick results
// edit: I will not use StructOpt as I wanna do something else invloving some enum tomfoolery
#[derive(Debug)]
struct Cli {
    cmd: Command,
    path: String, // It being string is more costly but we dont need to worry about lifetimes
}

impl Cli {
    fn from_args(args: &[String]) -> Option<Cli> {
        if args.len() > 1 {
            let query = args[1].to_lowercase();
            let path = args[2].clone();

            match Command::from_str(&query) {
                Ok(cmd) => Some(Cli { cmd, path }),
                Err(_e) => {
                    Cli::emit_error();
                    None
                }
            }
        } else {
            Cli::emit_error();
            None
        }
    }

    fn emit_error() {
        println!("There seems to be an error here...");
    }

    fn emit_help() {
        println!("###############");
        println!("##   help    ##");
        println!("###############");
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
            "push" => Ok(Command::Push),
            "pull" => Ok(Command::Pull),
            "fetch" => Ok(Command::Fetch),
            "status" => Ok(Command::Status),
            "diff" => Ok(Command::Diff),
            _ => Err(()),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match Cli::from_args(&args) {
        Some(cli) => match cli.cmd {
            Command::Status => git::status(&cli.path),
            Command::Diff => git::diff(&cli.path),
            Command::Push => println!("Not yet implemented"),
            Command::Pull => println!("Not yet implemented"),
            Command::Fetch => println!("Not yet implemented"),
        },
        None => {
            Cli::emit_help();
        }
    };
}

mod git {
    use git2::{Repository, Status, StatusOptions};

    /// Simple Struct to hold the file status information
    /// Might expand a bit to handle the additional info from stuff like git diff, etc
    struct FileStatus {
        new: Vec<String>,
        modified: Vec<String>,
        deleted: Vec<String>,
    }

    impl FileStatus {
        fn new() -> FileStatus {
            FileStatus {
                new: Vec::new(),
                modified: Vec::new(),
                deleted: Vec::new(),
            }
        }

        fn add_new(&mut self, new_file: &str) {
            self.new.push(String::from(new_file));
        }

        fn add_mod(&mut self, mod_file: &str) {
            self.modified.push(String::from(mod_file));
        }

        fn add_del(&mut self, del_file: &str) {
            self.deleted.push(String::from(del_file));
        }

        fn display(&self) {
            if self.modified.len() > 0 {
                println!("Files edited: ");
                for modified_file in &self.modified {
                    println!("{}", modified_file);
                }
            }

            if self.new.len() > 0 {
                println!("Files Added: ");
                for new_file in &self.new {
                    println!("{}", new_file);
                }
            }

            if self.deleted.len() > 0 {
                println!("Files Deleted: ");
                for deleted_file in &self.deleted {
                    println!("{}", deleted_file);
                }
            }
        }
    }

    pub fn status(repo_path: &str) {
        /*
        There is a number of file statues we need to consider,
        Here is the full list, I am not sure if we will use it all yet but, here it is

        CURRENT: Status
        INDEX_NEW: Status
        INDEX_MODIFIED: Status
        INDEX_DELETED: Status
        INDEX_RENAMED: Status
        INDEX_TYPECHANGE: Status
        WT_NEW: Status
        WT_MODIFIED: Status
        WT_DELETED: Status
        WT_TYPECHANGE: Status
        WT_RENAMED: Status
        IGNORED: Status
        CONFLICTED: Status
        */
        let repo = open_repo(repo_path);

        let mut options = StatusOptions::new();
        options.show(git2::StatusShow::IndexAndWorkdir);
        let statuses = repo.statuses(Some(&mut options)).unwrap();
        let mut filestatus = FileStatus::new();

        for status in statuses.iter() {
            //only considering WT_ statues for now (duuno what the rest mean, havent read the docs completely :O )
            match status.status() {
                Status::WT_NEW => filestatus.add_new(status.path().unwrap()),
                Status::WT_MODIFIED => filestatus.add_mod(status.path().unwrap()),
                Status::WT_DELETED => filestatus.add_del(status.path().unwrap()),
                Status::WT_TYPECHANGE => (),
                Status::WT_RENAMED => (),
                Status::IGNORED => (),
                Status::CONFLICTED => (),
                _ => (),
            };
        }

        filestatus.display();
    }

    pub fn diff(repo_path: &str) {}

    pub fn fetch() {}

    pub fn push() {}

    pub fn pull() {}

    fn open_repo(repo_path: &str) -> Repository {
        match Repository::open(repo_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        }
    }
}
