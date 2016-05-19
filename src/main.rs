extern crate rand;

#[derive(Copy, Clone)]
pub enum Tile {
  Empty,
  Tree,
  Burning,
}

const GROW_PROB: u32 = 10;
const INITIAL_TREE_PROB: u32 = 50;
const FIRE_PROB: u32 = 5;
const MAX_GENERATIONS: u32 = 30;

use Tile::{Empty, Tree, Burning};
use rand::Rng;
use std::cmp::Ordering;

fn main() {

  let mut tile = if prob_check(INITIAL_TREE_PROB) { Tree } else { Empty } ;

  report(tile);

  for _ in 1..MAX_GENERATIONS {
    tile = match tile {
      Empty => {
        if prob_check(GROW_PROB) == true {
          Tree
        } else {
          tile
        }
      },
      Tree => {
        if prob_check(FIRE_PROB) == true {
          Burning
        } else {
          tile
        }
      }
      _ => tile,
    };

    report(tile);
  }

}

fn report(tile: Tile) {
  match tile {
    Empty => println!("no tree"),
    Tree => println!("tree"),
    Burning => println!("fire!"),
  }
}

fn prob_check(chance: u32) -> bool {
  let roll: u32 = rand::thread_rng().gen_range(1, 101);
  if roll.cmp(&chance) == Ordering::Less { true } else { false }
}


