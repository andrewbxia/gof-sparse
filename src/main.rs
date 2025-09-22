// shut up rustc i know there are unneeded parenthesis i used c++ SHDJOIAHDSFPF

use pixels::Error;


mod types;

type Pair = types::Pair;
type P16 = types::P16;

mod game; // needs this or else display/mod.rs starts complaining??????>?>
mod display;
use crate::display::gentlemen_synchronize_your_death_watches;


const RESOLUTION: P16 = (1920/2, 1080/2); // x width, y height
// const DEF_BOUNDS: (Pair, Pair) = ((-20, -5), (20, 5)); // bottom left, top right
const DEF_BOUNDS: (Pair, Pair) = ((0, 0), (1920, 1080)); // bottom left, top right
const DISPLAYSCALE: f64 = 2.0;
const ZOOMSPEED: i32 = 40;


fn main() -> Result<(), Error> {
    gentlemen_synchronize_your_death_watches(DEF_BOUNDS, DISPLAYSCALE, ZOOMSPEED, RESOLUTION)?;
    Ok(())
}
