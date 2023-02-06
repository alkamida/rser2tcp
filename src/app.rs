use serialport;

pub struct MyApp {
    // Example stuff:
    label: String,
    tty: u8,
    baud: u32,
    data: u32,
    parity: String,
    stop: String,
    ip: String,
    port: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            tty: 1,
            baud: 115_200,
            data: 8,
            parity: "None".to_string(),
            stop: "1".to_string(),
            ip: "127.0.0.1".to_string(),
            port: "9990".to_string(),
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MyApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            tty,
            baud,
            data,
            parity,
            stop,
            ip,
            port
        } = self;
        let baud_rate = [
            110, 300, 600, 1200, 2400, 4800, 9600, 14400, 19200, 38400, 57600, 115200, 128000,
            256000,
        ];
        let data_bits = [5, 6, 7, 8];
        let parity = ["None", "Odd", "Even", "Mark", "Space"];
        let stop_bits = ["1", "1.5", "2"];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Serial2TCP");
            ui.separator();
            
            egui::Grid::new("serial_grid")
                .num_columns(4)
                .spacing([20.0,20.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("UART:");
                    egui::ComboBox::from_id_source("Choose UART")
                        .selected_text(format!("/dev/ttyUSB{:?}", self.tty))
                        .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.tty, 1, "/dev/ttyUSB0");
                        ui.selectable_value(&mut self.tty, 2, "/dev/ttyUSB1");
                        ui.selectable_value(&mut self.tty, 3, "/dev/ttyUSB2");
                    });
                    ui.label("IP:");
                    ui.add(egui::TextEdit::singleline(& mut self.ip).hint_text("127.0.0.1"));
                    ui.end_row();
                    ui.label("Baud:");
                    egui::ComboBox::from_id_source("Baud rate")
                        .selected_text(self.baud.to_string())
                        .show_ui(ui, |ui| {
                        for b in baud_rate {
                            ui.selectable_value(&mut self.baud, b, b.to_string());
                        }
                    });
                    ui.label("Port:");
                    ui.add(egui::TextEdit::singleline(& mut self.port).hint_text("9990"));
                    ui.end_row();
                    ui.label("Data bits:");
                    egui::ComboBox::from_id_source("Data bits")
                        .selected_text(self.data.to_string())
                        .show_ui(ui, |ui| {
                        for d in data_bits {
                            ui.selectable_value(&mut self.data, d, d.to_string());
                        }
                    });
                    ui.end_row();
                    ui.label("Parity:");
                    egui::ComboBox::from_id_source("Parity")
                        .selected_text(&self.parity)
                        .show_ui(ui, |ui| {
                    for p in parity {
                        ui.selectable_value(&mut self.parity, p.to_string(), p.to_string());
                    }
                });
                    ui.end_row();
                    ui.label("Stop bits:");
                    egui::ComboBox::from_id_source("Stop bits")
                        .selected_text(&self.stop)
                        .show_ui(ui, |ui| {
                            for s in stop_bits {
                                ui.selectable_value(&mut self.stop, s.to_string(), s.to_string());
                            }
                     });
                    ui.end_row();
                });
            ui.separator();
            ui.horizontal(|ui| {


            if ui.button("Start").clicked() {
                _frame.close();
            }
            if ui.button("Stop").clicked() {
                _frame.close();
            }
            if ui.button("Quit").clicked() {
                _frame.close();
            }
            });

            ui.separator();

            ui.label(format!(
                "/dev/ttyUSB{} | {} | {}{}{} | {}:{}",
                self.tty, self.baud, self.data, self.parity, self.stop, self.ip, self.port));
            ui.separator();
            egui::warn_if_debug_build(ui);
        });
    }
}
