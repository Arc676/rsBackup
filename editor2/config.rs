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
        }
    }
}
