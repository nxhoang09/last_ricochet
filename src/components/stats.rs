use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerStats {
    pub current_hp: f32,
    pub max_hp: f32,
    pub money: u32,
    pub damage: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            current_hp: 5.0,
            max_hp:5.0, 
            money: 10,
            damage: 1.0,
        }
    }
}


#[derive(Component)]
pub struct HpText;

#[derive(Component)]
pub struct MoneyText;