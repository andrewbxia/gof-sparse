// shut up rustc i know there are unneeded parenthesis i used c++ SHDJOIAHDSFPF

use pixels::Error;


mod types;

type Pair = types::Pair;
type PPair = types::PPair;
type P16 = types::P16;
type Pf = types::Pf;
use crate::types::*;


mod game;
use crate::game::{Game};

mod display;
use crate::display::{draw_fps, draw_actives_len, draw_activeness, start};


const RESOLUTION: P16 = (1920/2, 1080/2); // x width, y height
// const DEF_BOUNDS: (Pair, Pair) = ((-20, -5), (20, 5)); // bottom left, top right
const DEF_BOUNDS: (Pair, Pair) = ((0, 0), (1920, 1080)); // bottom left, top right
const DISPLAYSCALE: f64 = 2.0;
const ZOOMSPEED: i32 = 40;


fn main() -> Result<(), Error> {
    start(DEF_BOUNDS, DISPLAYSCALE, ZOOMSPEED, RESOLUTION)?;
    Ok(())
}
