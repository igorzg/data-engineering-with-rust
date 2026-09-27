use clap::Parser;
use fruit_salad_maker::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Name Surname <test.test@example.com>",
    about = "Make Fruit Salad"
)]
struct Opts {
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(",").map(|s| s.trim().to_string()).collect()
}

fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains: ");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let fruit_list: Vec<String> = match opts.csvfile {
        Some(filename) => {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(filename);
            let fruits = std::fs::read_to_string(path).expect("Could not read file");
            csv_to_vec(&fruits)
        }
        None => opts
            .fruits
            .unwrap_or_default()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect(),
    };
    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad);
}
