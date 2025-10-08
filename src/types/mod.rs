use std::time::{Duration, Instant};


pub(crate) const RED: [u8; 4] = [0xff, 0x00, 0x00, 0xff]; // r g b a
pub(crate) const GREEN: [u8; 4] = [0x00, 0xff, 0x00, 0xff];
pub(crate) const BLUE: [u8; 4] = [0x00, 0x00, 0xff, 0xff];
pub(crate) const WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
pub(crate) const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xff];


type P8 = (u8, u8);
pub(crate) type P16 = (u16, u16);
pub(crate) type Pf = (f32, f32);
pub(crate) type Pair = (i32, i32);
pub(crate) type Cell = (Pair, i32);

pub(crate) type PPair = i64;

pub(crate)trait Unpack{
    fn unpack(&self) -> Pair;
}
impl Unpack for PPair{
    fn unpack(&self) -> Pair{
        ((self >> 32) as i32, (self & 0xffff_ffff) as u32 as i32) // u32 for signange
    }
}

pub(crate)trait AddASelf{
    fn addaself(&mut self, other: i32);
}

pub(crate)trait AddBSelf{
    fn addbself(&mut self, other: i32);
}

pub(crate)trait AddSelf{
    fn addself(&mut self, other: &PPair);
}

pub(crate)trait AddA{
    fn adda(&self, other: i32) -> PPair;
}
pub(crate)trait AddB{
    fn addb(&self, other: i32) -> PPair;
}
pub(crate) trait Add{
    fn add(&self, other: PPair) -> PPair;
}


impl AddSelf for PPair {
    #[inline(always)]
    fn addself(&mut self, other: &PPair) {
        let a = *self;
        let b = *other;
        let hi = ((a >> 32) as i32).wrapping_add((b >> 32) as i32);
        let lo = ((a as u32).wrapping_add(b as u32)) as i32;
        *self = PPair::pack(hi, lo);
    }
}

impl Add for PPair {
    #[inline(always)]
    fn add(&self, other: PPair) -> PPair {
        let a = *self;
        let b = other;
        let hi = ((a >> 32) as i32).wrapping_add((b >> 32) as i32);
        let lo = ((a as u32).wrapping_add(b as u32)) as i32;
        PPair::pack(hi, lo)
    }
}


pub(crate) trait Pack {
    fn pack(a: i32, b: i32) -> PPair;
}

impl Pack for PPair {
    #[inline(always)]
    fn pack(a: i32, b: i32) -> PPair {
        ((a as i64) << 32) | (b as u32 as i64)
    }
}

pub(crate) trait ToPack{
    fn topack(p: &Pair) -> PPair;
}

impl ToPack for PPair{
    #[inline(always)]
    fn topack(p: &Pair) -> PPair{
        PPair::pack(p.0, p.1)
    }
}


pub(crate) trait AllAround{
    fn allaround(&self, f: &mut dyn FnMut(PPair));
}

macro_rules! ppair {
    ($a:expr, $b:expr) => {
        ((($a as i64) << 32) | (($b as i32 as i64) & 0xFFFF_FFFF))
    };
}

pub const NEIGHBOR_OFFSETS_ALL: [PPair; 9] = [
    ppair!(-1, -1), ppair!(0, -1), ppair!(1, -1),
    ppair!(-1,  0), ppair!(0,  0), ppair!(1,  0),
    ppair!(-1,  1), ppair!(0,  1), ppair!(1,  1),
];

pub const NEIGHBOR_OFFSETS_AROUND: [PPair; 8] = [
    ppair!(-1, -1), ppair!(0, -1), ppair!(1, -1),
    ppair!(-1,  0),                ppair!(1,  0),
    ppair!(-1,  1), ppair!(0,  1), ppair!(1,  1),
];


impl AllAround for PPair{
    fn allaround(&self, f: &mut dyn FnMut(PPair)){

        for offset in NEIGHBOR_OFFSETS_ALL {
            f(self.add(offset));
        }
    }
}


pub(crate) trait Around {
    fn around(&self, f: &mut dyn FnMut(PPair));
}


impl Around for PPair{
    fn around(&self, f: &mut dyn FnMut(PPair)){

        for offset in NEIGHBOR_OFFSETS_AROUND {
            f(self.add(offset));
        }
    }
}


pub(crate) struct Timestamp{
    pub(crate) start: Instant,
}

impl Timestamp{
    pub fn bump(&mut self){
        self.start = Instant::now();
    }
}
pub(crate) trait Stamp{
    fn stamp(&mut self, title: String) -> Duration;
}

impl Stamp for Timestamp{
    fn stamp(&mut self, title: String) -> Duration{
        let now = Instant::now();
        println!("{}: {} ms", title, now.duration_since(self.start).as_millis());
        let duration = now.duration_since(self.start);
        self.start = now;
        duration
    }
}
