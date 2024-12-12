use core::panic;
use std::fmt::Display;

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

const GUARD_DIRECTION_CHARS: [char; 4] = ['^', '>', 'v', '<'];

#[derive(Debug)]
struct TileMap {
    tiles: Vec<Tile>,
    // obstacles: Vec<Obstacle>,
    guard: Guard,
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
        let guard_char_index = lines
            .join("")
            .find(|c: char| GUARD_DIRECTION_CHARS.contains(&c))
            .expect("There was no guard");
        let guard_x = guard_char_index % width;
        let guard_y = (guard_char_index - guard_x) / width;
        let guard_char = lines
            .join("")
            .chars()
            .skip(guard_char_index)
            .take(1)
            .next()
            .unwrap();
        let mut guard = Guard::from(guard_char);
        guard.position = Vec2::new(guard_x as i64, guard_y as i64);
        println!("{:?}", guard);

        Self {
            tiles,
            width,
            height,
            guard,
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
            (0..self.height)
                .map(|y| {
                    (0..self.width)
                        .map(|x| {
                            if self.guard.position.x == x as i64
                                && self.guard.position.y == y as i64
                            {
                                return self.guard.to_string();
                            }
                            self.get(x as i64, y as i64).unwrap_or_default().to_string()
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Guard {
    position: Vec2,
    looking_at: Vec2,
}

impl Guard {
    fn turn_right(&mut self) {
        self.looking_at = match self.looking_at {
            Vec2 { x: 0, y: -1 } => Vec2 { x: 1, y: 0 },
            Vec2 { x: 1, y: 0 } => Vec2 { x: 0, y: 1 },
            Vec2 { x: 0, y: 1 } => Vec2 { x: -1, y: 0 },
            Vec2 { x: -1, y: 0 } => Vec2 { x: 0, y: -1 },
            _ => panic!("AAAAAAA"),
        };
    }
}

impl From<char> for Guard {
    fn from(value: char) -> Self {
        Self {
            position: Vec2::new(0, 0),
            looking_at: match value {
                '^' => Vec2::new(0, -1),
                '>' => Vec2::new(1, 0),
                'v' => Vec2::new(0, 1),
                '<' => Vec2::new(-1, 0),
                _ => panic!("Invalid char for guard direction {}", value),
            },
        }
    }
}

impl ToString for Guard {
    fn to_string(&self) -> String {
        match self.looking_at {
            Vec2 { x: 0, y: -1 } => '^',
            Vec2 { x: 1, y: 0 } => '>',
            Vec2 { x: 0, y: 1 } => 'v',
            Vec2 { x: -1, y: 0 } => '<',
            _ => todo!(),
        }
        .to_string()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Tile {
    obstacle: bool,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self {
            obstacle: value == '#',
        }
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self.obstacle {
            true => '#',
            false => '.',
        }
        .to_string()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Solver {
    pub fn solve(&self, input: String) -> String {
        match self {
            Solver::Part1 => {
                let mut map = TileMap::from(input);
                println!("{}", map);
                let mut visited_positions = Vec::new();
                loop {
                    let tile_in_front = map.get(
                        map.guard.position.x + map.guard.looking_at.x,
                        map.guard.position.y + map.guard.looking_at.y,
                    );
                    if tile_in_front.is_none() {
                        break;
                    }
                    let tile_in_front = tile_in_front.unwrap();
                    if !visited_positions.contains(&map.guard.position) {
                        visited_positions.push(map.guard.position);
                    }
                    if tile_in_front.obstacle {
                        map.guard.turn_right();
                    } else {
                        map.guard.position.x += map.guard.looking_at.x;
                        map.guard.position.y += map.guard.looking_at.y;
                    }
                }
                println!("{}", map);
                (visited_positions.len() + 1).to_string()
            }
            Solver::Part2 => {
                let mut map = TileMap::from(input);
                println!("{}", map);
                let mut visited_positions = Vec::new();
                loop {
                    let tile_in_front = map.get(
                        map.guard.position.x + map.guard.looking_at.x,
                        map.guard.position.y + map.guard.looking_at.y,
                    );
                    if tile_in_front.is_none() {
                        break;
                    }
                    let tile_in_front = tile_in_front.unwrap();
                    if !visited_positions.contains(&map.guard.position) {
                        visited_positions.push(map.guard.position);
                    }
                    if tile_in_front.obstacle {
                        map.guard.turn_right();
                    } else {
                        map.guard.position.x += map.guard.looking_at.x;
                        map.guard.position.y += map.guard.looking_at.y;
                    }
                }
                println!("{}", map);
                (visited_positions.len() + 1).to_string()
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let input = args.path;
    println!("{:?}", input);
    let contents = std::fs::read_to_string(input).unwrap();
    println!("{}", contents);
    let result = args.solver.solve(contents);
    println!("{}", result);
}
