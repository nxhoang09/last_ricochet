use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum EnemyType {
    Normal, 
    Tank,   
    Speed,  
}

#[derive(Clone)]
pub struct WaveData {
    pub enemy_count: usize,
    pub spawn_interval: f32,
    pub enemy_type: EnemyType, 
}

pub struct LevelConfig {
    pub waves: Vec<WaveData>,
    pub difficulty_multiplier: f32, 
}

impl LevelConfig {
    pub fn get_config_for_level(level: usize) -> Self {
        match level {
            1 => LevelConfig {
                difficulty_multiplier: 1.0,
                waves: vec![
                    WaveData { enemy_count: 5, spawn_interval: 1.5, enemy_type: EnemyType::Normal },
                    // WaveData { enemy_count: 8, spawn_interval: 1.2, enemy_type: EnemyType::Normal },
                ],
            },
            2 => LevelConfig {
                difficulty_multiplier: 1.2, 
                waves: vec![
                    WaveData { enemy_count: 8, spawn_interval: 1.2, enemy_type: EnemyType::Normal },
                    WaveData { enemy_count: 5, spawn_interval: 1.5, enemy_type: EnemyType::Speed }, 
                    WaveData { enemy_count: 10, spawn_interval: 1.0, enemy_type: EnemyType::Normal },
                ],
            },
            3 => LevelConfig {
                difficulty_multiplier: 1.5,
                waves: vec![
                    WaveData { enemy_count: 10, spawn_interval: 1.0, enemy_type: EnemyType::Normal },
                    WaveData { enemy_count: 4, spawn_interval: 2.0, enemy_type: EnemyType::Tank }, 
                    WaveData { enemy_count: 15, spawn_interval: 0.8, enemy_type: EnemyType::Speed },
                ],
            },
            _ => LevelConfig {
                difficulty_multiplier: 3.0,
                waves: vec![WaveData { enemy_count: 50, spawn_interval: 0.3, enemy_type: EnemyType::Normal }],
            },
        }
    }
}