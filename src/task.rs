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
use std::io::BufRead;
use std::path::PathBuf;

pub struct Task {
	id: String,
	is_update: bool,
	always_confirm: bool,
	src: Option<PathBuf>,
	dst: Option<PathBuf>,
	backup_path: Option<PathBuf>,
	compare_paths: bool,
	link_dest: Vec<PathBuf>,
	compare_dest: Vec<PathBuf>,
	exclude_from: Option<PathBuf>,
	include_from: Option<PathBuf>,
	files_from: Option<PathBuf>
}

impl Task {
	fn new() -> Self {
		Task {
			id: String::from("New Task"), is_update: true, always_confirm: false,
			src: None, dst: None, backup_path: None, compare_paths: false,
			link_dest: Vec::new(), compare_dest: Vec::new(),
			exclude_from: None, include_from: None, files_from: None
		}
	}

	pub fn should_confirm(&self) -> bool {
		self.always_confirm
	}

	pub fn is_update_task(&self) -> bool {
		self.is_update
	}

	pub fn get_id(&self) -> &str {
		self.id.as_str()
	}

	pub fn get_description(&self) -> String {
		format!("{} -> {}",
			self.src.as_ref().map_or("", |p| p.to_str().unwrap_or("")),
			self.dst.as_ref().map_or("", |p| p.to_str().unwrap_or("")))
	}

	pub fn from_reader(reader: &mut impl BufRead) -> Result<Self, String> {
		let mut task = Task::new();
		let mut type_determined = false;
		loop {
			let mut line1 = String::new();
			match reader.read_line(&mut line1) {
				Ok(len) => {
					if len == 0 {
						return Err(String::from("EOF"));
					}
				},
				Err(err) => {
					return Err(err.to_string());
				}
			}
			if line1.starts_with("#") {
				continue;
			}
			let line = line1.trim();
			if !type_determined {
				match line {
					"[BACKUP]" => { task.is_update = false },
					"[UPDATE]" => {},
					_ => {
						return Err(String::from("Failed to parse configuration file. Could not find task."));
					}
				};
				type_determined = true;
				continue;
			}
			if line == "[END]" {
				break;
			}
			if let Some(path) = line.strip_prefix("SRC=") {
				task.src = Some(PathBuf::from(path));
			} else if let Some(path) = line.strip_prefix("DST=") {
				task.dst = Some(PathBuf::from(path));
			} else if let Some(path) = line.strip_prefix("EXFR=") {
				task.exclude_from = Some(PathBuf::from(path));
			} else if let Some(path) = line.strip_prefix("INFR=") {
				task.include_from = Some(PathBuf::from(path));
			} else if let Some(path) = line.strip_prefix("FIFR=") {
				task.files_from = Some(PathBuf::from(path));
			} else if let Some(path) = line.strip_prefix("BPATH=") {
				if task.is_update {
					return Err(String::from("Unexpected BPATH parameter in update task configuration."));
				} else {
					task.backup_path = Some(PathBuf::from(path));
				}
			} else if let Some(name) = line.strip_prefix("ID=") {
				task.id = name.to_string();
			} else {
				match line {
					"[CONFIRM]" => { task.always_confirm = true; },
					"[COMPARE BPATH]" => {
						if task.is_update {
							return Err(String::from("Unexpected [COMPARE BPATH] tag in update task configuration."));
						} else {
							task.compare_paths = true;
						}
					},
					_ => {
						return Err(format!("Unexpected line '{}' in configuration.", line));
					}
				};
			}
		}
		if !type_determined {
			return Err(String::from("EOF"));
		}
		match task.src {
			Some(ref path) => {
				if !path.exists() {
					return Err(String::from("Source path nonexistent or inaccessible."));
				}
			},
			None => {
				return Err(String::from("No source path specified."));
			}
		};
		match task.dst {
			Some(ref path) => {
				if !path.exists() {
					return Err(String::from("Destination path nonexistent or inaccessible."));
				}
			},
			None => {
				return Err(String::from("No destination path specified."));
			}
		};
		Ok(task)
	}
}
