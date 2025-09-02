use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, ImageReader, RgbaImage};

use crate::schemas::{LDtk, Level};

struct WorldDims {
  x: i32,
  y: i32,
  width: i32,
  height: i32,
}

// TODO: Generate image from world levels
pub fn world_image(world: &LDtk, img_path: &PathBuf) -> RgbaImage {
  let world_size = total_world_size(world);
  let mut img = RgbaImage::new(
    world_size.width.try_into().unwrap(),
    world_size.height.try_into().unwrap(),
  );
  for level in &world.levels {
    let lvl_img = level_image(level, img_path);
    for x in (level.world_x as u32)..((level.world_x + level.px_wid) as u32) {
      for y in (level.world_y as u32)..((level.world_y + level.px_hei) as u32) {
        let pixel = lvl_img.get_pixel(x, y);
        img.put_pixel(x, y, pixel.to_owned());
      }
    }
  }
  img
}

fn level_image(level: &Level, img_path: &PathBuf) -> DynamicImage {
  let mut lvl_name = level.identifier.to_owned();
  lvl_name.push_str(".png");
  ImageReader::open(img_path.join(lvl_name)).unwrap().decode().unwrap()
}

fn total_world_size(world: &LDtk) -> WorldDims {
  let mut dims = WorldDims {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
  };

  dims.x = world
    .levels
    .iter()
    .min_by(|levela, levelb| levela.world_x.cmp(&levelb.world_x))
    .unwrap()
    .world_x;
  dims.y = world
    .levels
    .iter()
    .min_by(|levela, levelb| levela.world_y.cmp(&levelb.world_y))
    .unwrap()
    .world_y;
  let max_level_x = world
    .levels
    .iter()
    .max_by(|levela, levelb| {
      (levela.world_x + levela.px_wid).cmp(&(levelb.world_x + levelb.px_wid))
    })
    .unwrap();
  let max_level_y = world
    .levels
    .iter()
    .max_by(|levela, levelb| {
      (levela.world_y + levela.px_hei).cmp(&(levelb.world_y + levelb.px_hei))
    })
    .unwrap();
  dims.width = max_level_x.world_x + max_level_x.px_wid;
  dims.height = max_level_y.world_y + max_level_y.px_hei;

  dims
}
