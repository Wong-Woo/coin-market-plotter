use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "eframe example",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    name: String,
    age: u32,
    greeted: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe example");

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Age"));

            if ui.button("Greet me").clicked() {
                self.greeted = true;
            }

            if self.greeted {
                ui.label(format!("Hello {}, age {}", self.name, self.age));
            }
        });
    }
}
