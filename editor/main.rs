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
	// Task type
	ui.radio_button(im_str!("Update task"), &mut cfg.is_update, true);
	ui.same_line(0.0);
	ui.radio_button(im_str!("Backup task"), &mut cfg.is_update, false);

	ui.text("Task ID:");
	ui.same_line(0.0);
	ui.input_text(im_str!("##ID"), &mut cfg.id)
		.build();

	ui.checkbox(im_str!("Always ask for confirmation"), &mut cfg.always_confirm);

	ui.text("Source path:");
	ui.same_line(0.0);
	ui.input_text(im_str!("##SRC"), &mut cfg.src)
		.build();

	ui.text("Destination path:");
	ui.same_line(0.0);
	ui.input_text(im_str!("##DST"), &mut cfg.dst)
		.build();

	ui.text("Backup path:");
	ui.same_line(0.0);
	ui.input_text(im_str!("##BackupPath"), &mut cfg.backup_path)
		.build();
	ui.checkbox(im_str!("Compare with old backups"), &mut cfg.compare_paths);

	let mut rl = None;
	ui.text("Linked destinations");
	for (il, mut path) in cfg.link_dest.iter_mut().enumerate() {
		ui.input_text(&ImString::new(format!("##Link{}", il)), &mut path)
			.build();
		ui.same_line(0.0);
		if ui.button(&ImString::new(format!("Remove##RLink{}", il)), [0.0,0.0]) {
			rl = Some(il);
		}
	}
	if let Some(idx) = rl {
		cfg.link_dest.remove(idx);
	}
	if ui.button(im_str!("Add path##LinkDest"), [0.0,0.0]) {
		cfg.link_dest.push(ImString::with_capacity(255));
	}

	let mut rc = None;
	ui.text("Compared destinations");
	for (ic, mut path) in cfg.compare_dest.iter_mut().enumerate() {
		ui.input_text(&ImString::new(format!("##Comp{}", ic)), &mut path)
			.build();
		ui.same_line(0.0);
		if ui.button(&ImString::new(format!("Remove##RComp{}", ic)), [0.0,0.0]) {
			rc = Some(ic);
		}
	}
	if let Some(idx) = rc {
		cfg.compare_dest.remove(idx);
	}
	if ui.button(im_str!("Add path##CompDest"), [0.0,0.0]) {
		cfg.compare_dest.push(ImString::with_capacity(255));
	}

	ui.text("Include from:");
	ui.same_line(0.0);
	ui.input_text(im_str!("##IncludeFrom"), &mut cfg.include_from)
		.build();

	ui.text("Exclude from:");
	ui.same_line(0.0);
	ui.input_text(im_str!("##ExcludeFrom"), &mut cfg.exclude_from)
		.build();

	ui.text("Files from:");
	ui.same_line(0.0);
	ui.input_text(im_str!("##FilesFrom"), &mut cfg.files_from)
		.build();

	ui.button(im_str!("Save Task"), [0.0,0.0])
}

enum TaskButtons {
	NoButton,
	RemoveTask,
	EditTask
}

fn show_task(ui: &Ui, cfg: &TaskConfig) -> TaskButtons {
	let mut ret = TaskButtons::NoButton;
	if cfg.is_update {
		ui.text("Update task");
	} else {
		ui.text("Backup task");
	}
	if cfg.always_confirm {
		ui.text("Always asks for confirmation");
	}

	ui.text(format!("Source: {}", cfg.src));
	ui.text(format!("Destination: {}", cfg.dst));

	if !cfg.is_update {
		ui.text(format!("Backups: {}", cfg.backup_path));
		if cfg.compare_paths {
			ui.text("Compares with all other backups");
		}
	}

	ui.text("Links:");
	for path in &cfg.link_dest {
		ui.bullet_text(&path);
	}

	ui.text("Compared paths:");
	for path in &cfg.compare_dest {
		ui.bullet_text(&path);
	}

	if !cfg.exclude_from.is_empty() {
		ui.text(format!("Exclude patterns: {}", cfg.exclude_from));
	}
	if !cfg.include_from.is_empty() {
		ui.text(format!("Include patterns: {}", cfg.include_from));
	}
	if !cfg.files_from.is_empty() {
		ui.text(format!("Filename patterns: {}", cfg.files_from));
	}

	if ui.button(im_str!("Edit task"), [0.0,0.0]) {
		ret = TaskButtons::EditTask;
	}
	ui.same_line(0.0);
	if ui.button(im_str!("Remove task"), [0.0,0.0]) {
		ret = TaskButtons::RemoveTask;
	}
	ret
}

fn build_window(ui: &Ui, state: &mut State) {
	Window::new(im_str!("rsBackup Configuration Editor"))
		.size([500.0, 700.0], Condition::FirstUseEver)
		.build(&ui, || {
			// Editor
			if CollapsingHeader::new(im_str!("Task Editor")).build(&ui) {
				if task_editor(ui, &mut state.editing) {
					state.save_edited_task();
				}
			}

			// Show existing tasks
			if CollapsingHeader::new(im_str!("Saved Tasks")).build(&ui) {
				let mut action = None;
				for (i, task) in state.tasks_iter().enumerate() {
					let name = match task.id.is_empty() {
						true => format!("Task {}", i + 1),
						false => format!("{}", task.id)
					};
					TreeNode::new(&ImString::new(name)).build(&ui, || {
						let res = show_task(ui, task);
						match res {
							TaskButtons::NoButton => {},
							_ => { action = Some((i, res)); }
						};
					});
				}
				if let Some((idx, act)) = action {
					match act {
						TaskButtons::RemoveTask => { state.remove_task_at(idx); }
						_ => { state.edit_task_at(idx); }
					};
				}
			}

			// Disk I/O
			ui.text("Filename:");
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
