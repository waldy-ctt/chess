use std::time::Duration;

use ggez::{graphics, Context};

pub struct MyGame {
    pub dt: Duration,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        _ctx.gfx.add_font(
            "joystix_mono",
            graphics::FontData::from_path(_ctx, "/fonts/joystix monospace.ttf")
                .expect("Failed to load font"),
        );

        MyGame {
            dt: Duration::new(0, 0),
        }
    }
}
