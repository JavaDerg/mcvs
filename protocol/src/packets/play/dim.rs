#[derive(serde::Deserialize, serde::Serialize)]
pub struct DimensionCodec {
    #[serde(rename = "minecraft:dimension_type")]
    pub dim_type: DimensionTypeRegistry,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub wg_biom: BiomeRegistry,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DimensionTypeRegistry {
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: Vec<DimensionTypeRegistryEntry>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DimensionTypeRegistryEntry {
    name: String,
    id: i32,
    element: DimensionType,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DimensionType {
    piglin_safe: i8,
    natural: i8,
    fixed_time: f32,
    infiniburn: String,
    respawn_anchor_works: i8,
    has_skylight: i8,
    bed_works: i8,
    effects: String,
    has_raids: i8,
    min_y: i32,
    height: i32,
    logical_height: i32,
    coordinate_scale: f32,
    ultrawarm: i8,
    has_ceiling: i8,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BiomeRegistry {
    pub name: String,
    pub id: i32,
    pub element: BiomeProperties,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BiomeProperties {}
