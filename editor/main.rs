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

struct State {
	filename: ImString
}

impl Default for State {
	fn default() -> Self {
		State {
			filename: ImString::with_capacity(100)
		}
	}
}

fn build_window(ui: &Ui, state: &mut State) {
	Window::new(im_str!("rsBackup Configuration Editor"))
		.size([300.0, 110.0], Condition::FirstUseEver)
		.build(&ui, || {
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
