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
