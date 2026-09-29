use clap::Parser;
use polars::prelude::*;
const CSV_FILE: &str = "data/global-life-expt-2022.csv";

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "ig",
    about = "command line tool that reads csv"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Print {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long, default_value = "10")]
        rows: usize,
    },
    Describe {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Schema {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Shape {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Sort {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long, default_value = "2020")]
        year: String,
        #[clap(long, default_value = "10")]
        rows: usize,
        #[clap(long, default_value = "true")]
        order: bool,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Print { path, rows }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.head(Some(rows)));
        }
        Some(Commands::Describe { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df);
        }
        Some(Commands::Schema { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.schema());
        }
        Some(Commands::Shape { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.shape());
        },
        Some(Commands::Sort { path, year, rows, order }) => {
            let df = polarsdf::read_csv(&path);
            let country_column_name = "Country Name";

            let df2 = df
                .select([country_column_name, &year])
                .unwrap()
                .drop_nulls::<String>(None)
                .unwrap()
                .sort(
                    [&year],
                    SortMultipleOptions::new().with_order_descending(!order),
                )
                .unwrap();

            println!("{:?}", df2.head(Some(rows)));
        },
        None => {
            println!("No subcommand was used");
        }
    }
}
