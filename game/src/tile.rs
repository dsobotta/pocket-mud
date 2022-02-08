

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    GenWall,
    GenFloor,
    None,
}

pub struct RenderData {
    pub glyph: &'static str,
    pub rgb: u16
}

impl TileType {
    pub fn get_render_data(tile: TileType) -> RenderData {
        match tile {
            TileType::GenWall       => RenderData {glyph: "#", rgb: 0xB596},
            TileType::GenFloor      => RenderData {glyph: ".", rgb: 0x69C2},
            TileType::None          => RenderData {glyph: " ", rgb: 0xF81F},
        }
    }    
}
