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

use std::fmt::{Display, Error, Formatter};

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
}

macro_rules! write_if_nonempty {
    ($f:ident, $label:tt, $parameter:expr) => {
        if !$parameter.is_empty() {
            writeln!($f, "$label={}", $parameter)?;
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
        write!(f, "{}\nID={}\nSRC={}\nDST={}",
               match self.is_update {
                   true => "[UPDATE]",
                   false => "[BACKUP]"
               },
               self.id, self.src, self.dst
        )?;
        write_if_nonempty!(f, "EXFR", self.exclude_from);
        write_if_nonempty!(f, "INFR", self.include_from);
        write_if_nonempty!(f, "FIFR", self.files_from);
        if self.backup_path.is_empty() && self.compare_paths {
            return Err(Error::default());
        }
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
        write!(f, "[END]")
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
            exclude_others: false
        }
    }
}
