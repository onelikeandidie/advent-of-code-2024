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

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderRule((i64, i64));
#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderRuleList(Vec<OrderRule>);

impl From<String> for OrderRuleList {
    fn from(value: String) -> Self {
        Self(
            value
                .lines()
                .filter(|line| line.contains("|"))
                .map(|pair| pair.split_once("|"))
                .filter_map(|pair| pair)
                .map(|(left, right)| OrderRule((left.parse().unwrap(), right.parse().unwrap())))
                .collect(),
        )
    }
}

impl ToString for OrderRuleList {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|OrderRule((left, right))| format!("{}|{}", left, right))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

type Pages = Vec<i64>;

impl Solver {
    pub fn solve(&self, input: String) -> String {
        match self {
            Solver::Part1 => {
                let mut result = 0;
                let rules = OrderRuleList::from(input.clone());
                // println!("{}", rules.to_string());
                let page_lists: Vec<Pages> = input
                    .lines()
                    .filter(|line| !line.contains("|") && !line.is_empty())
                    .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
                    .collect();
                for page_list in page_lists {
                    // println!(
                    //     "{}",
                    //     pages
                    //         .iter()
                    //         .map(|page| page.to_string())
                    //         .collect::<Vec<String>>()
                    //         .join(",")
                    // );
                    println!("Pages {:?}", page_list);
                    let mut correct = true;
                    for (index, page) in page_list.iter().enumerate() {
                        for (other_index, other_page) in page_list.iter().enumerate() {
                            if page == other_page {
                                continue;
                            }
                            if index > other_index {
                                continue;
                            }
                            println!("Comparing {} with {}", page, other_page);
                            for OrderRule((left, right)) in rules.0.iter() {
                                if page == left && other_page == right {
                                    // We are good
                                    println!("- Rule {} -> {}", left, right);
                                    println!("- Correct Order");
                                }
                                if page == right && other_page == left {
                                    correct = false;

                                    println!("- Rule {} -> {}", left, right);
                                    println!("- Incorrect Order");
                                    break;
                                }
                            }
                            if !correct {
                                break;
                            }
                        }
                    }
                    if correct {
                        // Get middle number
                        let index = page_list.len() / 2;
                        println!("Getting {} for page {:?}", index, page_list);
                        let middle_page = page_list.get(index);
                        if let Some(page) = middle_page {
                            result += page;
                        }
                    }
                }

                result.to_string()
            }
            Solver::Part2 => {
                let mut result = 0;
                let rules = OrderRuleList::from(input.clone());
                // println!("{}", rules.to_string());
                let page_lists: Vec<Pages> = input
                    .lines()
                    .filter(|line| !line.contains("|") && !line.is_empty())
                    .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
                    .collect();
                for page_list in page_lists {
                    // println!(
                    //     "{}",
                    //     pages
                    //         .iter()
                    //         .map(|page| page.to_string())
                    //         .collect::<Vec<String>>()
                    //         .join(",")
                    // );
                    println!("Pages {:?}", page_list);
                    let mut correct = true;
                    for (index, page) in page_list.iter().enumerate() {
                        for (other_index, other_page) in page_list.iter().enumerate() {
                            if page == other_page {
                                continue;
                            }
                            if index > other_index {
                                continue;
                            }
                            println!("Comparing {} with {}", page, other_page);
                            for OrderRule((left, right)) in rules.0.iter() {
                                if page == left && other_page == right {
                                    // We are good
                                    println!("- Rule {} -> {}", left, right);
                                    println!("- Correct Order");
                                }
                                if page == right && other_page == left {
                                    correct = false;

                                    println!("- Rule {} -> {}", left, right);
                                    println!("- Incorrect Order");
                                    break;
                                }
                            }
                            if !correct {
                                break;
                            }
                        }
                    }
                    if correct {
                        // Get middle number
                        let index = page_list.len() / 2;
                        println!("Getting {} for page {:?}", index, page_list);
                        let middle_page = page_list.get(index);
                        if let Some(page) = middle_page {
                            result += page;
                        }
                    }
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
