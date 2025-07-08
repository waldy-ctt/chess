use crate::game::{assets::canvas::main_scene::main_scene, central::game_modules};
use ggez::{event::EventHandler, graphics::Canvas, Context, GameError, GameResult};

impl EventHandler<GameError> for game_modules::MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let main_scene: Canvas = main_scene(_ctx);
        main_scene.finish(_ctx)?;
        Ok(())
    }
}
