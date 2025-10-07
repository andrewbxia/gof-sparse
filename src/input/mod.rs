


use crate::types::{Pair, PPair, ToPack, Unpack, Stamp, Pack};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};




pub(crate) fn readfilecoord(filename: &str) -> Vec<PPair> {
    let file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    BufReader::new(file)
        .read_to_string(&mut contents)
        .expect("Unable to read file");

    contents
        .split(|c| c == '\n' || c == '\r')
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok());
            let a = nums.next()?;
            let b = nums.next()?;
            Some(PPair::pack(a, b))
        })
        .collect()
}

const BLANK: char = '.';
const FILLED: char = '*';

pub(crate) fn readfilegrid(filename: &str) -> Vec<PPair> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut cells = Vec::new();
    let mut ycnt = 0;
    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        let mut xcnt: i32 = 0;
        for (x, ch) in line.chars().enumerate() {
            if(ch != FILLED && ch != BLANK){
                continue; // ignore unknown chars
            }
            if ch != BLANK {
                cells.push(PPair::pack(xcnt as i32, ycnt as i32));
            }
            xcnt += 1;
        }
        if(xcnt == 0){
            ycnt += 50;
        }
        else{
            ycnt += 1;
        }
    }

    cells
}