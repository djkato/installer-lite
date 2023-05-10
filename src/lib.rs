use eframe::{egui, App};
use rfd::FileDialog;

#[cfg(target_os = "windows")]
#[derive(Default)]
pub struct Installer {
    executable: &'static [u8],
    target_path: String,
    app_name: String,
    installer_output: String,
    post_install_function: Option<Box<dyn Fn() -> String>>,
    pre_install_function: Option<Box<dyn Fn() -> String>>,
    is_phase_done: bool,
    is_phase_started: bool,
    current_phase: InstallPhase,
}
#[derive(Default)]
enum InstallPhase {
    #[default]
    Start,
    Installation,
    Error(Box<dyn std::error::Error>),
    Success,
    Finish,
}
impl InstallPhase {
    pub fn next(&mut self) {
        *self = match self {
            Self::Start => Self::Installation,
            Self::Installation => Self::Success,
            Self::Success => Self::Finish,
            Self::Error(_) => Self::Finish,
            Self::Finish => panic!("Nothing past finish"),
        };
    }
}
impl App for Installer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("{}, Installer", &self.app_name));
            ui.separator();
            ui.allocate_ui(ui.available_size() / egui::Vec2::new(1.1, 1.1), |ui| {
                use InstallPhase::*;
                self.is_phase_done = true;
                match &mut self.current_phase {
                    Start => self.start_ui(ui),
                    Installation => {
                        if !self.is_phase_started {
                            if let Some(pre_install_function) = self.pre_install_function.take() {
                                self.installer_output += &pre_install_function();
                            };

                            if let Err(e) = self.install() {
                                self.current_phase = InstallPhase::Error(e);
                            };

                            if let Some(post_install_function) = self.post_install_function.take() {
                                self.installer_output += &post_install_function()
                            };

                            self.is_phase_started = true;
                            self.is_phase_done = true;
                        }
                        self.installation_ui(ui);
                    }
                    Success => self.success_ui(ui),
                    Error(_) => self.error_ui(ui),
                    Finish => frame.close(),
                }
                ui.add_space(ui.available_height());
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    self.current_phase = InstallPhase::Finish;
                }
                ui.add_space(ui.available_width() - 41.0); //41 is finish button widh
                ui.add_enabled_ui(self.is_phase_done, |ui| {
                    let proceed_button_text = match self.current_phase {
                        InstallPhase::Start => "Install",
                        InstallPhase::Installation => "Next",
                        _ => "Finish",
                    };
                    let button = ui.button(proceed_button_text);
                    if button.clicked() {
                        self.current_phase.next();
                    }
                });
            });
        });
    }
}
/* Main Functions */
impl Installer {
    fn install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let app_path = self.target_path.clone() + r"\" + &self.app_name + ".exe";
        if let Ok(_) = std::fs::read(&app_path) {
            return Err("App already installed".into());
        };
        std::fs::create_dir_all(&self.target_path)?;
        self.installer_output += format!("\nCreating App folder: \n{}", &self.target_path).as_str();
        std::fs::write(&app_path, &self.executable)?;
        self.installer_output += "\nCopying executable to folder";
        Ok(())
    }
}

/* UIs */
impl Installer {
    fn start_ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(ui.available_height() / 2.1);

        ui.vertical(|ui| {
            ui.label("Installation directory: ");
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.target_path).desired_width(f32::INFINITY),
                );
                if ui.button("...").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.target_path = path.to_string_lossy().to_string()
                    }
                }
            });
        });
    }
    fn installation_ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .max_width(f32::INFINITY)
            .show(ui, |ui| {
                ui.allocate_ui(ui.available_size(), |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.installer_output)
                            .desired_width(f32::INFINITY)
                            .desired_rows(100),
                    )
                });
            });
    }
    fn success_ui(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Program Installed, click \"Finish\" to close the installer.");
        });
    }
    fn error_ui(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui|{
            ui.vertical(|ui|{
                ui.label("Something went wrong during the installation... Click \"Finish\" to close the installer.");
                let error = match &self.current_phase{
                    InstallPhase::Error(e) => e.to_string(),
                    _ => "".to_string()
                };
                ui.label(error);
            });
        });
    }
}

/* Public functions */
impl Installer {
    pub fn start(self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(420.0, 420.0)),
            ..Default::default()
        };
        eframe::run_native(
            format!("{} Installer", self.app_name).as_str(),
            options,
            Box::new(|_cc| Box::new(self)),
        )
    }

    pub fn new(executable: &'static [u8], target_path: Option<String>, app_name: String) -> Self {
        Installer {
            executable,
            target_path: target_path.unwrap_or(r"C:\Program Files (x86)".to_string()),
            app_name,
            current_phase: InstallPhase::Start,
            installer_output: "".to_owned(),
            post_install_function: None,
            pre_install_function: None,
            is_phase_done: false,
            is_phase_started: false,
        }
    }

    pub fn add_pre_install_function(&mut self, f: Box<dyn Fn() -> String>) {
        self.pre_install_function = Some(f)
    }
    pub fn add_post_install_function(&mut self, f: Box<dyn Fn() -> String>) {
        self.post_install_function = Some(f)
    }
}
