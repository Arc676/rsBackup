# rsBackup

A backup utility written in Rust. Runs `rsync` with flags based on information given in configuration files.

## Graphical Editors

While `rsbackup` is a command line program, the repository also includes two GUI applications for editing configuration files for use with the command line utility. One uses `imgui-rs`, which provides Rust bindings for Dear Imgui, whereas the other uses `egui` and `eframe` and is pure Rust.

# License

Project available under GPLv3. See `LICENSE` for the full license text.

- Dear ImGui editor: The Rust bindings are provided by [`imgui-rs`](https://github.com/imgui-rs/imgui-rs), available under Apache 2.0 or MIT. In this project, files copied from the `imgui-rs` repository are marked with an MIT license notice. Visit that repository for the full MIT license text.
- egui editor: [`egui`](https://github.com/emilk/egui) is available under Apache 2.0 or MIT. Some code for this project was taken from the [public `eframe` template](https://github.com/emilk/eframe_template/). No license is provided here (assuming free to use for whatever purpose, given that it's a public template).
