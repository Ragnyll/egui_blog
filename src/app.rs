pub struct TemplateApp {
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self { }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

fn render_home(ui: &mut egui::Ui) {
}

fn render_blog(ui: &mut egui::Ui) {
}

fn render_resume(ui: &mut egui::Ui) {
}

fn show_menu(ui: &mut egui::Ui) {
    use egui::menu;

    menu::bar(ui, |ui| {
        if ui.button("Home").clicked() {
            println!("ay fuck you man");
        }
        if ui.button("Blog").clicked() {
            println!("Gotta get that siracha shrimp");
        }
        if ui.button("Resume").clicked() {
            println!("PINK GUY");
        }
    });
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                show_menu(ui)
            });
        });
    }
}
