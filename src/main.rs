use ggez::*;

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello!! {}ns", self.dt.as_nanos());
        Ok(())
    }
}

fn main() {
    println!("Starting");
    let state: State = State {
        dt: std::time::Duration::new(0, 0),
    };
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "Waldy")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
