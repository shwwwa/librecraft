// use valence_nbt::serde::CompoundSerializer;
use std::error::Error;
use std::io::prelude::*;

use bevy::prelude::*;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use valence_nbt::from_binary;

/// Slot in inventory's storage.
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Default)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Slot {
    pub slot: i8,
    #[serde(rename = "id")]
    pub id: String,
    pub count: i8,
    // pub tag: unused
}

/// Contains player's abilities.
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Abilities {
    pub walk_speed: f32,
    pub fly_speed: f32,
    pub flying: i8,
    pub instabuild: i8,
    pub invulnerable: i8,
    pub may_build: i8,
    #[serde(rename = "mayfly")]
    pub may_fly: i8,
}

impl Default for Abilities {
    fn default() -> Self {
        Self {
            walk_speed: 0.1,
            fly_speed: 0.05,
            flying: 0,
            instabuild: 0,
            invulnerable: 0,
            may_build: 1,
            may_fly: 0,
        }
    }
}

/// Contains player's nbt data.
#[derive(Deserialize, Serialize, Clone, Resource, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Player {
    /// Data version of player's NBT.
    pub data_version: i32,
    pub absorption_amount: f32,
    pub dimension: String,
    pub health: f32,
    pub inventory: Vec<Slot>,
    pub invulnerable: i8,
    pub score: i32,
    pub selected_item_slot: u32,
    pub xp_level: i32,
    pub xp_total: i32,
    #[serde(rename = "abilities")]
    pub abilities: Abilities,
    #[serde(rename = "foodExhaustionLevel")]
    pub food_exhaustion_level: f32,
    #[serde(rename = "foodLevel")]
    pub food_level: i32,
    #[serde(rename = "foodSaturationLevel")]
    pub food_saturation_level: f32,
    #[serde(rename = "playerGameType")]
    pub player_game_type: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            data_version: 2975,
            absorption_amount: 0.,
            dimension: "minecraft:unknown".to_string(),
            health: 20.,
            inventory: vec![Slot::default()],
            invulnerable: 0,
            score: 0,
            selected_item_slot: 0,
            xp_level: 0,
            xp_total: 0,
            abilities: Abilities::default(),
            food_exhaustion_level: 0.,
            food_level: 0,
            food_saturation_level: 0.,
            player_game_type: 1,
        }
    }
}

/// Setups player data from assets folder.
pub fn setup_player_data(mut player: ResMut<Player>) {
    // Not in assets because loaded by fs.
    match read_player_data("./assets/playerdata/player.dat", &mut player) {
        Ok(()) => info!("Loaded player data: {:#?}", player),
        Err(e) => error!("Couldn't retrieve player data: {}", e),
    }
}

/// Reads player data from `file` path, loads into `player`.
///
/// Returns error if couldn't retrieve data.
pub fn read_player_data(file: &str, player: &mut Player) -> Result<(), Box<dyn Error>> {
    debug!("Path to player data: {:?}", std::fs::canonicalize(file));

    let player_data = std::fs::read(file)?;

    let mut player_data_decoder = GzDecoder::new(&player_data[..]);
    let mut nbt_binary_data: Vec<u8> = vec![];

    let _ = player_data_decoder.read_to_end(&mut nbt_binary_data)?;

    let (compound, _) = from_binary::<String>(&mut nbt_binary_data.as_slice()).unwrap();
    *player = Player::deserialize(compound)?;

    // let compound_2 = player.serialize(CompoundSerializer).unwrap();
    // info!("{:#?}", compound_2);
    Ok(())
}
