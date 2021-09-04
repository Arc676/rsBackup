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
use std::process::Command;
use std::result::Result;

use chrono::Utc;
use std::fs;

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
	exclude_others: bool,
	exclude_from: Option<PathBuf>,
	include_from: Option<PathBuf>,
	files_from: Option<PathBuf>
}

trait ToString {
	fn to_string(self) -> String;
}

impl ToString for &Option<PathBuf> {
	fn to_string(self) -> String {
		self.as_ref().map_or("", |p| p.to_str().unwrap_or("")).to_string()
	}
}

impl ToString for &PathBuf {
	fn to_string(self) -> String {
		AsRef::<std::path::Path>::as_ref(&self.as_path()).to_str().unwrap_or("").to_string()
	}
}

impl Task {
	fn new() -> Self {
		Task {
			id: String::from("New Task"), is_update: true, always_confirm: false,
			src: None, dst: None, backup_path: None, compare_paths: false,
			link_dest: Vec::new(), compare_dest: Vec::new(),
			exclude_others: false, exclude_from: None, include_from: None, files_from: None
		}
	}

	pub fn run_task(&self, quiet: bool, debug: bool, dry_run: bool) -> Result<(), String> {
		let mut args = vec![String::from("--exclude"), String::from(".*")];
		args.push(String::from(match self.is_update {
			true => "-ru",
			false => "-rt"
		}));
		if !quiet {
			args.push(String::from("-h"));
			args.push(String::from("--progress"));
			args.push(String::from("--verbose"));
		}

		if let Some(path) = &self.files_from {
			args.push(format!("--files-from={}", path.to_string()));
		}
		if let Some(path) = &self.exclude_from {
			args.push(format!("--exclude-from={}", path.to_string()));
		}
		if let Some(path) = &self.include_from {
			args.push(format!("--include-from={}", path.to_string()));
		}
		if self.exclude_others {
			args.push(String::from("--exclude"));
			args.push(String::from("*"));
		}

		for path in &self.link_dest {
			args.push(format!("--link-dest={}", path.to_string()));
		}
		for path in &self.compare_dest {
			args.push(format!("--compare-dest={}", path.to_string()));
		}
		if self.compare_paths {
			let path = self.backup_path.as_ref().unwrap().as_path();
			match fs::read_dir(path) {
				Ok(iterator) => {
					for entry in iterator {
						if let Ok(dir) = entry {
							if dir.path().is_dir() {
								args.push(format!("--compare-dest={}", dir.path().to_string()));
							}
						}
					}
				},
				Err(why) => {
					return Err(format!("Failed to read backup directory: {}", why));
				}
			};
		}
		if dry_run {
			args.push(String::from("--dry-run"));
		}
		args.push(self.src.to_string());
		if self.is_update {
			args.push(self.dst.to_string());
		} else {
			args.push(format!("{}/{}", self.dst.to_string(), Utc::now().format("%Y-%m-%d--%H_%M")));
		}
		if debug {
			println!("DEBUG: rsync {}", args.join(" "));
			return Ok(());
		}
		let mut cmd = Command::new("rsync");
		match cmd.args(args).spawn() {
			Ok(mut child) => match child.wait() {
				Ok(status) => match status.success() {
					true => Ok(()),
					false => Err(format!("rsync failed with exit code {}", match status.code() {
						Some(code) => format!("{}", code),
						None => String::from("(?)")
					}))
				},
				Err(why) => Err(format!("Failed to run rsync: {}", why))
			},
			Err(why) => Err(format!("Failed to run rsync: {}", why))
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
			if line1.starts_with("#") || line1.len() == 1 {
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
			} else if let Some(path) = line.strip_prefix("CDST=") {
				task.compare_dest.push(PathBuf::from(path));
			} else if let Some(path) = line.strip_prefix("LDST=") {
				task.link_dest.push(PathBuf::from(path));
			} else if let Some(name) = line.strip_prefix("ID=") {
				task.id = name.to_string();
			} else {
				match line {
					"[EXCLUDE OTHERS]" => { task.exclude_others = true; },
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
		if task.compare_paths && task.backup_path.is_none() {
			return Err(String::from("[COMPARE BPATH] specified but no backup path given."));
		}
		match task.src {
			Some(ref path) => {
				if !path.exists() {
					return Err(format!("Source path {} nonexistent or inaccessible.", path.to_string()));
				}
			},
			None => {
				return Err(String::from("No source path specified."));
			}
		};
		match task.dst {
			Some(ref path) => {
				if !path.exists() {
					return Err(format!("Destination path {} nonexistent or inaccessible.", path.to_string()));
				}
			},
			None => {
				return Err(String::from("No destination path specified."));
			}
		};
		Ok(task)
	}
}
