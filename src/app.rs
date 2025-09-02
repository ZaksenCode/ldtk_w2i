use std::error;
use std::{fs, path::PathBuf};

use egui::Ui;

use crate::generator;
use crate::schemas::LDtk;

#[derive(Default)]
pub struct LDtkApp {
  picked_path: Option<PathBuf>,
  saved_path: Option<PathBuf>,
  error: Option<Box<dyn error::Error>>,
  ldtk_world: Option<LDtk>,
  // Values
  world_depth_text: String,
  // Settings
  pub world_depth: i16,
}

impl LDtkApp {
  pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
    Self { 
      world_depth: 0,
      ..Default::default()
    }
  }
}

impl eframe::App for LDtkApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| draw_top_panel(self, ui));
    egui::CentralPanel::default().show(ctx, |ui| draw_ui(self, ui));
  }
}

fn draw_top_panel(_app: &mut LDtkApp, ui: &mut Ui) {
  ui.label("LDTK World 2 Image");
}

fn draw_ui(app: &mut LDtkApp, ui: &mut Ui) {
  ui.horizontal(|ui| {
    ui.label("World depth: ");
    match draw_depth_choice(app, ui) {
      Err(error) => app.error = Some(error),
      _ => {}
    }
  });
  ui.horizontal(|ui| draw_file_choice(app, ui));
  match draw_files(app, ui) {
    Err(error) => app.error = Some(error),
    _ => {}
  }

  if let Some(error) = &app.error {
    ui.label(format!("An error has occurred: {}", error.to_string()));
  }
}

fn draw_depth_choice(app: &mut LDtkApp, ui: &mut Ui) -> Result<(), Box<dyn error::Error>> {
  if ui.text_edit_singleline(&mut app.world_depth_text).changed() {
    app.world_depth = app.world_depth_text.parse()?;
  }
  Ok(())
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

fn draw_files(app: &mut LDtkApp, ui: &mut Ui) -> Result<(), Box<dyn error::Error>> {
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
      None => return Ok(()),
    }
    let ldtk_world_string: String = fs::read_to_string(picked_path)?;
    app.ldtk_world = serde_json::from_str(&ldtk_world_string)?;

    let ldtk_world: &LDtk;
    match &app.ldtk_world {
      Some(world) => ldtk_world = world,
      None => return Ok(()),
    }

    let img_dir = picked_path.parent().ok_or("Picked file is null")?.join(
      picked_path
        .file_stem()
        .ok_or("Unable to get file stem")?
        .to_str()
        .ok_or("Unable to convert file stem")?
        .to_owned()
        + "/png",
    );
    let image = generator::world_image(app, ldtk_world, &img_dir);
    let _result = image.save(saved_path);

    app.saved_path = None;
    app.picked_path = None;
    app.ldtk_world = None;
  }

  Ok(())
}
