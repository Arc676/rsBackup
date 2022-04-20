// Copyright (C) 2022 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3).

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::fmt::{Display, Formatter};
use std::io::BufRead;

pub struct TaskConfig {
    pub id: String,
    pub is_update: bool,
    pub always_confirm: bool,
    pub src: String,
    pub dst: String,
    pub backup_path: String,
    pub compare_paths: bool,
    pub link_dest: Vec<String>,
    pub compare_dest: Vec<String>,
    pub exclude_from: String,
    pub include_from: String,
    pub files_from: String,
    pub exclude_others: bool,

    pub editing_include: Option<String>,
    pub editing_exclude: Option<String>,
    pub editing_files: Option<String>,
}

macro_rules! write_if_nonempty {
    ($f:ident, $label:tt, $parameter:expr) => {
        if !$parameter.is_empty() {
            writeln!($f, "{}={}", $label, $parameter)?;
        }
    };
}

macro_rules! write_if_set {
    ($f:ident, $indicator:tt, $parameter:expr) => {
        if $parameter {
            writeln!($f, $indicator)?;
        }
    };
}

impl Display for TaskConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\nSRC={}\nDST={}",
               match self.is_update {
                   true => "[UPDATE]",
                   false => "[BACKUP]"
               },
               self.src, self.dst
        )?;
        write_if_nonempty!(f, "ID", self.id);
        write_if_nonempty!(f, "EXFR", self.exclude_from);
        write_if_nonempty!(f, "INFR", self.include_from);
        write_if_nonempty!(f, "FIFR", self.files_from);
        write_if_nonempty!(f, "BPATH", self.backup_path);
        for path in &self.compare_dest {
            writeln!(f, "CDST={}", path)?;
        }
        for path in &self.link_dest {
            writeln!(f, "LDST={}", path)?;
        }
        write_if_set!(f, "[EXCLUDE OTHERS]", self.exclude_others);
        write_if_set!(f, "[CONFIRM]", self.always_confirm);
        write_if_set!(f, "[COMPARE BPATH]", self.compare_paths);
        writeln!(f, "[END]")
    }
}

impl Default for TaskConfig {
    fn default() -> Self {
        TaskConfig {
            id: String::new(),
            is_update: true,
            always_confirm: false,
            src: String::new(),
            dst: String::new(),
            backup_path: String::new(),
            compare_paths: false,
            link_dest: Vec::new(),
            compare_dest: Vec::new(),
            exclude_from: String::new(),
            include_from: String::new(),
            files_from: String::new(),
            exclude_others: false,
            editing_include: None,
            editing_exclude: None,
            editing_files: None,
        }
    }
}

impl TaskConfig {
    pub fn from_reader(reader: &mut impl BufRead) -> Result<Self, String> {
        let mut task = TaskConfig::default();
        let mut type_determined = false;
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(len) => {
                    if len == 0 {
                        return Err(String::from("EOF"));
                    }
                }
                Err(err) => {
                    return Err(err.to_string());
                }
            }
            if line.starts_with("#") || line.len() == 1 {
                continue;
            }
            let line = line.trim();
            if !type_determined {
                match line {
                    "[BACKUP]" => task.is_update = false,
                    "[UPDATE]" => {}
                    _ => {
                        return Err(String::from(
                            "Failed to parse configuration file. Could not find task.",
                        ));
                    }
                };
                type_determined = true;
                continue;
            }
            if line == "[END]" {
                break;
            }
            if let Some(path) = line.strip_prefix("SRC=") {
                task.src = path.to_string();
            } else if let Some(path) = line.strip_prefix("DST=") {
                task.dst = path.to_string();
            } else if let Some(path) = line.strip_prefix("EXFR=") {
                task.exclude_from = path.to_string();
            } else if let Some(path) = line.strip_prefix("INFR=") {
                task.include_from = path.to_string();
            } else if let Some(path) = line.strip_prefix("FIFR=") {
                task.files_from = path.to_string();
            } else if let Some(path) = line.strip_prefix("BPATH=") {
                if task.is_update {
                    return Err(String::from(
                        "Unexpected BPATH parameter in update task configuration."
                    ));
                } else {
                    task.backup_path = path.to_string();
                }
            } else if let Some(path) = line.strip_prefix("CDST=") {
                task.compare_dest.push(path.to_string());
            } else if let Some(path) = line.strip_prefix("LDST=") {
                task.link_dest.push(path.to_string());
            } else if let Some(name) = line.strip_prefix("ID=") {
                task.id = name.to_string();
            } else {
                match line {
                    "[EXCLUDE OTHERS]" => task.exclude_others = true,
                    "[CONFIRM]" => task.always_confirm = true,
                    "[COMPARE BPATH]" => if task.is_update {
                        return Err(String::from(
                            "Unexpected [COMPARE BPATH] tag in update task configuration."
                        ));
                    } else {
                        task.compare_paths = true;
                    },
                    _ => return Err(format!("Unexpected line '{}' in configuration.", line))
                };
            }
        }
        if !type_determined {
            return Err(String::from("EOF"));
        }
        if task.compare_paths && task.backup_path.is_empty() {
            return Err(String::from(
                "[COMPARE BPATH] specified but no backup path given."
            ));
        }
        Ok(task)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.src.is_empty() {
            return Err("No source path specified");
        }
        if self.dst.is_empty() {
            return Err("No destination path specified");
        }
        if self.is_update {
            if self.compare_paths {
                return Err("Update task can't compare with backups");
            }
            if !self.backup_path.is_empty() {
                return Err("Update task can't have backup path");
            }
        }
        if self.backup_path.is_empty() && self.compare_paths {
            return Err("No backup path to compare to");
        }
        Ok(())
    }
}
