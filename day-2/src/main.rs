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

type Level = i64;

type Report = Vec<Level>;

#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

impl From<bool> for Safety {
    fn from(value: bool) -> Self {
        if value {
            Self::Safe
        } else {
            Self::Unsafe
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Decreasing,
    Stable,
    Increasing,
}

impl Safety {
    pub fn assess(report: Report) -> Self {
        // Conditions for safe:
        // the levels are all increasing or decreasing
        // the distance between levels has to be from 1 to 3
        println!("{:?}", report);
        let mut seq = report.iter().zip(report.iter().skip(1));
        let increasing = seq.clone().all(|(a, b)| a < b);
        let decreasing = seq.clone().all(|(a, b)| a > b);
        let preliminary_safety = increasing || decreasing;
        if !preliminary_safety {
            println!("{:?} {:?}", increasing, decreasing);
            return Safety::Unsafe;
        }
        let delta_safe = seq.all(|(a, b)| 1 <= (b - a).abs() && (b - a).abs() <= 3);
        println!("{:?}", delta_safe);
        Safety::from(delta_safe)
    }

    pub fn assess_with_tolerance(report: Report, tolerance: Option<i64>) -> Self {
        // Conditions for safe:
        // the levels are all increasing or decreasing
        // the distance between levels has to be from 1 to 3
        // if one level is bad, it's ok
        println!("\n{:?}", report);
        let seq = report.iter().zip(report.iter().skip(1));
        let increasing = seq.clone().filter(|(a, b)| a < b).count();
        let decreasing = seq.clone().filter(|(a, b)| a > b).count();
        let (direction, preliminary_safety) = match increasing.cmp(&decreasing) {
            std::cmp::Ordering::Less => {
                // This means that the levels are decreasing
                println!("Decreasing {} {}", increasing, decreasing);
                (Direction::Decreasing, increasing <= 1)
            }
            std::cmp::Ordering::Equal => {
                // This is fucked
                (Direction::Stable, false)
            }
            std::cmp::Ordering::Greater => {
                // This means that the levels are increasing
                println!("Increasing {} {}", increasing, decreasing);
                (Direction::Increasing, decreasing <= 1)
            }
        };
        if !preliminary_safety {
            println!("Preliminary Safety: {}", preliminary_safety);
            return Safety::Unsafe;
        }
        let mut skewed_report = report.clone();
        let mut enumerated_skewed_report: Vec<(usize, (i64, i64))> = skewed_report
            .iter()
            .cloned()
            .zip(skewed_report.iter().cloned().skip(1))
            .enumerate()
            .collect();
        let mut tolerance = tolerance.unwrap_or(1);
        let mut current_index = 0;
        while let Some((index, (previous, next))) =
            enumerated_skewed_report.get(current_index).cloned()
        {
            if 1 <= (next - previous).abs()
                && (next - previous).abs() <= 3
                && match direction {
                    Direction::Decreasing => previous > next,
                    Direction::Stable => false,
                    Direction::Increasing => previous < next,
                }
            {
                println!("Within delta range: {} {}", previous, next);
                current_index += 1;
                continue;
            }
            if tolerance < 0 {
                break;
            }
            println!("Reducing tolerance: {} {}", previous, next);
            tolerance -= 1;
            current_index = 0;
            let removed = skewed_report.remove(index);
            println!(
                "Removed report level: {} ({})\n - {:?}",
                index, removed, skewed_report
            );
            enumerated_skewed_report = skewed_report
                .iter()
                .cloned()
                .zip(skewed_report.iter().cloned().skip(1))
                .enumerate()
                .collect();
        }
        println!("Tolerance {}", tolerance);
        Safety::from(tolerance >= 0)
    }
}

impl Solver {
    pub fn solve(&self, input: String) -> String {
        match self {
            Solver::Part1 => {
                let reports: Vec<Report> = input
                    .lines()
                    .map(|line| line.split(' ').collect::<Vec<&str>>())
                    .map(|report| report.iter().map(|level| level.parse().unwrap()).collect())
                    .collect();
                let safety = reports
                    .iter()
                    .map(|report| Safety::assess(report.clone()))
                    .fold(
                        0,
                        |acc, safety| if safety == Safety::Safe { acc + 1 } else { acc },
                    );
                safety.to_string()
            }
            Solver::Part2 => {
                let reports: Vec<Report> = input
                    .lines()
                    .map(|line| line.split(' ').collect::<Vec<&str>>())
                    .map(|report| report.iter().map(|level| level.parse().unwrap()).collect())
                    .collect();
                let safety = reports
                    .iter()
                    .map(|report| Safety::assess_with_tolerance(report.clone(), Some(1)))
                    .fold(
                        0,
                        |acc, safety| if safety == Safety::Safe { acc + 1 } else { acc },
                    );
                safety.to_string()
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
