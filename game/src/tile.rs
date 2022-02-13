
#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    GenFloor = 0,    
    GenWall = 1,
    WallHorizontal,
    WallVertical,
    WallCornerNW,
    WallCornerNE,
    WallCornerSW,
    WallCornerSE,
    WallThreeW,
    WallThreeE,
    WallThreeN,
    WallThreeS,
    WallFour,
    Player,
    None,
}

pub struct RenderData {
    pub glyph: &'static str,
    pub rgb: u16
}

impl TileType {
    pub fn get_render_data(tile: TileType) -> RenderData {
        match tile {
            TileType::GenWall           => RenderData {glyph: "\u{2588}", rgb: 0xB596},
            TileType::GenFloor          => RenderData {glyph: "\u{00B7}", rgb: 0x69C2},
            TileType::WallHorizontal    => RenderData {glyph: "\u{2550}", rgb: 0xB596},
            TileType::WallVertical      => RenderData {glyph: "\u{2551}", rgb: 0xB596},
            TileType::WallCornerNW      => RenderData {glyph: "\u{2554}", rgb: 0xB596},
            TileType::WallCornerNE      => RenderData {glyph: "\u{2557}", rgb: 0xB596},
            TileType::WallCornerSW      => RenderData {glyph: "\u{255A}", rgb: 0xB596},
            TileType::WallCornerSE      => RenderData {glyph: "\u{255D}", rgb: 0xB596},
            TileType::WallThreeW        => RenderData {glyph: "\u{2563}", rgb: 0xB596},
            TileType::WallThreeE        => RenderData {glyph: "\u{2560}", rgb: 0xB596},
            TileType::WallThreeN        => RenderData {glyph: "\u{2569}", rgb: 0xB596},
            TileType::WallThreeS        => RenderData {glyph: "\u{2566}", rgb: 0xB596},
            TileType::WallFour          => RenderData {glyph: "\u{256C}", rgb: 0xB596},            
            TileType::Player            => RenderData {glyph: "\u{263A}", rgb: 0xB7E0},
            TileType::None              => RenderData {glyph: " ", rgb: 0xF81F},
        }
    }    
}
