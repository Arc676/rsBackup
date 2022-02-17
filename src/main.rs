// Copyright (C) 2021 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3).

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::io;
use std::io::Write;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;

use std::path::PathBuf;
use structopt::StructOpt;

mod task;

#[derive(Debug, StructOpt)]
#[structopt(name = "rsbackup", about = "rsync backup utility written in Rust")]
struct Options {
    #[structopt(short = "f", long = "conf", parse(from_os_str))]
    config: Option<PathBuf>,

    #[structopt(long)]
    ask: bool,

    #[structopt(long)]
    debug: bool,

    #[structopt(long)]
    dry_run: bool,

    #[structopt(long)]
    link: bool,

    #[structopt(short, long)]
    quiet: bool,

    #[structopt(long)]
    up_only: bool,

    #[structopt(long)]
    id_tasks: bool,

    #[structopt(short = "s", long = "safe")]
    quit_on_fail: bool,

    #[structopt(long)]
    download: bool,
}

fn operation_failed(err: &str, qof: bool) -> bool {
    println!("{}", err);
    if qof {
        return true;
    }
    if !get_yn("Continue backup?", false) {
        println!("User canceled");
        return true;
    }
    false
}

fn get_yn(prompt: &str, default: bool) -> bool {
    print!(
        "{} [{}]: ",
        prompt,
        match default {
            true => "Y/n",
            false => "y/N",
        }
    );
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush");
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "y" | "Y" => true,
            "n" | "N" => false,
            _ => default,
        },
        Err(_) => {
            panic!("Failed to read");
        }
    }
}

fn run_backup(opt: &Options) -> bool {
    if opt.debug {
        println!("Running in debug mode...");
    }
    let path = match &opt.config {
        Some(path) => path.as_path(),
        None => {
            println!("No configuration file specified. Defaulting to ~/.arcutillib/backup.conf");
            Path::new("~/.arcutillib/backup.conf")
        }
    };
    let result = File::open(&path);
    if let Err(why) = result {
        let err = format!("Failed to read configuration file: {}", why);
        operation_failed(&err, true);
        return false;
    }
    let config = result.unwrap();
    let mut config_reader = BufReader::new(config);
    loop {
        match task::Task::from_reader(&mut config_reader, opt.debug) {
            Ok(task) => {
                if task.is_update_task() {
                    println!("Found update task.");
                } else {
                    if opt.up_only {
                        continue;
                    }
                    println!("Found backup task.");
                }
                if opt.id_tasks {
                    println!("Task ID: {}", task.get_id());
                }
                if opt.ask || task.should_confirm() {
                    let prompt = format!(
                        "{} {}\nRun task?",
                        match task.is_update_task() {
                            true => match opt.download {
                                true => "Download",
                                false => "Upload",
                            },
                            false => "Backup",
                        },
                        task.get_description()
                    );
                    if !get_yn(&prompt, true) {
                        continue;
                    }
                }
                if let Err(why) = task.run_task(opt.quiet, opt.debug, opt.dry_run) {
                    let err = format!("Backup failed: {}", why);
                    if operation_failed(&err, opt.quit_on_fail) {
                        break;
                    }
                }
            }
            Err(err) => match err.as_str() {
                "EOF" => break,
                _ => {
                    println!("Failed to construct task: {}", err);
                    return false;
                }
            },
        }
    }
    println!("Backup complete.");
    true
}

fn main() {
    let opt = Options::from_args();
    process::exit(match run_backup(&opt) {
        true => 1,
        false => 0,
    });
}
