use polars::prelude::*;
use std::fs::File;
use std::io::{BufReader};

pub fn read_csv(path: &str) -> DataFrame {
    let file = File::open(path).unwrap();
    CsvReader::new(BufReader::new(file)).finish().unwrap()
}

pub fn print_df(df: &DataFrame, n: usize) {
    println!("{:?}", df.head(Some(n)));
}

pub fn print_schema(df: &DataFrame) {
    println!("{:?}", df.schema());
}

pub fn print_shape(df: &DataFrame) {
    println!("{:?}", df.shape());
}