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

impl Solver {
    pub fn solve(&self, input: String) -> String {
        match self {
            Solver::Part1 => {
                let input: Vec<(i64, i64)> = input
                    .split("\n")
                    .filter(|line| !line.is_empty())
                    .map(|line| line.split_once(" ").unwrap())
                    .map(|(left, right)| (left.trim(), right.trim()))
                    .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
                    .collect();
                let mut left_paper: Vec<i64> = input.iter().map(|(left, _)| *left).collect();
                let mut right_paper: Vec<i64> = input.iter().map(|(_, right)| *right).collect();
                let mut delta = 0;
                while let Some((left_index, left_value)) =
                    left_paper
                        .iter()
                        .enumerate()
                        .reduce(
                            |(acc_i, acc_e), (i, e)| {
                                if acc_e < e {
                                    (acc_i, acc_e)
                                } else {
                                    (i, e)
                                }
                            },
                        )
                {
                    if let Some((right_index, right_value)) =
                        right_paper
                            .iter()
                            .enumerate()
                            .reduce(
                                |(acc_i, acc_e), (i, e)| {
                                    if acc_e < e {
                                        (acc_i, acc_e)
                                    } else {
                                        (i, e)
                                    }
                                },
                            )
                    {
                        let distance = left_value - right_value;
                        delta += distance.abs();
                        left_paper.swap_remove(left_index);
                        right_paper.swap_remove(right_index);
                    } else {
                        // Panic in case the right paper has more values which
                        // should never happen
                        panic!("The right paper has more values than the left paper");
                    };
                }
                delta.to_string()
            }
            Solver::Part2 => {
                let input: Vec<(i64, i64)> = input
                    .split("\n")
                    .filter(|line| !line.is_empty())
                    .map(|line| line.split_once(" ").unwrap())
                    .map(|(left, right)| (left.trim(), right.trim()))
                    .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
                    .collect();
                let left_paper: Vec<i64> = input.iter().map(|(left, _)| *left).collect();
                let right_paper: Vec<i64> = input.iter().map(|(_, right)| *right).collect();
                let similarity = left_paper.iter().fold(0, |similarity, left_value| {
                    similarity
                        + (left_value
                            * right_paper.iter().fold(0, |count, right_value| {
                                if left_value == right_value {
                                    count + 1
                                } else {
                                    count
                                }
                            }))
                });
                similarity.to_string()
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
