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

use imgui::ImString;

pub struct TaskConfig {
	pub id: ImString,
	pub is_update: bool,
	pub always_confirm: bool,
	pub src: ImString,
	pub dst: ImString,
	pub backup_path: ImString,
	pub compare_paths: bool,
	pub link_dest: Vec<ImString>,
	pub compare_dest: Vec<ImString>,
	pub exclude_from: ImString,
	pub include_from: ImString,
	pub files_from: ImString,
}

impl Default for TaskConfig {
	fn default() -> Self {
		TaskConfig {
			id: ImString::with_capacity(255),
			is_update: true, always_confirm: false,
			src: ImString::with_capacity(255), dst: ImString::with_capacity(255),
			backup_path: ImString::with_capacity(255),
			compare_paths: false,
			link_dest: Vec::new(), compare_dest: Vec::new(),
			exclude_from: ImString::with_capacity(255),
			include_from: ImString::with_capacity(255),
			files_from: ImString::with_capacity(255)
		}
	}
}
