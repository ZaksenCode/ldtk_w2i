use ldtk_w2i::app::LDtkApp;

fn main() {
  let native_options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_title("LDTK World 2 Image")
      .with_inner_size([1280.0, 720.0])
      .with_drag_and_drop(true),
    ..Default::default()
  };
  let _ = eframe::run_native(
    "LDTK World 2 Image",
    native_options,
    Box::new(|cc| Ok(Box::new(LDtkApp::new(cc)))),
  );
}