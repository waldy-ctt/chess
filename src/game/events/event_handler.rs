use crate::game::central::game_modules;
use ggez::{event::EventHandler, Context, GameError, GameResult};

impl EventHandler<GameError> for game_modules::MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        println!("hi");
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}
