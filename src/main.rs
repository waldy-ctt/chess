use game::central::game_modules;
use ggez::{conf::Conf, *};

mod game {
    pub mod events {
        pub mod event_handler;
    }
    pub mod central {
        pub mod game_modules;
    }
    pub mod assets {
        pub mod draws {
            pub mod chess_boards;
        }
        pub mod canvas {
            pub mod main_scene;
        }
        pub mod shared {
            pub mod shared;
        }
    }
}

fn main() {
    let c: Conf = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("holychess", "Waldy")
        .default_conf(c)
        .build()
        .expect("Failed to run holychess");

    let game = game_modules::MyGame::new(&mut ctx);
    event::run(ctx, event_loop, game);
}
