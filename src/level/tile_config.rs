#[derive(Clone, Copy)]
pub struct TileConfig {
    pub has_collision: bool,
    pub z: f32,
}

// index = sprite index = grayscale value
pub const TILE_CONFIGS: [TileConfig; 28] = [
    TileConfig { has_collision: true, z: 10.0 }, // 0 
    TileConfig { has_collision: true,  z: 10.0  }, // 1 
    TileConfig { has_collision: true,  z: 10.0  }, // 2 
    TileConfig { has_collision: true,  z: 10.0  }, // 3 
    TileConfig { has_collision: true,  z: 10.0  }, // 4 
    TileConfig { has_collision: true, z: 10.0 }, // 5
    TileConfig { has_collision: true, z: 10.0 }, // 6
    TileConfig { has_collision: true, z: 10.0 }, // 7
    TileConfig { has_collision: true, z: 10.0 }, // 8
    TileConfig { has_collision: true, z: 10.0 }, // 9
    TileConfig { has_collision: true, z: 10.0 }, // 10
    TileConfig { has_collision: false, z: -0.5 }, // 11
    TileConfig { has_collision: true, z: 10.0 }, // 12
    TileConfig { has_collision: false, z: -0.5 }, // 13
    TileConfig { has_collision: true, z: 0.0 }, // 14
    TileConfig { has_collision: true, z: 0.0 }, // 15
    TileConfig { has_collision: true, z: 0.0 }, // 16
    TileConfig { has_collision: false, z: -0.5 }, // 17
    TileConfig { has_collision: false, z: -0.5 }, // 18
    TileConfig { has_collision: false, z: -0.5 }, // 19
    TileConfig { has_collision: false, z: -0.5 }, // 20
    TileConfig { has_collision: false, z: -1.0 }, // 21
    TileConfig { has_collision: false, z: -1.0 }, // 22
    TileConfig { has_collision: false, z: -1.0 }, // 23
    TileConfig { has_collision: false, z: -1.0 }, // 24
    TileConfig { has_collision: false, z: -1.0 }, // 25
    TileConfig { has_collision: false, z: -1.0 }, // 26
    TileConfig { has_collision: false, z: -1.0 }, // 27
];
