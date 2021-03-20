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

use std::vec::Vec;
use std::io::{BufRead, Result, Error};
use std::path::PathBuf;

pub struct Task {
	id: String,
	is_update: bool,
	src: Option<PathBuf>,
	dst: Option<PathBuf>,
	backup_path: Option<PathBuf>,
	link_dest: Vec<PathBuf>,
	compare_dest: Vec<PathBuf>,
	exclude_from: Option<PathBuf>,
	include_from: Option<PathBuf>,
	files_from: Option<PathBuf>
}

impl Task {
	fn new() -> Self {
		Task {
			id: String::from("New Task"), is_update: true,
			src: None, dst: None, backup_path: None,
			link_dest: Vec::new(), compare_dest: Vec::new(),
			exclude_from: None, include_from: None, files_from: None
		}
	}

	fn from_reader(mut reader: impl BufRead) -> Result<Self> {
		let mut task = Task::new();
		for line in reader.lines() {
			if let Err(err) = line {
				return Err(err);
			}
		}
		Ok(task)
	}
}
