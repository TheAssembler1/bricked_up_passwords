use std::collections::HashMap;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    // NOTE: holds mode of running app
    mode: AppModes,
    slots: HashMap<String, Slot>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Slot {
    title: String,
    username: String,
    email: String,
    password: String,
    description: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum EditModes {
    New(Slot),
    Prev(Slot),
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum AppModes {
    View,
    Edit(EditModes),
}

impl Default for Slot {
    fn default() -> Self {
        Self {
            title: String::from("test title"),
            username: String::new(),
            email: String::new(),
            password: String::new(),
            description: String::new(),
        }
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            mode: AppModes::View,
            slots: HashMap::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Create New Slot").clicked() {
                        self.mode = AppModes::Edit(EditModes::New(Slot::default()));
                        ui.close_menu();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.slots.insert(
                            String::from(format!("{}", self.slots.len())),
                            Slot::default(),
                        );
                        ui.close_menu();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Slots");

            let mut index = 1;
            for item in self.slots.iter() {
                index += 1;

                ui.horizontal(|ui| {
                    ui.label(format!("{}", index));
                    if ui.button(format!("{}", item.1.title)).clicked() {
                        self.mode = AppModes::Edit(EditModes::Prev(item.1.to_owned()));
                    }
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                AppModes::View => (),
                AppModes::Edit(ref mut edit_mode) => {
                    match edit_mode {
                        // FIXME: dry
                        EditModes::New(ref mut slot) => {
                            ui.label("Title");
                            ui.text_edit_singleline(&mut slot.title);
                            ui.label("Username");
                            ui.text_edit_singleline(&mut slot.username);
                            ui.label("Email");
                            ui.text_edit_singleline(&mut slot.email);
                            ui.label("Password");
                            ui.text_edit_singleline(&mut slot.password);
                            ui.label("Description");
                            ui.text_edit_multiline(&mut slot.description);
                        }
                        EditModes::Prev(ref mut slot) => {
                            ui.label("Title");
                            ui.text_edit_singleline(&mut slot.title);
                            ui.label("Username");
                            ui.text_edit_singleline(&mut slot.username);
                            ui.label("Email");
                            ui.text_edit_singleline(&mut slot.email);
                            ui.label("Password");
                            ui.text_edit_singleline(&mut slot.password);
                            ui.label("Description");
                            ui.text_edit_multiline(&mut slot.description);
                        }
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.mode = AppModes::View;
                        } else if ui.button("Save Slot").clicked() {
                            self.slots
                                .insert(format!("{}", self.slots.len()), Slot::default());
                            self.mode = AppModes::View;
                        }
                    });
                }
            }

            egui::warn_if_debug_build(ui);
        });
    }
}
