use std::{fs, io::Error, path::PathBuf};

use egui::Ui;

use crate::generator;
use crate::schemas::LDtk;

#[derive(Default)]
pub struct LDtkApp {
  picked_path: Option<PathBuf>,
  saved_path: Option<PathBuf>,
  error: Option<Error>,
  ldtk_world: Option<LDtk>,
}

impl LDtkApp {
  pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
    Self::default()
  }
}

impl eframe::App for LDtkApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| draw_ui(self, ui));
  }
}

fn draw_ui(app: &mut LDtkApp, ui: &mut Ui) {
  ui.horizontal(|ui| draw_file_choice(app, ui));
  draw_files(app, ui);

  if let Some(error) = &app.error {
    ui.label(format!("An error has occurred: {}", error.to_string()));
  }
}

fn draw_file_choice(app: &mut LDtkApp, ui: &mut Ui) {
  ui.label("Choose file:");
  if ui.button("Open file…").clicked() {
    if let Some(path) = rfd::FileDialog::new()
      .add_filter("json", &["json"])
      .set_directory("/")
      .pick_file()
    {
      app.picked_path = Some(path);
    }
  }
}

fn draw_files(app: &mut LDtkApp, ui: &mut Ui) {
  if let Some(picked_path) = &app.picked_path {
    ui.horizontal(|ui| {
      ui.label("Picked file:");
      ui.monospace(picked_path.display().to_string());
    });
    if ui.button("Save as").clicked() {
      if let Some(path) = rfd::FileDialog::new()
        .add_filter("png", &["png"])
        .set_directory("/")
        .save_file()
      {
        app.saved_path = Some(path)
      }
    }
  }

  // Process image generation and clear old paths.
  if let Some(saved_path) = &app.saved_path {
    let picked_path: &PathBuf;
    match &app.picked_path {
      Some(path) => picked_path = path,
      None => return,
    }
    let ldtk_world_string: String = fs::read_to_string(picked_path).unwrap();
    app.ldtk_world = serde_json::from_str(&ldtk_world_string).unwrap();

    let ldtk_world: &LDtk;
    match &app.ldtk_world {
      Some(world) => ldtk_world = world,
      None => return,
    }

    let img_dir = picked_path.parent().unwrap().join(
      picked_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "/png",
    );
    let image = generator::world_image(ldtk_world, &img_dir);
    let _result = image.save(saved_path);

    app.saved_path = None;
    app.picked_path = None;
    app.ldtk_world = None;
  }
}
