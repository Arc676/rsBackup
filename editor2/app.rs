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

use crate::TaskConfig;

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

impl epi::App for ConfigEditor {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Saved Tasks");
            if self.tasks.len() == 0 {
                ui.label("No tasks yet");
            }

            ui.heading("Disk");

            ui.horizontal(|ui| {
                ui.label("Filename: ");
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
