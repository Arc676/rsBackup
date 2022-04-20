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

use std::fs::{self, File};
use std::io;
use std::io::{BufReader, Error, ErrorKind, Write};
use eframe::{egui, epi};
use eframe::egui::{Separator, Ui, WidgetText};

use crate::TaskConfig;

enum TaskButtons {
    RemoveTask,
    EditTask,
}

enum IOState {
    FileNotFound,
    IOError(String),
    InvalidTask(String),
    ConfigWritten,
    ConfigRead
}

#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct ConfigEditor {
    filename: String,

    #[cfg_attr(feature = "persistence", serde(skip))]
    io_state: Option<IOState>,

    #[cfg_attr(feature = "persistence", serde(skip))]
    editing: TaskConfig,

    #[cfg_attr(feature = "persistence", serde(skip))]
    tasks: Vec<TaskConfig>
}

macro_rules! labeled_field {
    ($ui:ident, $lbl:tt, $target:expr) => {
        $ui.horizontal(|$ui| {
            $ui.label($lbl);
            $ui.text_edit_singleline($target);
        });
    }
}

macro_rules! labeled_editor_field {
    ($ui:ident, $lbl:tt, $target:expr, $editing:expr) => {
        labeled_field!($ui, $lbl, $target);
        if $target.len() > 0 {
            if $editing.is_none() && $ui.button("Edit file contents").clicked() {
                $editing.replace(match fs::read_to_string($target) {
                    Ok(contents) => contents,
                    Err(e) => format!("Failed to read file: {}", e)
                });
            } else if $editing.is_some() {
                $ui.text_edit_multiline($editing.as_mut().unwrap());
                if $ui.button("Save file contents").clicked() {
                    match fs::write($target, $editing.as_ref().unwrap() as &str) {
                        Ok(_) => $editing.take(),
                        Err(e) => panic!("Failed to write: {}", e)
                    };
                }
            }
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

    fn save_to_disk(&self) -> io::Result<()> {
        for (i, task) in self.tasks.iter().enumerate() {
            if let Err(e) = task.validate() {
                return Err(Error::new(ErrorKind::InvalidData,
                                      format!("{}: {}",
                                              if task.id.is_empty() {
                                                  format!("(Task #{})", i + 1)
                                              } else {
                                                  task.id.clone()
                                              },
                                              e)));
            }
        }
        let mut file = File::create(&self.filename)?;
        for task in &self.tasks {
            file.write(task.to_string().as_ref())?;
        }
        Ok(())
    }

    fn load_from_disk(&mut self) -> io::Result<()> {
        let mut new_tasks = Vec::new();
        let file = File::open(&self.filename)?;
        let mut reader = BufReader::new(file);
        loop {
            match TaskConfig::from_reader(&mut reader) {
                Ok(task) => new_tasks.push(task),
                Err(err) => match err.as_str() {
                    "EOF" => break,
                    err => return Err(Error::new(ErrorKind::Other, err))
                }
            }
        }
        self.tasks = new_tasks;
        Ok(())
    }
}

fn show_task(ui: &mut Ui, cfg: &TaskConfig) -> Option<TaskButtons> {
    let mut ret = None;
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

    if cfg.exclude_others {
        ui.label("Ignores all unincluded files");
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
            ret = Some(TaskButtons::EditTask);
        }
        if ui.button("Remove task").clicked() {
            ret = Some(TaskButtons::RemoveTask);
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
    ui.horizontal(|ui| {
        ui.radio_value(&mut cfg.is_update, true, "Update task");
        ui.radio_value(&mut cfg.is_update, false, "Backup task");
    });

    labeled_field!(ui, "Task ID:", &mut cfg.id);

    ui.checkbox(&mut cfg.always_confirm, "Always ask for confirmation");

    labeled_field!(ui, "Source path:", &mut cfg.src);
    labeled_field!(ui, "Destination path:", &mut cfg.dst);

    labeled_field!(ui, "Backup path:", &mut cfg.backup_path);
    ui.checkbox(&mut cfg.compare_paths, "Compare with old backups");

    ui.checkbox(&mut cfg.exclude_others, "Exclude all unincluded files");

    path_list_builder(ui, "Linked destinations", &mut cfg.link_dest);
    path_list_builder(ui, "Compared destinations", &mut cfg.compare_dest);

    labeled_editor_field!(ui, "Include from:", &mut cfg.include_from, cfg.editing_include);
    labeled_editor_field!(ui, "Exclude from:", &mut cfg.exclude_from, cfg.editing_exclude);
    labeled_editor_field!(ui, "Files from:", &mut cfg.files_from, cfg.editing_files);

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
                    let placeholder = format!("(Task #{})", i + 1);
                    let header = if task.id.is_empty() {
                        &placeholder
                    } else {
                        &task.id
                    };
                    ui.collapsing(header, |ui| {
                        match show_task(ui, task) {
                            Some(act) => action = Some((i, act)),
                            None => ()
                        }
                    });
                }
                if let Some((idx, act)) = action {
                    match act {
                        TaskButtons::RemoveTask => self.remove_task_at(idx),
                        TaskButtons::EditTask => self.edit_task_at(idx)
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
                    match self.load_from_disk() {
                        Ok(_) => self.io_state = Some(IOState::ConfigRead),
                        Err(e) => self.io_state = match e.kind() {
                            ErrorKind::NotFound => Some(IOState::FileNotFound),
                            _ => Some(IOState::IOError(e.to_string()))
                        }
                    }
                }
                if ui.button("Save").clicked() {
                    match self.save_to_disk() {
                        Ok(_) => self.io_state = Some(IOState::ConfigWritten),
                        Err(e) => self.io_state = match e.kind() {
                            ErrorKind::InvalidData => Some(IOState::InvalidTask(e.to_string())),
                            _ => Some(IOState::IOError(e.to_string()))
                        }
                    }
                }
            });
            if let Some(state) = &self.io_state {
                match state {
                    IOState::FileNotFound => ui.label("Err: File not found"),
                    IOState::IOError(err) => ui.label(format!("Err: {}", err)),
                    IOState::ConfigWritten => ui.label("Saved config file"),
                    IOState::ConfigRead => ui.label("Read config file"),
                    IOState::InvalidTask(e) => ui.label(format!(
                        "Error in {}", e))
                };
            }
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
