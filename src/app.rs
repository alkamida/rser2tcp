use serialport;


pub struct MyApp {
    // Example stuff:
    label: String,
    tty : u8,
    baud : u32,
    // this how you opt-out of serialization of a member
    value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            tty: 1,
            baud : 115_200,
            value: 2.7,
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
    fn save(&mut self, storage: &mut dyn eframe::Storage) {

    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, tty, baud, value } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if ui.button("Start").clicked() {
                _frame.close();
            }
            if ui.button("Quit").clicked() {
                _frame.close();
            }
            ui.add_space(16.);

            //TODO: Create empty string and label to set label to the left or make PR.
            egui::ComboBox::from_label("Chose UART")
                .selected_text(format!("/dev/ttyUSB{:?}", self.tty))
                .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.tty, 1, "/dev/ttyUSB0");
                ui.selectable_value(&mut self.tty, 2, "/dev/ttyUSB1");
                ui.selectable_value(&mut self.tty, 3, "/dev/ttyUSB2");
                }
                );

            ui.add_space(16.);
            

            egui::warn_if_debug_build(ui);

            //println!("Selected: {}",self.tty);
        });
    }
}
