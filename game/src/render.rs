use crate::worldcell::WorldCell;
use crate::tile::TileType;

use embedded_graphics::{
    mono_font::{
        ascii::FONT_5X7,
        MonoTextStyleBuilder,
    },
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};

use embedded_graphics_simulator::{
    OutputSettingsBuilder, 
    SimulatorDisplay, 
    Window
};

const SIMULATOR_RENDER_SCALE: u32 = 2;

const DISPLAY_WIDTH: u32 = 240;
const DISPLAY_HEIGHT: u32 = 240;

const VIEWPORT_WIDTH: usize = 40;
const VIEWPORT_HEIGHT: usize = 40;

pub struct Renderer {
    display: SimulatorDisplay<Rgb565>,
    window: Window,
}

impl Renderer {

    pub fn new() -> Renderer {

        let output_settings = OutputSettingsBuilder::new().scale(SIMULATOR_RENDER_SCALE).build();
        Renderer {
            display: SimulatorDisplay::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)),
            window: Window::new("Pocket MUD", &output_settings),
        }
    }

    pub fn render(&mut self, worldcell: &WorldCell) -> Result<(), core::convert::Infallible> {

        let white = MonoTextStyleBuilder::new()
        .font(&FONT_5X7)
        .text_color(Rgb565::new(25, 50, 25))
        .build();

        self.display.clear(Rgb565::BLACK)?;

        let y_inc: i32 = 6;
        let start_pos = Point::new(1, 4);
        let mut pos = start_pos;

        for y in 0..VIEWPORT_HEIGHT {
            pos.x = start_pos.x;
            let mut line = Text::new("", pos, white).draw(&mut self.display)?;

            for x in 0..VIEWPORT_WIDTH {
                let renderdata = TileType::get_render_data(worldcell.tiles[x][y]);
                line = Text::new(&renderdata.glyph, line, white).draw(&mut self.display)?;
            }
            pos.y += y_inc;
        }

        //self.window.show_static(&self.display);
        
        self.window.update(&self.display);

        Ok(())
    }
}

    // let col_status = MonoTextStyleBuilder::new()
    //     .font(&FONT_6X13_BOLD)
    //     .text_color(Rgb565::WHITE)
    //     .background_color(Rgb565::new(6, 12, 6))
    //     .build();

    // let col_xp = MonoTextStyleBuilder::from(&col_status)
    //     .background_color(Rgb565::new(6, 24, 6))
    //     .build();

    // let col_hp = MonoTextStyleBuilder::from(&col_status)
    //     .text_color(Rgb565::RED)
    //     .build();

    // let col_mp = MonoTextStyleBuilder::from(&col_status)
    //     .text_color(Rgb565::CYAN)
    //     .build();

    // let underline = MonoTextStyleBuilder::from(&normal)
    //     .text_color(Rgb565::CSS_YELLOW)
    //     .underline()
    //     .build();

    // let strikethrough = MonoTextStyleBuilder::from(&normal)
    //     .strikethrough_with_color(Rgb565::RED)
    //     .build();

    // let hud_background = MonoTextStyleBuilder::from(&white)
    //     .background_color(Rgb565::CSS_BLUE)
    //     .text_color(Rgb565::CSS_WHITE)
    //     .build();

    // let bold = MonoTextStyleBuilder::from(&white)
    //     .font(&FONT_6X13_BOLD)
    //     .build();

    // let italic = MonoTextStyleBuilder::from(&white)
    //     .font(&FONT_6X13_ITALIC)
    //     .build();


    // 40x16 gameplay
    // 40x4 hud
    // Text::new("   /----\\", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("   |....|      ############", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("   |....|      #          #", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("   |....|      #          #", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("   |.$..+########         #", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("   |....|       #      /--+--\\", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("   \\----/       #      |.....|", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("                #      |.!...\\", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("                #      |......\\-----\\", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("                #      |............|", pos, white).draw(&mut display)?; pos.y += y_inc;
    // let at = Text::new("                #      |..", pos, white).draw(&mut display)?;
    // let at = Text::new("@", at, col_mp).draw(&mut display)?;
    // Text::new("g........|", at, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("     /--\\       #      |............|", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("     |..|       #######+..D.../-----/", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("     |<.+###    #      |...../", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("     \\--/  #    #      |.?...|", pos, white).draw(&mut display)?; pos.y += y_inc;
    // Text::new("           ######      \\-----/", pos, white).draw(&mut display)?; pos.y += y_inc;

    // //HUD
    // let hud_line = Text::new(" 123/456 ", pos, col_hp).draw(&mut display)?;
    // let hud_line = Text::new("[----+----13---", hud_line, col_xp).draw(&mut display)?;
    // let hud_line = Text::new("-+----]", hud_line, col_status).draw(&mut display)?;
    // let _hud_line = Text::new(" 123/456 ", hud_line, col_mp).draw(&mut display)?;
    // pos.x = start_pos.x;
    // pos.y += y_inc;

    // Text::new("Shield Bash: Ready                      ", pos, white).draw(&mut display)?;
    // pos.y += y_inc;
    // Text::new("You hit Goblin for 17 damage!           ", pos, white).draw(&mut display)?;
    // pos.y += y_inc;
    // Text::new("Goblin misses.                          ", pos, white).draw(&mut display)?;
    // pos.y += y_inc;

