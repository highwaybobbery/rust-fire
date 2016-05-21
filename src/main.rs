extern crate rand;
extern crate ansi_term;

#[derive(Copy, Clone, PartialEq)]
enum State {
  Empty,
  Tree,
  Burning,
  Heating,
}

#[derive(Copy, Clone)]
struct Point {
  x: isize,
  y: isize,
}

#[derive(Copy, Clone)]
struct Tile {
  coords: Point,
  state: State,
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output = match self.state {
      Empty => Black.paint(" "),
      Tree => Green.bold().paint("T"),
      Burning => Red.bold().paint("B"),
      Heating => Yellow.bold().paint("T"),
    };
    write!(f, "{}", output)
  }
}

const GROW_PROB: f32 = 0.01;
const INITIAL_TREE_PROB: f32 = 0.5;
const FIRE_PROB: f32 = 0.001;

const FOREST_WIDTH: usize = 90;
const FOREST_HEIGHT: usize = 45;

const MAX_GENERATIONS: u32 = 1000;
const SLEEP_MILLIS: u64 = 5;

use std::fmt;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Stdout;
use rand::Rng;
use std::process::Command;
use std::time::Duration;
use ansi_term::Colour::*;

use State::{Empty, Tree, Burning, Heating};

fn main() {
  let sleep_duration = Duration::from_millis(SLEEP_MILLIS);

  let mut forest_1 = [[ Tile { coords: Point { x: 0, y: 0 }, state: Empty } ; FOREST_WIDTH]; FOREST_HEIGHT];
  let mut forest_2 = [[ Tile { coords: Point { x: 0, y: 0 }, state: Empty } ; FOREST_WIDTH]; FOREST_HEIGHT];

  generate_forest(&mut forest_1);

  print_forest(forest_1, 0);

  let mut current_forest = &mut forest_1;
  let mut next_forest = &mut forest_2;

  for generation in 1..MAX_GENERATIONS {

    for x in 0..FOREST_WIDTH {
      for y in 0..FOREST_HEIGHT {
        next_forest[y][x].state = update_tree(current_forest[y][x]);
      }
    }

    for x in 0..FOREST_WIDTH {
      for y in 0..FOREST_HEIGHT {
        if next_forest[y][x].state == Burning {
          heat_neighbors(&mut next_forest, y, x);
        }
      }
    }

    print_forest(*next_forest, generation);

    std::mem::swap(&mut current_forest, &mut next_forest);
    std::thread::sleep(sleep_duration);
  }
}

fn generate_forest(forest: &mut [[ Tile; FOREST_WIDTH]; FOREST_HEIGHT]) {
  for row in forest.iter_mut() {
    for tile in row.iter_mut() {
      tile.state = if prob_check(INITIAL_TREE_PROB) { Tree } else { Empty } ;
    }
  }
}

fn update_tree(tree: Tile) -> State{
  match tree.state {
    Empty => {
      if prob_check(GROW_PROB) == true {
        Tree
      } else {
        Empty
      }
    },
    Tree => {
      if prob_check(FIRE_PROB) == true {
        Burning
      } else {
        Tree
      }
    },
    Burning => {
      Empty
    },
    Heating => {
      Burning
    },
  }
}

fn heat_neighbors(forest: &mut [[Tile; FOREST_WIDTH]; FOREST_HEIGHT], y: usize, x: usize) {
  let neighbors = [
    (-1,-1), (-1, 0), (-1, 1),
    ( 0,-1),          ( 0, 1),
    ( 1,-1), ( 1, 0), ( 1, 1),
  ];

  for &(xoff, yoff) in neighbors.iter() {
    let nx: i32 = (x as i32) + xoff;
    let ny: i32 = (y as i32) + yoff;
    if nx >= 0 &&
       nx < FOREST_WIDTH as i32 &&
       ny >= 0 &&
       ny < FOREST_HEIGHT as i32 &&
       forest[ny as usize][nx as usize].state == Tree
    {
      forest[ny as usize][nx as usize].state =  Heating
    }
  }
}

fn print_forest(forest:  [[Tile; FOREST_WIDTH]; FOREST_HEIGHT], generation: u32) {
  let mut writer = BufWriter::new(io::stdout());
  clear_screen(&mut writer);
  writeln!(writer, "Generation: {}", generation + 1).unwrap();
  for row in forest.iter() {
    for tree in row.iter() {
      write!(writer, "{}", tree).unwrap();
    }
    writer.write(b"\n").unwrap();
  }
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
