use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X13, FONT_6X13_BOLD, FONT_6X13_ITALIC},
        MonoTextStyleBuilder,
    },
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(240, 240));

    let white = MonoTextStyleBuilder::new()
        .font(&FONT_6X13)
        .text_color(Rgb565::WHITE)
        .build();

    let col_status = MonoTextStyleBuilder::new()
        .font(&FONT_6X13_BOLD)
        .text_color(Rgb565::WHITE)
        .background_color(Rgb565::new(6, 12, 6))
        .build();

    let col_xp = MonoTextStyleBuilder::from(&col_status)
        .background_color(Rgb565::new(6, 24, 6))
        .build();

    let col_hp = MonoTextStyleBuilder::from(&col_status)
        .text_color(Rgb565::RED)
        .build();

    let col_mp = MonoTextStyleBuilder::from(&col_status)
        .text_color(Rgb565::CYAN)
        .build();

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

    let bold = MonoTextStyleBuilder::from(&white)
        .font(&FONT_6X13_BOLD)
        .build();

    // let italic = MonoTextStyleBuilder::from(&white)
    //     .font(&FONT_6X13_ITALIC)
    //     .build();

    let y_inc: i32 = 12;
    let start_pos = Point::new(1, 9);
    let mut pos = start_pos;


    // 40x16 gameplay
    // 40x4 hud
    Text::new("   /----\\", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("   |....|      ############", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("   |....|      #          #", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("   |....|      #          #", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("   |.$..+########         #", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("   |....|       #      /--+--\\", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("   \\----/       #      |.....|", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("                #      |.!...\\", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("                #      |......\\-----\\", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("                #      |............|", pos, white).draw(&mut display)?; pos.y += y_inc;
    let at = Text::new("                #      |..", pos, white).draw(&mut display)?;
    let at = Text::new("@", at, col_mp).draw(&mut display)?;
    Text::new(".........|", at, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("     /--\\       #      |............|", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("     |..|       #######+..D.../-----/", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("     |<.+###    #      |...../", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("     \\--/  #    #      |.?...|", pos, white).draw(&mut display)?; pos.y += y_inc;
    Text::new("           ######      \\-----/", pos, white).draw(&mut display)?; pos.y += y_inc;

    //HUD
    let hud_line = Text::new(" 123/456 ", pos, col_hp).draw(&mut display)?;
    let hud_line = Text::new("[----+----13---", hud_line, col_xp).draw(&mut display)?;
    let hud_line = Text::new("-+----]", hud_line, col_status).draw(&mut display)?;
    let hud_line = Text::new(" 123/456 ", hud_line, col_mp).draw(&mut display)?;
    pos.x = start_pos.x;
    pos.y += y_inc;

    Text::new("Shield Bash: Ready                      ", pos, white).draw(&mut display)?;
    pos.y += y_inc;
    Text::new("You hit Goblin for 17 damage!           ", pos, white).draw(&mut display)?;
    pos.y += y_inc;
    Text::new("Goblin misses.                          ", pos, white).draw(&mut display)?;
    pos.y += y_inc;


    // let hud_line = Text::new(" 123/456 ", pos, col_hp).draw(&mut display)?;
    // let hud_line = Text::new("[----+----13----+----]", hud_line, col_status).draw(&mut display)?;
    // let hud_line = Text::new(" 123/456 ", hud_line, col_mp).draw(&mut display)?;
    // pos.x = start_pos.x;
    // pos.y += y_inc;

    // Text::new("Shield Bash: Ready                      ", pos, white).draw(&mut display)?;
    // pos.y += y_inc;
    // Text::new("You hit Goblin for 17 damage!           ", pos, white).draw(&mut display)?;
    // pos.y += y_inc;
    // Text::new("Goblin misses.                          ", pos, white).draw(&mut display)?;
    // pos.y += y_inc;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Pocket MUD", &output_settings).show_static(&display);

    Ok(())
}