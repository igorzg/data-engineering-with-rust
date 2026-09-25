use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@test.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let num_fruits = opts.number;
    let fruits = create_fruit_salad(num_fruits);
    println!("LinkedList Fruit salad: ");
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits, fruits
    )
}
