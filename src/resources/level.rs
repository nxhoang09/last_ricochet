use bevy::prelude::*;
use crate::resources::game_config::{LevelConfig, WaveData};

#[derive(Resource)]
pub struct LevelManager {
    pub current_level: usize,      
    pub current_wave_index: usize,
    
   
    pub current_waves_data: Vec<WaveData>, 
    pub difficulty_multiplier: f32,

   
    pub enemies_spawned: usize,
    pub spawn_timer: Timer,
    
    
    pub level_completed: bool,
}

impl Default for LevelManager {
    fn default() -> Self {
        let mut manager = Self {
            current_level: 1,
            current_wave_index: 0,
            current_waves_data: vec![],
            difficulty_multiplier: 1.0,
            enemies_spawned: 0,
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            level_completed: false,
        };
        
        manager.load_level(1);
        manager
    }
}

impl LevelManager {
    pub fn load_level(&mut self, level: usize) {
        let config = LevelConfig::get_config_for_level(level);
        
        self.current_level = level;
        self.current_wave_index = 0;
        self.current_waves_data = config.waves;
        self.difficulty_multiplier = config.difficulty_multiplier;
        
        self.enemies_spawned = 0;
        self.level_completed = false;
        self.spawn_timer.reset();
        
        info!("Loaded Level {} with multiplier {}", level, self.difficulty_multiplier);
    }

    pub fn next_level(&mut self) {
        self.load_level(self.current_level + 1);
    }
}