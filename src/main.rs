// shut up rustc i know there are unneeded parenthesis i used c++ SHDJOIAHDSFPF
use pixels::Error;
use std::env;
use std::fs::File;

mod types;
mod input;
mod font;

type Pair = types::Pair;
type P16 = types::P16;

mod game; // needs this or else display/mod.rs starts complaining??????>?>
mod display;
use crate::game::Game;
use crate::display::gentlemen_synchronize_your_death_watches;


const RESOLUTION: P16 = (1920, 1080); // x width, y height
// const DEF_BOUNDS: (Pair, Pair) = ((-20, -5), (20, 5)); // bottom left, top right
const DEF_BOUNDS: (Pair, Pair) = ((0, 0), (1920, 1080)); // bottom left, top right
const DISPLAYSCALE: f64 = 1.0;
const ZOOMSPEED: i32 = 40;


fn main() -> Result<(), Error> {

    let mut game = Game {
        bounds: DEF_BOUNDS,
        ..Default::default()
    };

    
    
    let args: Vec<String> = env::args().collect();

    if(args.len() > 1){
        let filename = &args[1];
        let pairs = input::readfilegrid(filename);
        for p in pairs{
            game.addcell(p);
        }
        println!("Loaded {} cells from {}", game.cells.len(), filename);
    }


    gentlemen_synchronize_your_death_watches(&mut game, DISPLAYSCALE, ZOOMSPEED, RESOLUTION)?;
    Ok(())
}
