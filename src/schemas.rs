use serde::{Deserialize, Serialize};

// ~~~ LDTK Root ~~~
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LDtk {
  pub bg_color: String,
  pub defs: Definitions,
  pub external_levels: bool,
  pub iid: String,
  pub json_version: String,
  pub levels: Vec<Level>,
  pub toc: Vec<TocEntry>,
  pub world_grid_height: Option<i16>,
  pub world_grid_width: Option<i16>,
  pub world_layout: Option<WorldLayout>,
  pub worlds: Vec<World>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TocEntry {
  pub identifier: String,
  pub instances_data: Vec<TocEntryData>,
  pub instances: Vec<EntityInstanceRef>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TocEntryData {
  // TODO: add fields
  pub hei_px: i32,
  pub wid_px: i32,
  pub iids: EntityInstanceRef,
  pub world_x: i32,
  pub world_y: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct World {
  pub identifier: String,
  pub iid: String,
  pub levels: Vec<Level>,
  pub world_grid_height: i32,
  pub world_grid_width: i32,
  pub world_layout: WorldLayout
}
#[derive(Serialize, Deserialize, Debug)]
pub enum WorldLayout {
  Free,
  GridVania,
  LinearHorizontal,
  LinearVertical
}

// ~~~ Level ~~~
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Level {
  #[serde(rename(serialize = "__bgColor", deserialize = "__bgColor"))]
  pub bg_color: String,
  #[serde(rename(serialize = "__bgPos", deserialize = "__bgPos"))]
  pub bg_pos: Option<BgPos>,
  #[serde(alias = "__neighbours")]
  pub neighbours: Vec<Neighbour>,
  pub bg_rel_path: Option<String>,
  pub external_rel_path: Option<String>,
  pub field_instances: Vec<FieldInstance>,
  pub identifier: String,
  pub iid: String,
  pub layer_instances: Option<Vec<LayerInstance>>,
  pub px_hei: i32,
  pub px_wid: i32,
  pub uid: i32,
  pub world_depth: i16,
  pub world_x: i32,
  pub world_y: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BgPos(
  Vec<f32>,
  Vec<f32>,
  Vec<i32>,
);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Neighbour {
  pub level_iid: String,
  pub dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LayerInstance {
  #[serde(alias = "__cHei")]
  pub c_hei: u16,
  #[serde(alias = "__cWid")]
  pub c_wid: u16,
  #[serde(alias = "__gridSize")]
  pub grid_size: i16,
  #[serde(alias = "__identifier")]
  pub identifier: String,
  #[serde(alias = "__opacity")]
  pub opacity: f32,
  #[serde(alias = "__pxTotalOffsetX")]
  pub px_total_offset_x: i16,
  #[serde(alias = "__pxTotalOffsetY")]
  pub px_total_offset_y: i16,
  #[serde(alias = "__tilesetDefUid")]
  pub tileset_def_uid: Option<i32>,
  #[serde(alias = "__tilesetRelPath")]
  pub tileset_rel_path: Option<String>,
  #[serde(alias = "__type")]
  pub _type: String,
  pub auto_layer_tiles: Vec<TileInstance>,
  pub entity_instances: Vec<EntityInstance>,
  pub grid_tiles: Vec<TileInstance>,
  pub iid: String,
  pub int_grid_csv: Vec<i32>,
  pub layer_def_uid: i32,
  pub level_id: i32,
  pub override_tileset_uid: Option<i32>,
  pub px_offset_x: i16,
  pub px_offset_y: i16,
  pub visible: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TileInstance {
  pub a: f32,
  pub f: u8,
  pub px: Vec<i32>,
  pub src: Vec<i32>,
  pub t: i32
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntityInstance {
  #[serde(alias = "__grid")]
  pub grid: Vec<i32>,
  #[serde(alias = "__identifier")]
  pub identifier: String,
  #[serde(alias = "__pivot")]
  pub pivot: Vec<f32>,
  #[serde(alias = "__smartColor")]
  pub smart_color: String,
  #[serde(alias = "__tags")]
  pub tags: Vec<String>,
  #[serde(alias = "__tile")]
  pub tile: Option<TilesetRect>,
  #[serde(alias = "__worldX")]
  pub world_x: Option<i32>,
  #[serde(alias = "__worldY")]
  pub world_y: Option<i32>,
  pub def_uid: i32,
  pub field_instances: Vec<FieldInstance>,
  pub height: u16,
  pub width: u16,
  pub iid: String,
  pub px: Vec<i32>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldInstance {
  #[serde(alias = "__identifier")]
  pub identifier: String,
  #[serde(alias = "__tile")]
  pub tile: Option<TilesetRect>,
  #[serde(alias = "__type")]
  pub _type: String,
  pub value: String,
  pub def_uid: i32
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntityInstanceRef {
  pub entity_iid: String,
  pub layer_iid: String,
  pub level_iid: String,
  pub world_iid: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GridPoint {
  pub cx: i32,
  pub cy: i32,
}

// ~~~ Definitions ~~~
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Definitions {
  pub entities: Vec<EntityDef>,
  pub enums: Vec<EnumDef>,
  pub external_enums: Vec<EnumDef>,
  pub layers: Vec<LayerDef>,
  pub level_fields: Vec<FieldInstance>,
  pub tilesets: Vec<TilesetDef>
}

// ~~~ Layer Definition ~~~
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LayerDef {
  pub _type: String,
  pub auto_source_layer_def_uid: Option<i32>,
  pub display_opacity: f32,
  pub identifier: String,
  pub int_grid_values: Vec<LayerGridValue>,
  pub int_grid_values_groups: Vec<LayerGridGroup>,
  pub parallax_factor_x: f32,
  pub parallax_factor_y: f32,
  pub parallax_scaling: bool,
  pub px_offset_x: i16,
  pub px_offset_y: i16,
  pub tileset_def_uid: Option<i32>,
  pub uid: i32
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LayerGridValue {
  pub color: String,
  pub group_uid: i32,
  pub identifier: Option<String>,
  pub tile: Option<TilesetRect>,
  pub value: i16
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LayerGridGroup {
  pub color: Option<String>,
  pub identifier: Option<String>,
  pub uid: i32
}

// ~~~ Entity Definition ~~~
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntityDef {
  pub color: String,
  pub height: u16,
  pub width: u16,
  pub identifier: String,
  pub nine_slice_borders: Vec<i16>,
  pub pivot_x: f32,
  pub pivot_y: f32,
  pub tile_rect: Option<TilesetRect>,
  pub tile_render_mode: EntityTileRenderMode,
  pub tileset_id: Option<i32>,
  pub ui_tile_rect: Option<TilesetRect>,
  pub uid: i32,
  pub tile_id: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum EntityTileRenderMode {
  Cover,
  FitInside,
  Repeat,
  Stretch,
  FullSizeCropped,
  FullSizeUncropped,
  NineSlice
}

// ~~~ Tileset Definition ~~~
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TilesetDef {
  #[serde(alias = "__cHei")]
  pub c_hei: u16,
  #[serde(alias = "__cWid")]
  pub c_wid: u16,
  pub custom_data: Vec<TileCustomData>,
  pub embed_atlas: Option<String>,
  pub enum_tags: Vec<TileEnumTag>,
  pub identifier: String,
  pub padding: i16,
  pub px_hei: u16,
  pub px_wid: u16,
  pub rel_path: Option<String>,
  pub spacing: i16,
  pub tags: Vec<String>,
  pub tags_source_enum_uid: Option<i32>,
  pub tile_grid_size: i16,
  pub uid: i32
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TileCustomData {
  pub data: String, 
  pub tile_id: i32
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TileEnumTag {
  pub enum_value_id: String, 
  pub tile_ids: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TilesetRect {
  pub h: i16,
  pub w: i16,
  pub tileset_uid: i32,
  pub x: i32,
  pub y: i32
}

// ~~~ Enum Definition ~~~
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumDef {
  pub external_rel_path: Option<String>,
  pub icon_tileset_uid: Option<i32>,
  pub identifier: String,
  pub tags: Vec<String>,
  pub uid: i32,
  pub values: Vec<EnumValueDef>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueDef {
  pub color: i32,
  pub id: String,
  pub tile_rect: Option<TilesetRect>
}