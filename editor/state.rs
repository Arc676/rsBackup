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
use std::vec::Vec;
use std::slice::IterMut;

use crate::TaskConfig;

pub struct State {
	pub filename: ImString,
	tasks: Vec<TaskConfig>
}

impl Default for State {
	fn default() -> Self {
		State {
			filename: ImString::with_capacity(100),
			tasks: Vec::new()
		}
	}
}

impl State {
	pub fn tasks_iter(&mut self) -> IterMut<TaskConfig> {
		self.tasks.iter_mut()
	}

	pub fn add_task(&mut self, cfg: TaskConfig) {
		self.tasks.push(cfg);
	}
}
