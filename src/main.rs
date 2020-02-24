use ggez::*;

struct PaninjaGameState {}

impl event::EventHandler for PaninjaGameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let game_builder = ggez::ContextBuilder::new("paninja", "mshassium");
    let (context, event_loop) = &mut game_builder
        .window_mode(conf::WindowMode::default()
                    .fullscreen_type(conf::FullscreenType::True))
        .build()?;
    println!("Context: {:?}, Event Loop: {:?}", context, event_loop);
    let game_state = &mut PaninjaGameState {};
    event::run(context, event_loop, game_state)?;
    Ok(())
}
