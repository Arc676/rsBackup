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

use std::process;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rsbackup", about = "rsync backup utility written in Rust")]
struct Options {
	#[structopt(short = "f", long = "conf", parse(from_os_str))]
	config: Option<PathBuf>,

	#[structopt(long)]
	always_confirm: bool,

	#[structopt(long)]
	ask: bool,

	#[structopt(long)]
	debug: bool,

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
	download: bool
}

fn operation_failed(err: &str, qof: bool) -> bool {
	println!("{}", err);
	if qof {
		return true
	}
	print!("Continue backup? [y/N]: ");
	let mut input = String::new();
	io::stdout().flush().expect("Failed to flush");
	match io::stdin().read_line(&mut input) {
		Ok(_) => match input.trim() {
			"y" | "Y" => false,
			_ => {
				println!("User canceled");
				true
			}
		},
		Err(_) => {
			panic!("Failed to read");
		}
	}
}

fn run_backup(opt: &Options) -> bool {
	let path = match &opt.config {
		Some(path) => path.as_path(),
		None => Path::new("~/.arcutillib/backup.conf")
	};
	let result = File::open(&path);
	if let Err(why) = result {
		operation_failed(&why.to_string(), true);
		return false;
	}
	let mut config = result.unwrap();
	let mut config_reader = BufReader::new(config);
	true
}

fn main() {
	let opt = Options::from_args();
	process::exit(match run_backup(&opt) {
		true => 1,
		false => 0
	});
}
