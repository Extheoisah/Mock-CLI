use std::io::stdout;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use structopt::clap::{crate_name, Shell};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// A fake git application that showcases building a CLI application with structopt
enum FGit {
    /// Create an empty Git repository or reinitialize an existing one
    ///
    /// This command creates an empty git repository - basically a .git directory with
    /// subdirectories for objects, refs/heads, refs/tags and template files.
    Init {
        #[structopt(short, long)]
        /// Be quiet
        _quiet: bool,
    },
    /// Show the working tree status
    ///
    /// Displays path that have differences between the index file and the commit,
    /// paths that have differences between the working tree and the paths
    /// in the workking tree that are not tracked by Git (and are not ignored
    /// by gitignore(5))
    Status { _pathspec: Option<String> },
    /// Auto generates shell completion
    Completion {
        #[structopt(possible_values = &["bash", "zsh"])]
        shell: Shell,
    },
    ///Launches an interactive fgit shell
    ///
    /// Launches an intractive fgit shell allowing the user to continually invoke fgit commands
    Shell,
}

fn main() {
    let opts = FGit::from_args();
    process_cmd(opts);
}

fn process_cmd(opt: FGit) {
    match opt {
        FGit::Init { _quiet: _ } => {
            println!("{:?}", opt);
        }
        FGit::Status { _pathspec: _ } => {
            println!("{:?}", opt);
        }
        FGit::Completion { shell } => {
            FGit::clap().gen_completions_to(crate_name!(), shell, &mut stdout());
        }
        FGit::Shell => repl(),
    }
}

fn repl() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.trim() == "shell" {
                    println!("Already within a shell");
                    continue;
                }

                let cmd = format!("{} {}", crate_name!(), line);
                match FGit::from_iter_safe(Vec::from_iter(cmd.split_whitespace().map(String::from)))
                {
                    Ok(pcmd) => {
                        process_cmd(pcmd);
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                }
                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
