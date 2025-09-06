// shut up rustc i know there are unneeded parenthesis i used c++ SHDJOIAHDSFPF

use std::collections::{HashSet, HashMap};
use ahash::{AHashSet, AHashMap};
use std::io::{self, Write};
use std::time::{Instant};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::{min, max};

type P8 = (u8, u8);
type P16 = (u16, u16);
type Pf = (f32, f32);
type Pair = (i32, i32);
type Cell = (Pair, i32);

type PPair = i64;

trait Unpack{
    fn unpack(&self) -> Pair;
}
impl Unpack for PPair{
    fn unpack(&self) -> Pair{
        ((self >> 32) as i32, (self & 0xffff_ffff) as u32 as i32) // u32 for signange
    }
}

trait AddASelf{
    fn addaself(&mut self, other: i32);
}

trait AddBSelf{
    fn addbself(&mut self, other: i32);
}

trait AddSelf{
    fn addself(&mut self, other: &PPair);
}

trait AddA{
    fn adda(&self, other: i32) -> PPair;
}
trait AddB{
    fn addb(&self, other: i32) -> PPair;
}
trait Add{
    fn add(&self, other: PPair) -> PPair;
}


impl AddSelf for PPair {
    #[inline(always)]
    fn addself(&mut self, other: &PPair) {
        // extract high 32 as signed i32 (arithmetic shift)
        let a_self = (*self >> 32) as i32;
        let a_other = (*other >> 32) as i32;
        // extract low 32 as signed i32: take low bits as u32 then interpret as i32
        let b_self = (*self as u32) as i32;
        let b_other = (*other as u32) as i32;

        let a_sum = a_self.wrapping_add(a_other);
        let b_sum = b_self.wrapping_add(b_other);

        *self = PPair::pack(a_sum, b_sum);
    }
}

impl Add for PPair {
    #[inline(always)]
    fn add(&self, other: PPair) -> PPair {
        let a = ((*self >> 32) as i32).wrapping_add((other >> 32) as i32);
        let b = ((*self as u32) as i32).wrapping_add((other as u32) as i32);
        PPair::pack(a, b)
    }
}


trait Pack {
    fn pack(a: i32, b: i32) -> PPair;
}

impl Pack for PPair {
    #[inline(always)]
    fn pack(a: i32, b: i32) -> PPair {
        ((a as i64) << 32) | (b as u32 as i64)
    }
}

trait ToPack{
    fn topack(p: &Pair) -> PPair;
}

impl ToPack for PPair{
    #[inline(always)]
    fn topack(p: &Pair) -> PPair{
        PPair::pack(p.0, p.1)
    }
}


struct Timestamp{
    start: Instant,
}

impl Timestamp{
    pub fn bump(&mut self){
        self.start = Instant::now();
    }
}
trait Stamp{
    fn stamp(&mut self, title: String);
}

impl Stamp for Timestamp{
    fn stamp(&mut self, title: String){
        let now = Instant::now();
        println!("{}: {} ms", title, now.duration_since(self.start).as_millis());
        self.start = now;
    }
}

macro_rules! ppair {
    
    ($a:expr, $b:expr) => {
        ((($a as i64) << 32) | (($b as i32 as i64) & 0xFFFF_FFFF))
    };
}

const NEIGHBOR_OFFSETS_ALL: [PPair; 9] = [
    ppair!(-1, -1), ppair!(0, -1), ppair!(1, -1),
    ppair!(-1,  0), ppair!(0,  0), ppair!(1,  0),
    ppair!(-1,  1), ppair!(0,  1), ppair!(1,  1),
];

const NEIGHBOR_OFFSETS_AROUND: [PPair; 8] = [
    ppair!(-1, -1), ppair!(0, -1), ppair!(1, -1),
    ppair!(-1,  0),                ppair!(1,  0),
    ppair!(-1,  1), ppair!(0,  1), ppair!(1,  1),
];


trait AllAround{
    fn allaround(&self, f: &mut dyn FnMut(PPair));
}


impl AllAround for PPair{
    fn allaround(&self, f: &mut dyn FnMut(PPair)){

        for offset in NEIGHBOR_OFFSETS_ALL {
            f(self.add(offset));
        }
    }
}


trait Around {
    fn around(&self, f: &mut dyn FnMut(PPair));
}


impl Around for PPair{
    fn around(&self, f: &mut dyn FnMut(PPair)){

        for offset in NEIGHBOR_OFFSETS_AROUND {
            f(self.add(offset));
        }
    }
}

struct Game{
    cells: AHashSet<PPair>,
    active: AHashSet<PPair>, // coords of cells that needs to be updated
    nmap: AHashMap<PPair, u8>, // map of neighbor counts for cells so don't need to query all neighbors everytime // IMPLEMENT LATER
    bounds: (Pair, Pair), // bottom left, top right
    lifetime: i32,
    seed: ThreadRng,
    ts: Timestamp,
    frametimer: Instant,
    nrcells: (Vec<PPair>, Vec<PPair>), // new cells remov cells
    prevgen: i32,
}

impl Default for Game{
    fn default() -> Self{
        Game{
            cells: AHashSet::new(),
            active: AHashSet::new(),
            nmap: AHashMap::new(),
            bounds: ((-50, -50), (50, 50)),
            lifetime: 0,
            seed: rand::rng(),
            ts: Timestamp{start: Instant::now()},
            frametimer: Instant::now(),
            nrcells: (Vec::new(), Vec::new()),
            prevgen: 0,
        }
    }
}


fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}




impl Game {
    
    pub fn run(&mut self) {
        self.insertglider();
        
        loop{
            self.loopgame();
            std::thread::sleep(std::time::Duration::from_millis(75));
            // break;
        }
        // placeholder
    }
    pub fn loopgame(&mut self) {

        // runs one loop
        self.ts.bump();
        self.processactives();


        // let elapsed = Instant::now() - self.frametimer;
        // if(elapsed.as_millis() >= 200){
        if(true){
            clear_terminal();
            // self.incbounds();

            self.ts.stamp("processactives".to_string());

            self.display();
            self.ts.stamp("display".to_string());

            let nowgen = self.lifetime;
            // println!("gens/s, {} [{} ms]", (nowgen - self.prevgen) * 1000 / elapsed.as_millis() as i32, 
            //     elapsed.as_millis() as i32 / (nowgen - self.prevgen)
            // );

            self.frametimer = Instant::now();
            self.prevgen = nowgen;
        }

        if(self.lifetime % 100 == 1){
            // println!("spitting randommly...");
            self.handleinput();
        }

        self.lifetime += 1;
        
    }
    pub fn insertglider(&mut self){
        let points = [
            (0, 1),
            (1, 0),
            (2, 0),
            (2, 1),
            (2, 2)
        ];
        for p in points{
            self.addcell(PPair::pack(p.0, p.1));
        }
    }
}

impl Game {

    pub fn incbounds(&mut self){
        // return;
        let shift_x = (self.seed.random_range(-5..=5-1), self.seed.random_range(-5..=5)+1);
        let shift_y = (self.seed.random_range(-2..=2-1), self.seed.random_range(-2..=2)+1);
        self.bounds.0.0 += shift_x.0;
        self.bounds.0.1 += shift_y.0;
        self.bounds.1.0 += shift_x.1;
        self.bounds.1.1 += shift_y.1;


        self.bounds.1.1 = max(self.bounds.0.1 + 10, self.bounds.1.1);
        self.bounds.1.0 = max(self.bounds.0.0 + 10, self.bounds.1.0);
    }
    pub fn handleinput(&mut self){
        // placeholder

        let ((min_x, min_y), (max_x, max_y)) = self.bounds;
        for _ in 0..50 {
            let x = self.seed.random_range(min_x..=max_x);
            let y = self.seed.random_range(min_y..=max_y);
            
            if(x < self.bounds.0.0 || x > self.bounds.1.0 || y < self.bounds.0.1 || y > self.bounds.1.1){
                println!("out of bounds generated ogm");
            }

            self.addcell(PPair::pack(x, y));
        }

    }

    
    pub fn debuginfo(&self) -> String{
        format!("Cells: {}, Active: {}, Lifetime: {}, Bounds: {:?}", self.cells.len(), self.active.len(), self.lifetime, self.bounds)
    }

    pub fn addcell(&mut self, cellcoord: PPair){

        if(!self.cells.insert(cellcoord)) {return;} // already live

        self.activearound(&cellcoord);
        cellcoord.around(&mut |c|{
            *self.nmap.entry(c).or_insert(0) += 1;
        });
    }
    pub fn removecell(&mut self, cellcoord: &PPair){

        if(!self.cells.remove(cellcoord)) {return;} // already dead
        self.activearound(cellcoord);
        cellcoord.around(&mut |c|{
            if let Some(v) = self.nmap.get_mut(&c){
                *v -= 1;
                if(*v <= 0){
                    self.nmap.remove(&c);
                }
            }
        })
    }

    pub fn activearound(&mut self, cellcoord: &PPair){
        cellcoord.allaround(&mut |c|{self.active.insert(c);});
    }

    pub fn processactives(&mut self){

        for coord in self.active.drain(){
            
            let curralive: bool = self.cells.contains(&coord);

            let ncnt = *self.nmap.get(&coord).unwrap_or(&0); // can be None if active comes from removed neighbor
            assert!(ncnt <= 8);

            if(curralive){
                if(ncnt < 2 || ncnt > 3){
                    self.nrcells.1.push(coord);
                }
            }
            else{
                if(ncnt == 3){
                    self.nrcells.0.push(coord);
                }
            }

        }

        let toremove = std::mem::take(&mut self.nrcells.1);
        let toadd = std::mem::take(&mut self.nrcells.0);
        for coord in toremove{
            self.removecell(&coord);
        }
        for coord in toadd{
            self.addcell(coord);
        }
        self.nrcells.1.clear();
        self.nrcells.0.clear();

    }

    pub fn mapbs(&self, location: &Pair) -> Pair{
        // maps location in BOUNDS to location on the screen (RESOLUTION)
        let (distx, disty): Pair;
        distx = (self.bounds.1.0 - self.bounds.0.0);
        disty = (self.bounds.1.1 - self.bounds.0.1);

    let (scx, scy): Pair;

    scx = ((location.0 - self.bounds.0.0) * RESOLUTION.0 as i32) / distx;
    scy = ((location.1 - self.bounds.0.1) * RESOLUTION.1 as i32) / disty;

    return (scx, scy);
    }
    pub fn mapsb(&self, scloc: &P16) -> Pair{
        // maps screen loc to BOUNDS location
        let (distx, disty): Pair;
        distx = (self.bounds.1.0 - self.bounds.0.0);
        disty = (self.bounds.1.1 - self.bounds.0.1);

        let (bx, by): Pair;

        bx = (scloc.0 as i32 * distx) / RESOLUTION.0 as i32 + self.bounds.0.0;
        by = (scloc.1 as i32 * disty) / RESOLUTION.1 as i32 + self.bounds.0.1;

        return (bx, by);
    }

    pub fn display(&self){
    for y in 0..RESOLUTION.1{
        for x in 0..RESOLUTION.0{
            let packed = PPair::topack(&self.mapsb(&(x, y)));
            if !self.cells.contains(&packed){
                print!(".");
            }
            else{
                print!("#");
            }
        }
        println!();
    }
    println!(
        "{}",
        self.debuginfo()
    );
    println!();
    println!();
    }


}

const RESOLUTION: P16 = (160, 40); // x width, y height
const DEF_BOUNDS: (Pair, Pair) = ((-20, -5), (20, 5)); // bottom left, top right
const TEST: PPair = ppair!(1, 2);
fn main() {
    println!("Hello, world!");
    // println!("Test pack: {}, unpack: {:?}", TEST, PPair::pack(1,2));
    // for p in NEIGHBOR_OFFSETS_ALL {
    //     let (a, b) = p.unpack();
    //     print!("({},{}) ", a, b);
    // }
    // std::process::exit(0);

    let mut game = Game{
        bounds: DEF_BOUNDS,
        ..Default::default()
    };
    game.run();
}
