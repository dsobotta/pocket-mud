
#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    GenFloor = 0,    
    GenWall = 1,
    WallHorizontal = 2,
    WallVertical = 3,
    WallCornerNW = 4,
    WallCornerNE = 5,
    WallCornerSW = 6,
    WallCornerSE = 7,
    WallThreeW = 8,
    WallThreeE = 9,
    WallThreeN = 10,
    WallThreeS = 11,
    WallFour = 12,
    WallCapN = 13,
    WallCapS = 14,
    WallCapW = 15,
    WallCapE = 16,
    WallIsland = 17,
    WallMax = 18,
    FloorDirt,
    Player,
    Test,
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
            TileType::WallCornerNW      => RenderData {glyph: "\u{255D}", rgb: 0xB596},
            TileType::WallCornerNE      => RenderData {glyph: "\u{255A}", rgb: 0xB596},
            TileType::WallCornerSW      => RenderData {glyph: "\u{2557}", rgb: 0xB596},
            TileType::WallCornerSE      => RenderData {glyph: "\u{2554}", rgb: 0xB596},
            TileType::WallThreeW        => RenderData {glyph: "\u{2563}", rgb: 0xB596},
            TileType::WallThreeE        => RenderData {glyph: "\u{2560}", rgb: 0xB596},
            TileType::WallThreeN        => RenderData {glyph: "\u{2569}", rgb: 0xB596},
            TileType::WallThreeS        => RenderData {glyph: "\u{2566}", rgb: 0xB596},
            TileType::WallFour          => RenderData {glyph: "\u{256C}", rgb: 0xB596},
            TileType::WallCapN          => RenderData {glyph: "\u{25BC}", rgb: 0xB596},
            TileType::WallCapS          => RenderData {glyph: "\u{25B2}", rgb: 0xB596},
            TileType::WallCapW          => RenderData {glyph: "\u{25BA}", rgb: 0xB596},
            TileType::WallCapE          => RenderData {glyph: "\u{25C4}", rgb: 0xB596},
            TileType::WallIsland        => RenderData {glyph: "\u{25A0}", rgb: 0xB596},                                     
            TileType::WallMax           => RenderData {glyph: "?", rgb: 0xB596},
            TileType::FloorDirt         => RenderData {glyph: "\u{2219}", rgb: 0xB596},             
            TileType::Player            => RenderData {glyph: "\u{263A}", rgb: 0xB7E0},
            TileType::Test              => RenderData {glyph: "\u{2302}", rgb: 0xB596},
            TileType::None              => RenderData {glyph: " ", rgb: 0xF81F},
        }
    }    
}
