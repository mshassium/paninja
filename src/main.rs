use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::Image as img;
use ggez::nalgebra;
use ggez::*;
use std::collections::{self, HashMap, LinkedList};

static HERO_RUN_FRAME_COUNT: u32 = 20;

struct Hero {
    sprite: graphics::Image,
    draw_param: graphics::DrawParam,
    velocity: f32,
    hero_assets: HeroAnimationAssets,
    count_sprite: usize,
    hero_animation_enum: HeroAnimationEnum,
}

#[derive(PartialEq, Eq, Hash)]
enum HeroAnimationEnum {
    Idle(u32),
    Run(u32),
    Walk(u32),
    Jump(u32),
    Dead(u32),
}

struct HeroAnimationAssets {
    images: HashMap<HeroAnimationEnum, Vec<graphics::Image>>,
}

struct PaninjaGameState {
    hero: Hero,
}

impl HeroAnimationAssets {
    fn new(ctx: &mut Context) -> HeroAnimationAssets {
        let vec_idle_images = vec![
            img::new(ctx, "/dino/idle/Idle (1).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (2).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (3).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (4).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (5).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (6).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (7).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (8).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (9).png").unwrap(),
            img::new(ctx, "/dino/idle/Idle (10).png").unwrap(),
        ];
        let vec_dead_images = vec![
            img::new(ctx, "/dino/dead/Dead (1).png").unwrap(),
            img::new(ctx, "/dino/dead/Dead (2).png").unwrap(),
            img::new(ctx, "/dino/dead/Dead (3).png").unwrap(),
            img::new(ctx, "/dino/dead/Dead (4).png").unwrap(),
            img::new(ctx, "/dino/dead/Dead (5).png").unwrap(),
            img::new(ctx, "/dino/dead/Dead (6).png").unwrap(),
            img::new(ctx, "/dino/dead/Dead (7).png").unwrap(),
            img::new(ctx, "/dino/dead/Dead (8).png").unwrap(),
        ];
        let vec_run_images = vec![
            img::new(ctx, "/dino/run/Run (1).png").unwrap(),
            img::new(ctx, "/dino/run/Run (2).png").unwrap(),
            img::new(ctx, "/dino/run/Run (3).png").unwrap(),
            img::new(ctx, "/dino/run/Run (4).png").unwrap(),
            img::new(ctx, "/dino/run/Run (5).png").unwrap(),
            img::new(ctx, "/dino/run/Run (6).png").unwrap(),
            img::new(ctx, "/dino/run/Run (7).png").unwrap(),
            img::new(ctx, "/dino/run/Run (8).png").unwrap(),
        ];
        let mut hero_animation_assets_map = HashMap::new();
        hero_animation_assets_map.insert(HeroAnimationEnum::Idle(24), vec_idle_images);
        hero_animation_assets_map.insert(HeroAnimationEnum::Dead(24), vec_dead_images);
        hero_animation_assets_map.insert(HeroAnimationEnum::Run(HERO_RUN_FRAME_COUNT), vec_run_images);
        let hero_assets = HeroAnimationAssets {
            images: hero_animation_assets_map,
        };
        hero_assets
    }
}

impl Hero {
    fn new(ctx: &mut Context) -> GameResult<Hero> {
        let sprite = graphics::Image::new(ctx, "/dino/idle/Idle (1).png").unwrap();
        let default_draw_param = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(30.0, 30.0))
            .scale(nalgebra::Vector2::new(0.5, 0.5));
        let new_hero = Hero {
            sprite: sprite,
            draw_param: default_draw_param,
            velocity: 0.0,
            hero_assets: HeroAnimationAssets::new(ctx),
            count_sprite: 0,
            hero_animation_enum: HeroAnimationEnum::Idle(24),
        };
        Ok(new_hero)
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
        match self.hero.hero_animation_enum {
            HeroAnimationEnum::Dead(c) => {
                while (timer::check_update_time(_ctx, c)) {
                    hero_change_sprite(&mut self.hero);
                }
            }
            HeroAnimationEnum::Run(c) => {
                while (timer::check_update_time(_ctx, c)) {
                    hero_change_sprite(&mut self.hero);
                }
            }
            HeroAnimationEnum::Idle(c) => {
                while (timer::check_update_time(_ctx, c)) {
                    hero_change_sprite(&mut self.hero);
                }
            }
            _ => println!("olo"),
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, [0.1, 0.2, 0.3, 1.0].into());
        graphics::draw(_ctx, &self.hero.sprite, self.hero.draw_param)?;
        graphics::present(_ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Right => {
                self.hero.velocity = 1.0;
                self.hero.hero_animation_enum = HeroAnimationEnum::Run(HERO_RUN_FRAME_COUNT);
            }
            KeyCode::Left => {
                self.hero.velocity = -1.0;
                self.hero.hero_animation_enum = HeroAnimationEnum::Run(HERO_RUN_FRAME_COUNT)
            }
            KeyCode::Down => self.hero.hero_animation_enum = HeroAnimationEnum::Dead(24),
            _ => println!("Click on: {:?}", keycode),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Right => {
                self.hero.velocity = 0.0;
                self.hero.hero_animation_enum = HeroAnimationEnum::Idle(24)
            }
            KeyCode::Left => {
                self.hero.velocity = 0.0;
                self.hero.hero_animation_enum = HeroAnimationEnum::Idle(24)
            }
            KeyCode::Down => self.hero.hero_animation_enum = HeroAnimationEnum::Idle(24),
            _ => println!("Click on: {:?}", keycode),
        }
    }
}

fn hero_move_position(hero: &mut Hero) {
    hero.draw_param.dest = mint::Point2 {
        x: hero.draw_param.dest.x + hero.velocity,
        y: hero.draw_param.dest.y,
    };
}

fn hero_change_sprite(hero: &mut Hero) {
    let next_sprite = &hero.hero_assets.images[&hero.hero_animation_enum][hero.count_sprite];
    hero.sprite = img::clone(next_sprite);
    hero.count_sprite = hero.count_sprite + 1;
    if (hero.count_sprite >= hero.hero_assets.images[&hero.hero_animation_enum].len()) {
        hero.count_sprite = 0;
    }
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
