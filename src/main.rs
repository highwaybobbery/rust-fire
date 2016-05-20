extern crate rand;
extern crate ansi_term;

#[derive(Copy, Clone)]
pub enum Tile {
  Empty,
  Tree,
  Burning,
  Heating,
}

const GROW_PROB: f32 = 0.005;
const INITIAL_TREE_PROB: f32 = 0.5;
const FIRE_PROB: f32 = 0.0001;

const FOREST_WIDTH: usize = 60;
const FOREST_HEIGHT: usize = 30;

const MAX_GENERATIONS: u32 = 900;
const SLEEP_MILLIS: u64 = 5;

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Stdout;
use rand::Rng;
use std::process::Command;
use std::time::Duration;
use ansi_term::Colour::*;

use Tile::{Empty, Tree, Burning, Heating};

fn main() {
  let neighbors = [
    (-1,-1), (-1, 0), (-1, 1),
    ( 0,-1),          ( 0, 1),
    ( 1,-1), ( 1, 0), ( 1, 1),
  ];

  let sleep_duration = Duration::from_millis(SLEEP_MILLIS);

  let mut forest = [[Tile::Empty; FOREST_WIDTH]; FOREST_HEIGHT];

  for row in forest.iter_mut() {
    let mut writer = BufWriter::new(io::stdout());
    clear_screen(&mut writer);
    for tree in row.iter_mut() {
      *tree = if prob_check(INITIAL_TREE_PROB) { Tree } else { Empty } ;
      report(*tree, &mut writer)
    }
    writer.write(b"\n");
  }

  for generation in 1..MAX_GENERATIONS {
    for x in 0..FOREST_WIDTH {
      for y in 0..FOREST_HEIGHT {
        let tree = forest[y][x];

        forest[y][x] = match tree {
          Empty => {
            if prob_check(GROW_PROB) == true {
              Tree
            } else {
              tree
            }
          },
          Tree => {
            if prob_check(FIRE_PROB) == true {
              Burning
            } else {
              tree
            }
          },
          Burning => {
            Empty
          },
          Heating => {
            Burning
          },
        };
      }
    }

    let mut writer = BufWriter::new(io::stdout());
    clear_screen(&mut writer);
    writeln!(writer, "------------ Generation: {} ----------------", generation);
    for y in 0..FOREST_HEIGHT {
      for x in 0..FOREST_WIDTH {
        let tree = forest[y][x];
        match tree {
          Burning => {
            for &(xoff, yoff) in neighbors.iter() {
              let nx: i32 = (x as i32) + xoff;
              let ny: i32 = (y as i32) + yoff;
              if nx >= 0 && nx < FOREST_WIDTH as i32 && ny >= 0 && ny < FOREST_HEIGHT as i32 {
                //println!("x:{}, y:{}, nx:{}, ny:{}", x, y, nx, ny);
                match forest[ny as usize][nx as usize] {
                  Tree => { forest[ny as usize][nx as usize] =  Heating },
                  _ => {  },
                }
              }
            }
          },
          _ => { },
        }
        report(tree, &mut writer)
      }

      writer.write(b"\n").unwrap();
    }
    std::thread::sleep(sleep_duration);
  }
}

fn report(tile: Tile, writer: &mut BufWriter<Stdout>) {
  let output = match tile {
    Empty => Black.paint(" "),
    Tree => Green.bold().paint("T"),
    Burning => Red.bold().paint("B"),
    Heating => Yellow.bold().paint("T"),
  };
  write!(writer, "{}", output).unwrap();
}

fn prob_check(chance: f32) -> bool {
  let roll = rand::thread_rng().gen::<f32>();
  if chance - roll > 0.0 { true } else { false }
}

fn clear_screen(writer: &mut BufWriter<Stdout>) {
  let output = Command::new("clear").output().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
  });

  write!(writer, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
}
