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
                let mut result = 0;
                let mul_regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                for (_, [left, right]) in mul_regex.captures_iter(&input).map(|c| c.extract()) {
                    println!("mul({},{})", left, right);
                    let (left, right): (i64, i64) = (left.parse().unwrap(), right.parse().unwrap());
                    result += left * right;
                }
                result.to_string()
            }
            Solver::Part2 => {
                let mut result = 0;
                // Remove anything between don't() and do()
                let input = format!(
                    "{}{}{}",
                    "do()",
                    input.lines().collect::<Vec<&str>>().join("/* line */"),
                    "don't()",
                );
                let input = input.replace("do()", "\n/* replaced a do */");
                let input = input
                    .lines()
                    .map(|line| {
                        if let Some(index) = line.find("don't()") {
                            format!("{}{}\n", &line[..index], "/* clipped the rest */")
                        } else {
                            line.to_string()
                        }
                    })
                    .collect::<String>();
                let mul_regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                for (_, [left, right]) in mul_regex.captures_iter(&input).map(|c| c.extract()) {
                    // println!("mul({},{})", left, right);
                    let (left, right): (i64, i64) = (left.parse().unwrap(), right.parse().unwrap());
                    result += left * right;
                }
                result.to_string()
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
