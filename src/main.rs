use rltk::{Rltk, GameState}; // Using amethyst roguelike toolkit (RLTK) https://github.com/amethyst/bracket-lib

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Roguelike").build()?;
    let gamestate = State {};
    rltk::main_loop(context, gamestate)
}