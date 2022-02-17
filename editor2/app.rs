// Copyright (C) 2022 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>
// Based on public eframe template https://github.com/emilk/eframe_template

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3).

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use eframe::{egui, epi};
use eframe::egui::{Separator, Ui, WidgetText};

use crate::TaskConfig;

enum TaskButtons {
    NoButton,
    RemoveTask,
    EditTask,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct ConfigEditor {
    filename: String,

    #[cfg_attr(feature = "persistence", serde(skip))]
    editing: TaskConfig,

    #[cfg_attr(feature = "persistence", serde(skip))]
    tasks: Vec<TaskConfig>
}

impl Default for ConfigEditor {
    fn default() -> Self {
        Self {
            filename: String::new(),
            editing: TaskConfig::default(),
            tasks: Vec::new()
        }
    }
}

impl ConfigEditor {
    fn save_edited_task(&mut self) {
        self.tasks.push(std::mem::take(&mut self.editing));
    }

    fn remove_task_at(&mut self, idx: usize) {
        self.tasks.remove(idx);
    }

    fn edit_task_at(&mut self, idx: usize) {
        self.editing = self.tasks.remove(idx);
    }
}

fn show_task(ui: &mut Ui, cfg: &TaskConfig) -> TaskButtons {
    let mut ret = TaskButtons::NoButton;
    if cfg.is_update {
        ui.label("Update task");
    } else {
        ui.label("Backup task");
    }
    if cfg.always_confirm {
        ui.label("Always asks for confirmation");
    }

    ui.label(format!("Source: {}", cfg.src));
    ui.label(format!("Destination: {}", cfg.dst));

    if !cfg.is_update {
        ui.label(format!("Backups: {}", cfg.backup_path));
        if cfg.compare_paths {
            ui.label("Compares with all other backups");
        }
    }

    ui.label("Links:");
    for path in &cfg.link_dest {
        ui.label(format!("- {}", &path));
    }

    ui.label("Compared paths:");
    for path in &cfg.compare_dest {
        ui.label(format!("- {}", &path));
    }

    if !cfg.exclude_from.is_empty() {
        ui.label(format!("Exclude patterns: {}", cfg.exclude_from));
    }
    if !cfg.include_from.is_empty() {
        ui.label(format!("Include patterns: {}", cfg.include_from));
    }
    if !cfg.files_from.is_empty() {
        ui.label(format!("Filename patterns: {}", cfg.files_from));
    }

    ui.horizontal(|ui| {
        if ui.button("Edit task").clicked() {
            ret = TaskButtons::EditTask;
        }
        if ui.button("Remove task").clicked() {
            ret = TaskButtons::RemoveTask;
        }
    });
    ret
}

fn path_list_builder(ui: &mut Ui, label: impl Into<WidgetText>, paths: &mut Vec<String>) {
    let mut to_remove = None;
    ui.label(label);
    for (idx, path) in paths.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(path);
            if ui.button("Remove").clicked() {
                to_remove = Some(idx);
            }
        });
    }
    if let Some(idx) = to_remove {
        paths.remove(idx);
    }
    if ui.button("Add path").clicked() {
        paths.push(String::new());
    }
}

fn task_editor(ui: &mut Ui, cfg: &mut TaskConfig) -> bool {
    // Task type

    ui.horizontal(|ui| {
        ui.label("Task ID:");
        ui.text_edit_singleline(&mut cfg.id);
    });

    ui.checkbox(&mut cfg.always_confirm, "Always ask for confirmation");

    ui.horizontal(|ui| {
        ui.label("Source path:");
        ui.text_edit_singleline(&mut cfg.src);
    });
    ui.horizontal(|ui| {
        ui.label("Destination path:");
        ui.text_edit_singleline(&mut cfg.dst);
    });

    ui.horizontal(|ui| {
        ui.label("Backup path:");
        ui.text_edit_singleline(&mut cfg.backup_path);
    });
    ui.checkbox(&mut cfg.compare_paths, "Compare with old backups");

    path_list_builder(ui, "Linked destinations", &mut cfg.link_dest);
    path_list_builder(ui, "Compared destinations", &mut cfg.compare_dest);

    ui.horizontal(|ui| {
        ui.label("Include from:");
        ui.text_edit_singleline(&mut cfg.include_from);
    });
    ui.horizontal(|ui| {
        ui.label("Exclude from:");
        ui.text_edit_singleline(&mut cfg.exclude_from);
    });
    ui.horizontal(|ui| {
        ui.label("Files from:");
        ui.text_edit_singleline(&mut cfg.files_from);
    });

    ui.button("Save Task").clicked()
}

impl epi::App for ConfigEditor {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Saved Tasks");
            if self.tasks.len() == 0 {
                ui.label("No tasks yet");
            } else {
                let mut action = None;
                for (i, task) in self.tasks.iter().enumerate() {
                    ui.collapsing(&task.id, |ui| {
                        match show_task(ui, task) {
                            TaskButtons::NoButton => (),
                            res => action = Some((i, res))
                        }
                    });
                }
                if let Some((idx, act)) = action {
                    match act {
                        TaskButtons::RemoveTask => self.remove_task_at(idx),
                        _ => self.edit_task_at(idx)
                    }
                }
            }

            let sep = Separator::default().spacing(12.).horizontal();
            ui.add(sep);

            ui.heading("Disk");

            ui.horizontal(|ui| {
                ui.label("Filename:");
                ui.text_edit_singleline(&mut self.filename);
            });
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    // load config
                }
                if ui.button("Save").clicked() {
                    // save config
                }
            });
            if ui.button("Exit").clicked() {
                frame.quit();
            }

            // egui/eframe attribution links
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Editor");
            if task_editor(ui, &mut self.editing) {
                self.save_edited_task();
            }
        });
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "rsBackup Configuration Editor"
    }
}
