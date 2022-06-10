use egui_extras::RetainedImage;

pub struct TemplateApp {}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {}
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let image_path = include_bytes!("/home/ragnyll/dev/blog/assets/hacker_smaller.png");
        let retained_image = RetainedImage::from_image_bytes("profile picture", image_path).unwrap();

        egui::SidePanel::left("intro_panel")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(16.00);
                        ui.heading("Jake Gallow");
                        ui.add_space(4.00);
                    });

                    ui.separator();
                    retained_image.show(ui);
                    ui.separator();
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.00);
                        ui.label("Software Engineer by day");
                        ui.label("Bboy by night");
                        ui.add_space(10.00);
                        ui.separator();

                        ui.add_space(12.00);
                        ui.hyperlink_to("LinkedIn", "https://www.linkedin.com/in/jakegallow/");
                        ui.add_space(2.00);
                        ui.hyperlink_to("Github", "https://github.com/ragnyll");
                        ui.add_space(2.00);
                        ui.hyperlink_to("Blog", "https://blog.gallowzhumour.dev");
                    });
                });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(16.00);
                ui.heading("Curriculum Vitae");
                ui.add_space(4.00);
                ui.separator();


            });
        });
    }
}
