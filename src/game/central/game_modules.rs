use std::time::Duration;

use ggez::Context;

pub struct MyGame {
    pub dt: Duration,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {


        MyGame {
            dt: Duration::new(0, 0),
        }
    }
}
