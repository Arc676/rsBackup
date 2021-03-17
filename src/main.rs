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

fn main() {
	let opt = Options::from_args();
	println!("{:?}", opt);
}
