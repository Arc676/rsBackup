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

use imgui::*;

mod support;
mod config;
mod state;

use config::TaskConfig;
use state::State;

fn task_editor(ui: &Ui, cfg: &mut TaskConfig) -> bool {
	ui.button(im_str!("Save Task"), [0.0,0.0])
}

enum TaskButtons {
	NoButton,
	RemoveTask,
	EditTask
}

fn show_task(ui: &Ui, cfg: &TaskConfig) -> TaskButtons {
	TaskButtons::NoButton
}

fn build_window(ui: &Ui, state: &mut State) {
	let mut editing = TaskConfig::default();
	Window::new(im_str!("rsBackup Configuration Editor"))
		.size([300.0, 110.0], Condition::FirstUseEver)
		.build(&ui, || {
			// Editor
			if task_editor(ui, &mut editing) {
				state.add_task(editing);
				editing = TaskConfig::default();
			}

			// Show existing tasks
			for task in state.tasks_iter() {
				match show_task(ui, task) {
					_ => {}
				};
			}

			// Disk I/O
			ui.text(im_str!("Filename:"));
			ui.same_line(0.0);
			ui.input_text(im_str!("##Filename"), &mut state.filename)
				.build();
			if ui.button(im_str!("Load"), [0.0,0.0]) {
			}
			ui.same_line(0.0);
			if ui.button(im_str!("Save"), [0.0,0.0]) {
			}
		});
}

fn main() {
	let mut state = State::default();
	let system = support::init(file!());
	system.main_loop(move |_, ui| build_window(ui, &mut state));
}
