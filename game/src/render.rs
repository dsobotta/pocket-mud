use crate::worldcell as worldcell;
use crate::tile::TileType;

use embedded_graphics::{
    mono_font::{
        MonoTextStyleBuilder,
    },
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};

use ibm437::{
    IBM437_8X8_REGULAR,
    // IBM437_8X8_BOLD
};

use embedded_graphics_simulator::{
    OutputSettingsBuilder, 
    SimulatorDisplay, 
    Window
};

// use st7789::ST7789;


const SIMULATOR_RENDER_SCALE: u32 = 3;

const DISPLAY_WIDTH: u32 = 240;
const DISPLAY_HEIGHT: u32 = 240;

const VIEWPORT_WIDTH: u16 = 30;
const VIEWPORT_HEIGHT: u16 = 30;
const HALF_VIEWPORT_WIDTH: u16 = VIEWPORT_WIDTH / 2;
const HALF_VIEWPORT_HEIGHT: u16 = VIEWPORT_HEIGHT / 2;
const MAX_VIEWPORT_X: u16 = worldcell::MAX_WIDTH - 1 - VIEWPORT_WIDTH;
const MAX_VIEWPORT_Y: u16 = worldcell::MAX_HEIGHT - 1 - VIEWPORT_HEIGHT;


pub struct Renderer {
    display: SimulatorDisplay<Rgb565>,
    window: Window,
    focus_region: worldcell::Region,
}

impl Renderer {

    pub fn new() -> Renderer {

        let output_settings = OutputSettingsBuilder::new().scale(SIMULATOR_RENDER_SCALE).build();
        let default_region = worldcell::Region {
            x: 0,
            y: 0,
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT
        };
        
        Renderer {
            display: SimulatorDisplay::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)),
            window: Window::new("Pocket MUD", &output_settings),
            focus_region: default_region,
        }
    }

    pub fn set_focus(&mut self, x: u16, y: u16) {

        let start_x = if x < HALF_VIEWPORT_WIDTH {
            0
        } else {
            if x > MAX_VIEWPORT_X {
                MAX_VIEWPORT_X
            } else {
                x - HALF_VIEWPORT_WIDTH
            }
        };

        let start_y = if y < HALF_VIEWPORT_HEIGHT {
            0
        } else {
            if y > MAX_VIEWPORT_Y {
                MAX_VIEWPORT_Y
            } else {
                y - HALF_VIEWPORT_HEIGHT
            }
        };

        self.focus_region = worldcell::Region {
            x: start_x,
            y: start_y,
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT
        }
    }

    pub fn render(&mut self, worldcell: &worldcell::WorldCell) -> Result<(), core::convert::Infallible> {

        let white = MonoTextStyleBuilder::new()
        .font(&IBM437_8X8_REGULAR)
        .text_color(Rgb565::new(25, 50, 25))
        .build();

        self.display.clear(Rgb565::BLACK)?;

        let y_inc: i32 = 8;
        let render_start_pixel = Point::new(0, 6);
        let mut pos = render_start_pixel;

        for y in 0..self.focus_region.height {
            pos.x = render_start_pixel.x;
            let mut line = Text::new("", pos, white).draw(&mut self.display)?;

            for x in 0..self.focus_region.width {
                let cell_x = self.focus_region.x + x;
                let cell_y = self.focus_region.y + y;
                let renderdata = TileType::get_render_data(worldcell.read(cell_x , cell_y));
                line = Text::new(&renderdata.glyph, line, white).draw(&mut self.display)?;
            }
            pos.y += y_inc;
        }

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

    // let hud_background = MonoTextStyleBuilder::from(&white)
    //     .background_color(Rgb565::CSS_BLUE)
    //     .text_color(Rgb565::CSS_WHITE)
    //     .build();


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

