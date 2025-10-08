use crate::types::{Pair, PPair, ToPack, 
    Pack, Timestamp, Stamp, Around, AllAround, NEIGHBOR_OFFSETS_AROUND,Add,
    P16, RED, GREEN, BLUE, WHITE, BLACK};


const FADERANDOMNESSEXP: u8 = 2;
const FADERANDOMNESS: u8 = 1 << FADERANDOMNESSEXP;


use crate::RESOLUTION;
use std::time::Instant;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::io::{self, Write};
use rayon::prelude::*;
use dashmap::DashMap;

use std::collections::{HashSet, HashMap};
use ahash::{AHashSet, AHashMap};
use std::cmp::{min, max};


fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
pub(crate) struct Game{
    pub(crate) cells: AHashSet<PPair>,
    pub(crate) active: AHashSet<PPair>, // coords of cells that needs to be updated
    pub(crate) nmap: AHashMap<PPair, u8>, // map of neighbor counts for cells so don't need to query all neighbors everytime // IMPLEMENT LATER
    pub(crate) bounds: (Pair, Pair), // bottom left, top right
    pub(crate) lifetime: i32,
    pub(crate) ts: Timestamp,
    pub(crate) seed: ThreadRng,
    pub(crate) frametimer: Instant,
    pub(crate) nrcells: (Vec<PPair>, Vec<PPair>), // new cells remov cells
    pub(crate) prevgen: i32,
}

impl Default for Game{
    fn default() -> Self{
        Game{
            cells: AHashSet::new(),
            active: AHashSet::new(),
            nmap: AHashMap::new(),
            bounds: ((-50, -50), (50, 50)),
            lifetime: 0,
            seed: rand::thread_rng(),
            ts: Timestamp{start: Instant::now()},
            frametimer: Instant::now(),
            nrcells: (Vec::new(), Vec::new()),
            prevgen: 0,
        }
    }
}

impl Game {
    
   
    pub fn loopgame(&mut self, resolution: &P16) {

        // runs one loop
        self.ts.bump();
        self.processactives();


        let elapsed = Instant::now() - self.frametimer;
        
        if(self.lifetime % 100 == 1){
            // println!("spitting randommly...");
            self.handleinput();
        }
        if(elapsed.as_millis() >= 200){
        // if(true){
            clear_terminal();
            // self.incbounds(); 
            self.ts.stamp("processactives".to_string());

            self.display(resolution);
            self.ts.stamp("display".to_string());

            let nowgen = self.lifetime;
            println!("gens/s, {} [{} ms]", (nowgen - self.prevgen) * 1000 / elapsed.as_millis() as i32, 
                elapsed.as_millis() as i32 / (nowgen - self.prevgen)
            );

            self.frametimer = Instant::now();
            self.prevgen = nowgen;
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
        for _ in 0..4000 {
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
        cellcoord.around(&mut |c| {
            let v = self.nmap.entry(c).or_insert(0);
            *v = v.saturating_add(1).min(8); // clamp at 8 (no point > 8)
        });
    }
    pub fn removecell(&mut self, cellcoord: &PPair){

        if(!self.cells.remove(cellcoord)) {return;} // already dead
        self.activearound(cellcoord);
        cellcoord.around(&mut |c| {
            if let Some(v) = self.nmap.get_mut(&c) {
                *v = v.saturating_sub(1);
                if *v == 0 {
                    self.nmap.remove(&c);
                }
            }
        });
    }


    pub fn activearound(&mut self, cellcoord: &PPair){
        cellcoord.allaround(&mut |c|{self.active.insert(c);});
    }

    pub fn advancelife(&mut self){
        self.lifetime += 1;
    }
    pub fn processactives_old(&mut self){

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
        self.advancelife();

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
        let mut deltas: AHashMap<PPair, i8> = AHashMap::new();
        deltas.reserve((toremove.len() + toadd.len()) << 3);
        
        // calculate adding and removing deltas

        for coord in toremove.iter(){
            self.cells.remove(coord);
            self.active.insert(*coord);
            for offset in NEIGHBOR_OFFSETS_AROUND {
                let nb = coord.add(offset);
                let entry = deltas.entry(nb).or_insert(0);
                *entry -= 1;
            }
        }
        for coord in toadd.iter(){
            self.active.insert(*coord);
            self.cells.insert(*coord);
            for offset in NEIGHBOR_OFFSETS_AROUND {
                let nb = coord.add(offset);
                let entry = deltas.entry(nb).or_insert(0);
                *entry += 1;
            }
        }

        deltas.drain().for_each(|entry|{
            self.active.insert(entry.0);
            if(entry.1 == 0){
                return;
            }
            let v = self.nmap.entry(entry.0).or_insert(0);
            *v = (*v as i8 + entry.1) as u8; // clamp at 8 (no point > 8)
            if *v == 0 {
                self.nmap.remove(&entry.0);
            }
        });

        self.nrcells.1.clear();
        self.nrcells.0.clear();
        self.advancelife();

    }

    pub fn mapbs(&self, location: &Pair, resolution: &P16) -> Pair{
        // maps location in BOUNDS to location on the screen (RESOLUTION)
        let (distx, disty): Pair;
        distx = (self.bounds.1.0 - self.bounds.0.0);
        disty = (self.bounds.1.1 - self.bounds.0.1);
 
    let (scx, scy): Pair;

    scx = ((location.0 - self.bounds.0.0) * resolution.0 as i32) / distx;
    scy = ((location.1 - self.bounds.0.1) * resolution.1 as i32) / disty;

    return (scx, scy);
    }
    pub fn mapsb(&self, scloc: P16, resolution: &P16) -> Pair{
        // maps screen loc to BOUNDS location
        let (distx, disty): Pair;
        distx = (self.bounds.1.0 - self.bounds.0.0);
        disty = (self.bounds.1.1 - self.bounds.0.1);

        let (bx, by): Pair;

        bx = (scloc.0 as i32 * distx) / resolution.0 as i32 + self.bounds.0.0;
        by = (scloc.1 as i32 * disty) / resolution.1 as i32 + self.bounds.0.1;

        return (bx, by);
    }

    pub fn display(&self, resolution: &P16){
    for y in 0..resolution.1{
        for x in 0..resolution.0{
            let packed = PPair::topack(&self.mapsb((x, y), resolution));
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

    pub fn draw(&self, frame: &mut [u8], paused: bool, fades: &mut Vec<Vec<i8>>, resolution: &P16) {
        let mut x = 0;
        let mut y = 0;
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {


            let fadespos = &mut fades[y as usize][x as usize];

            let packed = PPair::topack(&self.mapsb((x, y), resolution));
            let color = if self.cells.contains(&packed) {
                *fadespos = 100 + (rand::random::<u8>() % 4) as i8;
                WHITE
            } else {
                if true {//  !paused {
                    *fadespos = max(0, *fadespos - 1);
                }
                // BLACK
                
                [BLUE[0], BLUE[1], BLUE[2], ((*fadespos as u8 * 10) as u8).min(255)]
            };
            x += 1;
            if(x >= RESOLUTION.0){
                x = 0;
                y += 1;
            }
            pixel.copy_from_slice(&color);
        }
    }

}
impl Game {
    /// Optimized draw: rasterize live cells to a dense screen buffer then paint.
    pub fn draw_optimized(&mut self, frame: &mut [u8], paused: bool, fades: &mut Vec<Vec<i8>>) {
        let resx = RESOLUTION.0 as usize;
        let resy = RESOLUTION.1 as usize;
        let screen_size = resx * resy;

        // 1) prepare screen occupancy buffer (0 = empty, 1 = live)
        // reuse a Vec<u8> allocated once (here we create; you can store it in Game to reuse)
        let mut screen: Vec<bool> = vec![false; screen_size];

        // precompute bounds -> scaling factors (use integer-safe mapping)
        let (min_x, min_y) = self.bounds.0;
        let (max_x, max_y) = self.bounds.1;
        let distx = (max_x - min_x).max(1);
        let disty = (max_y - min_y).max(1);


        // Using integer arithmetic: scx = ((x - min_x) * resx) / distx
        // rasterize each live cell once
        for &pp in &self.cells {
            // unpack inline
            let wx = (pp >> 32) as i32;
            let wy = (pp & 0xffff_ffff) as u32 as i32;

            // map world -> screen (clamp)
            let sx = (((wx - min_x) * resx as i32) / distx);
            let sy = (((wy - min_y) * resy as i32) / disty);

            let nsx = (((wx - min_x + 1) * resx as i32) / distx);
            let nsy = (((wy - min_y + 1) * resy as i32) / disty);

            if sx >= 0 && sy >= 0 && (sx as usize) < resx && (sy as usize) < resy {


                let idx = (sy as usize) * resx + (sx as usize);
                for y in sy..nsy{
                    for x in sx..nsx{
                        screen[(y as usize) * resx + (x as usize)] = true;
                    }
                }
                // screen[idx] = true;
            }
        }

        // 2) Now fill frame once using the dense screen buffer
        // iterate in the same order you do pixel writing
        let mut idx: usize = 0;
        for y in 0..resy {
            for x in 0..resx {
                let fadespos = &mut fades[y][x];
                let buffer_val = screen[idx]; // 0 or 1

                let color: [u8;4] = if buffer_val {
                    // live cell: set fade and color
                    let random = rand::random::<u8>();
                    *fadespos = 100 + ((random & (FADERANDOMNESS - 1)) as i8);
                    WHITE
                } else {
                    // background: decay fade
                    // if !paused {
                    if true{
                        *fadespos = max(0, *fadespos - 1);
                    }
                    // randomly pick blue, green, or red
                    // match rand::random::<u8>() % 3  {
                    //     0 => [BLUE[0], BLUE[1], BLUE[2], ((*fadespos as u8 * 10) as u8).min(255)],
                    //     1 => [GREEN[0], GREEN[1], GREEN[2], ((*fadespos as u8 * 10) as u8).min(255)],
                    //     _ => [RED[0], RED[1], RED[2], ((*fadespos as u8 * 10) as u8).min(255)],
                    // }

                    [RED[0], RED[1], RED[2], ((*fadespos as u8 * 10) as u8).min(255)]
                };

                let fidx = idx * 4;
                frame[fidx..fidx+4].copy_from_slice(&color);
                idx += 1;
            }
        }
    }
}