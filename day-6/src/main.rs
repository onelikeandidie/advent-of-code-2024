use core::panic;
use std::{
    fmt::Display,
    sync::{mpsc, Arc, Mutex},
    thread,
};

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
    // Threads for the part 2 multi-threaded brute force solver
    #[arg(short, long, default_value_t = 1)]
    threads: usize,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Solver {
    Part1,
    Part2,
    Part2MultiThread,
}

const GUARD_DIRECTION_CHARS: [char; 4] = ['^', '>', 'v', '<'];

#[derive(Debug, Clone)]
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

    fn get_mut(&mut self, x: i64, y: i64) -> Option<&mut Tile> {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        let y_offset = y * self.width;
        self.tiles.get_mut(y_offset + x)
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
    pub fn solve(&self, input: String, thread_count: usize) -> String {
        let map = TileMap::from(input);
        if map.height > 32 {
            println!("Map height too large, ommiting output");
        } else {
            println!("{}", map);
        }
        match self {
            Solver::Part1 => {
                let mut map = map.clone();
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
                // The second part, we have to put a temporary obstacle on
                // the map and check if the guard loops. If it loops, add it
                // to the list
                // Create a map of all spots
                let spots_to_check = (0..map.height)
                    .map(|y| {
                        (0..map.width)
                            .map(|x| (x as i64, y as i64))
                            .collect::<Vec<(i64, i64)>>()
                    })
                    .flatten()
                    .map(|(x, y)| Vec2::new(x, y))
                    // Get all the spots that are not occupied with an obstacle
                    .filter(|pos| {
                        !map.get(pos.x, pos.y)
                            .expect(format!("Could not get tile at pos: {:?}", pos).as_str())
                            .obstacle
                    })
                    .filter(|pos| &map.guard.position != pos)
                    .collect::<Vec<Vec2>>();
                let mut loop_spots = Vec::new();
                let mut iter = spots_to_check.iter();
                while let Some(spot_to_check) = iter.next() {
                    let mut visited_positions = Vec::new();
                    println!("Checking for position: {:?}", spot_to_check);
                    let mut map = map.clone();
                    {
                        let modified_tile = map.get_mut(spot_to_check.x, spot_to_check.y).unwrap();
                        modified_tile.obstacle = true;
                    }

                    loop {
                        let tile_in_front = map.get(
                            map.guard.position.x + map.guard.looking_at.x,
                            map.guard.position.y + map.guard.looking_at.y,
                        );
                        if tile_in_front.is_none() {
                            break;
                        }
                        let tile_in_front = tile_in_front.unwrap();
                        if !visited_positions.contains(&(map.guard.position, map.guard.looking_at))
                        {
                            visited_positions.push((map.guard.position, map.guard.looking_at));
                        } else {
                            // This should mean the guard has looped
                            println!("This looped");
                            loop_spots.push(spot_to_check);
                            break;
                        }
                        if tile_in_front.obstacle {
                            map.guard.turn_right();
                        } else {
                            map.guard.position.x += map.guard.looking_at.x;
                            map.guard.position.y += map.guard.looking_at.y;
                        }
                    }
                }
                println!("{}", map);
                loop_spots.len().to_string()
            }
            Solver::Part2MultiThread => {
                // The second part, we have to put a temporary obstacle on
                // the map and check if the guard loops. If it loops, add it
                // to the list
                // Create a map of all spots
                let spots_to_check = (0..map.height)
                    .map(|y| {
                        (0..map.width)
                            .map(|x| (x as i64, y as i64))
                            .collect::<Vec<(i64, i64)>>()
                    })
                    .flatten()
                    .map(|(x, y)| Vec2::new(x, y))
                    // Get all the spots that are not occupied with an obstacle
                    .filter(|pos| {
                        !map.get(pos.x, pos.y)
                            .expect(format!("Could not get tile at pos: {:?}", pos).as_str())
                            .obstacle
                    })
                    .filter(|pos| &map.guard.position != pos)
                    .collect::<Vec<Vec2>>();
                // Setup communication between spawned threads and the main thread
                let (tx, rx) = mpsc::channel();
                // Make the result a shared memory mutex so it can be shared across threads
                // let loop_spots: Arc<Mutex<Vec<Vec2>>> = Arc::new(Mutex::new(Vec::new()));
                let spots_count = spots_to_check.len();
                // Divide the spots into equal amounts to use in each thread
                // To avoid a number of chunks bigger than threads, we divide
                // by +1 to make sure we don't have an extra chunk with 1 element
                let chunks = spots_to_check
                    .chunks(spots_count / thread_count + 1)
                    .map(|chunk| Vec::from(chunk));
                for (thread_index, chunk) in chunks.enumerate() {
                    // Clone variables to use inside thread
                    let map = map.clone();
                    let chunk = chunk.clone();
                    let tx = tx.clone();
                    thread::spawn(move || {
                        println!("Thead {}", thread_index);
                        let mut loop_spots: Vec<Vec2> = Vec::new();
                        let mut iter = chunk.iter();
                        while let Some(spot_to_check) = iter.next() {
                            println!("Checking for position: {:?}", spot_to_check);
                            let mut map = map.clone();
                            {
                                let modified_tile =
                                    map.get_mut(spot_to_check.x, spot_to_check.y).unwrap();
                                modified_tile.obstacle = true;
                            }
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
                                if !visited_positions
                                    .contains(&(map.guard.position, map.guard.looking_at))
                                {
                                    visited_positions
                                        .push((map.guard.position, map.guard.looking_at));
                                } else {
                                    println!("This looped");
                                    loop_spots.push(spot_to_check.clone());
                                    break;
                                }
                                if tile_in_front.obstacle {
                                    map.guard.turn_right();
                                } else {
                                    map.guard.position.x += map.guard.looking_at.x;
                                    map.guard.position.y += map.guard.looking_at.y;
                                }
                            }
                        }
                        tx.send(loop_spots.len()).unwrap();
                    });
                }
                // Wait for all threads to finish processing
                let mut loop_spots = 0;
                for _ in 0..thread_count {
                    let result = rx.recv().unwrap();
                    loop_spots += result;
                }
                loop_spots.to_string()
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let input = args.path;
    println!("{:?}", input);
    let contents = std::fs::read_to_string(input).unwrap();
    if contents.lines().count() > 32 {
        println!("Input contents too long, ommitting output");
    } else {
        println!("{}", contents);
    }
    let result = args.solver.solve(contents, args.threads);
    println!("{}", result);
}
