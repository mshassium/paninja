use ggez::event;
use ggez::graphics;
use ggez::nalgebra;
use ggez::*;

struct Hero {
    sprite: graphics::Image,
    draw_param: graphics::DrawParam,
}

struct PaninjaGameState {
    hero: Hero,
}

impl Hero {
    fn new(ctx: &mut Context) -> GameResult<Hero> {
        let sprite = graphics::Image::new(ctx, "/hero.png").unwrap();
        let default_draw_param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(30.0, 30.0))
            .scale(nalgebra::Vector2::new(0.5, 0.5));
        let new_hero = Hero {
            sprite: sprite,
            draw_param: default_draw_param,
        };
        Ok(new_hero)
    }

    fn update_draw_params(hero: &mut Hero, params: graphics::DrawParam) {
        hero.draw_param = params;
    }
}

impl PaninjaGameState {
    fn new(ctx: &mut Context) -> GameResult<PaninjaGameState> {
        let hero = Hero::new(ctx)?;
        let state = PaninjaGameState { hero };
        Ok(state)
    }
}

impl event::EventHandler for PaninjaGameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        hero_move_position(&mut self.hero);
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, [0.1, 0.2, 0.3, 1.0].into());
        graphics::draw(_ctx, &self.hero.sprite, self.hero.draw_param)?;
        graphics::present(_ctx)?;
        Ok(())
    }
}

fn hero_move_position(hero: &mut Hero) {
    hero.draw_param.dest = mint::Point2 {
        x: hero.draw_param.dest.x + 1.0,
        y: hero.draw_param.dest.y + 1.0,
    };
}

fn main() -> GameResult {
    let game_builder = ggez::ContextBuilder::new("paninja", "mshassium");
    let (context, event_loop) = &mut game_builder
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true),
        )
        .add_resource_path("./resources")
        .build()?;
    println!("Context: {:?}, Event Loop: {:?}", context, event_loop);
    let game_state = &mut PaninjaGameState::new(context)?;
    event::run(context, event_loop, game_state)?;
    Ok(())
}
