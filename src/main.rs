// basic structure, for minimal terminal.
// requires dependency bracket-lib = "~0.8.1"

use bracket_lib::prelude::*;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler basic basic framework")
        .build()?;

    main_loop(context, State{})
}
