use core::panic;
use std::{collections::HashMap, fmt::Display};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path of the file to get
    #[arg(short, long)]
    path: std::path::PathBuf,
    /// Solver
    #[arg(short, long, value_enum)]
    solver: Solver,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Solver {
    Part1,
    Part2,
}

const SEARCH_WORD: &'static str = "XMAS";
const SEARCH_X: &'static str = "MAS";

#[rustfmt::skip]
const SEARCH_DIRECTIONS: [Vec2; 8] = [
    Vec2 { x: -1, y: -1 }, Vec2 { x: 0, y: -1 }, Vec2 { x: 1, y: -1 },
    Vec2 { x: -1, y: 0  }, /* --------------- */ Vec2 { x: 1, y: 0  },
    Vec2 { x: -1, y: 1  }, Vec2 { x: 0, y: 1  }, Vec2 { x: 1, y: 1  },
];

#[rustfmt::skip]
const X_DIRECTIONS: [Vec2; 4] = [
    Vec2 { x: -1, y: -1 }, /* --------------- */ Vec2 { x: 1, y: -1 },
    /* ---------------- */ /* --------------- */ /* --------------- */
    Vec2 { x: -1, y: 1  }, /* --------------- */ Vec2 { x: 1, y: 1  },
];

#[derive(Debug)]
struct TileMap {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl TileMap {
    fn get(&self, x: i64, y: i64) -> Option<Tile> {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        let y_offset = y * self.width;
        self.tiles.get(y_offset + x).cloned()
    }

    fn find_word(&self, x: i64, y: i64, search: &str) -> Option<Vec<Word>> {
        let mut words = Vec::new();
        let current_tile = self.get(x, y);
        let current_tile = current_tile.as_ref()?; // return early if no letter at xy pos
        let letter_index_in_word = search.find(|c| c == current_tile.c);
        let letter_index_in_word = letter_index_in_word? as i64;
        let start_offset = 0 - letter_index_in_word;
        let end_offset = (search.len() as i64 - 1) - letter_index_in_word;
        // println!("start_pos {} {} {:?}", x, y, current_tile.c);
        for direction in SEARCH_DIRECTIONS {
            let mut word = "".to_string();
            for offset in start_offset..=end_offset {
                let search_pos = Vec2::new(x + (offset * direction.x), y + (offset * direction.y));
                // println!(
                //     "search_pos {:?} {:?}",
                //     search_pos,
                //     self.get(search_pos.x, search_pos.y)
                // );
                let search_tile = self.get(search_pos.x, search_pos.y);
                match search_tile {
                    Some(tile) => {
                        word.push(tile.c);
                    }
                    None => break, // no point in searching this direction anymore
                }
            }
            if word == search {
                let start = Vec2::new(
                    x + (start_offset * direction.x),
                    y + (start_offset * direction.y),
                );
                let end = Vec2::new(
                    x + (end_offset * direction.x),
                    y + (end_offset * direction.y),
                );
                words.push(Word {
                    start,
                    end,
                    direction,
                });
            }
        }
        Some(words)
    }

    fn find_exes(&self, x: i64, y: i64, search: &str) -> Option<()> {
        let search_length = search.len();
        // if word to search has a even number of characters, it's not
        // searchable as an x word
        if search_length % 2 == 0 {
            eprintln!("Cannot search of X shaped word with an even number of characters");
            return None;
        }
        let half_point_in_word = (search.len() / 2) as i64;
        let start_offset = 0 - half_point_in_word;
        let end_offset = (search.len() as i64 - 1) - half_point_in_word;
        let mut found_word = false;
        for direction in X_DIRECTIONS {
            let mut word = "".to_string();
            for offset in start_offset..=end_offset {
                let search_pos = Vec2::new(x + (offset * direction.x), y + (offset * direction.y));
                let search_tile = self.get(search_pos.x, search_pos.y);
                match search_tile {
                    Some(tile) => {
                        word.push(tile.c);
                    }
                    None => break, // no point in searching this direction anymore
                }
            }
            if word == search {
                if found_word == true {
                    return Some(());
                }
                found_word = true;
            }
        }
        None
    }
}

impl From<String> for TileMap {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.lines().collect();
        let width = { lines.first().unwrap().chars().count() };
        let height = lines.iter().count();
        let tiles = lines
            .iter()
            .flat_map(|line| line.chars().map(|c| Tile::from(c)))
            .collect();
        Self {
            tiles,
            width,
            height,
        }
    }
}

impl Display for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TileMap {}x{}\n{}",
            self.width,
            self.height,
            self.tiles
                .chunks(self.width)
                .map(|row| row
                    .iter()
                    .map(|tile| tile.to_string())
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Tile {
    c: char,
    taken: Vec<usize>,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self {
            c: value,
            taken: Vec::new(),
        }
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        self.c.to_string()
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Direction {
//     Forwards,
//     Reverse,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}
impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Word {
    start: Vec2,
    end: Vec2,
    direction: Vec2,
}

impl Solver {
    pub fn solve(&self, input: String) -> String {
        let map = TileMap::from(input);
        // println!("{}", map);
        match self {
            Solver::Part1 => {
                let mut words: Vec<Word> = Vec::new();
                for y in 0..map.height {
                    for x in 0..map.width {
                        let found_words = map.find_word(x as i64, y as i64, SEARCH_WORD);
                        // todo!("{:?}", found_words);
                        if found_words.is_none() {
                            continue;
                        }
                        let found_words = found_words.unwrap();
                        let found_words: Vec<Word> = found_words
                            .iter()
                            .cloned()
                            .filter(|found_word| {
                                words.iter().find(|word| found_word == *word).is_none()
                            })
                            .collect();
                        if !found_words.is_empty() {
                            words.extend(found_words);
                        }
                    }
                }
                words.len().to_string()
            }
            Solver::Part2 => {
                let mut xes = 0;
                for y in 0..map.height {
                    for x in 0..map.width {
                        let found_xes = map.find_exes(x as i64, y as i64, SEARCH_X);
                        // todo!("{:?}", found_words);
                        if found_xes.is_none() {
                            continue;
                        }
                        let found_xes = found_xes.unwrap();
                        xes += 1;
                    }
                }
                xes.to_string()
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let input = args.path;
    println!("{:?}", input);
    let contents = std::fs::read_to_string(input).unwrap();
    if contents.lines().count() < 64 {
        println!("{}", contents);
    } else {
        println!("Ommitting long contents");
    }
    let result = args.solver.solve(contents);
    println!("{}", result);
}
