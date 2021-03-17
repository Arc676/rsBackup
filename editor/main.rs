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

fn main() {
	let system = support::init(file!());
	let window_title = if cfg!(all(feature = "directx", windows)) {
		im_str!("Hello world (OpenGL)")
	} else {
		im_str!("Hello world (DirectX)")
	};

	system.main_loop(move |_, ui| {
		Window::new(im_str!("rsBackup Configuration Editor"))
			.size([300.0, 110.0], Condition::FirstUseEver)
			.build(ui, || {
				ui.text(im_str!("Hello world!"));
				ui.separator();
				let mouse_pos = ui.io().mouse_pos;
				ui.text(format!(
					"Mouse Position: ({:.1},{:.1})",
					mouse_pos[0], mouse_pos[1]
				));
			});
	});
}
