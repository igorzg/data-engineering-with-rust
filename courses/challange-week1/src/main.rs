//! Challenge week 1 — five data-structure tasks in a single binary.
//!
//! ```text
//! usage: challange-week1 <command> [args]
//!
//!   csv [file] [column]  task 1: min / max / mean of a CSV column
//!   words [file]         task 2: word frequencies in a hash map
//!   top10 [file]         task 3: top 10 words via a sorted linked list
//!   connected            task 4: fully-connected check on demo graphs
//!   kosaraju             task 5: strongly connected components
//!   all                  every task, on the bundled sample data
//! ```

use challange_week1::{csv_stats, graph_connected, kosaraju, word_count, word_list};
use petgraph::graph::{DiGraph, UnGraph};
use std::env;
use std::process::ExitCode;

const DEFAULT_CSV: &str = "data/sales.csv";
const DEFAULT_TEXT: &str = "data/sample.txt";
const DEFAULT_COLUMN: &str = "revenue";
const TOP_N: usize = 10;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let Some(command) = args.first() else {
        print_usage();
        return ExitCode::FAILURE;
    };

    match command.as_str() {
        "csv" => run_csv(&args[1..]),
        "words" => run_words(&args[1..]),
        "top10" => run_top10(&args[1..]),
        "connected" => run_connected(),
        "kosaraju" => run_kosaraju(),
        "all" => {
            let mut failed = false;
            for code in [
                run_csv(&[]),
                run_words(&[]),
                run_top10(&[]),
                run_connected(),
                run_kosaraju(),
            ] {
                failed |= code != ExitCode::SUCCESS;
            }
            if failed {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        other => {
            eprintln!("unknown command: {other}");
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn run_csv(args: &[String]) -> ExitCode {
    let path = arg_or(args, 0, DEFAULT_CSV);
    let column = arg_or(args, 1, DEFAULT_COLUMN);
    println!("== task 1: CSV column statistics — {path}, column `{column}` ==");
    match csv_stats::column_stats(path, column) {
        Ok(stats) => {
            println!(
                "parsed {} rows: min = {:.2}, max = {:.2}, mean = {:.2}",
                stats.count, stats.min, stats.max, stats.mean
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_words(args: &[String]) -> ExitCode {
    let path = arg_or(args, 0, DEFAULT_TEXT);
    println!("== task 2: word frequencies — {path} ==");
    match word_count::word_frequencies_file(path) {
        Ok(frequencies) => {
            let total: usize = frequencies.values().sum();
            println!("{} distinct words, {} tokens", frequencies.len(), total);
            let mut ranked: Vec<(&String, &usize)> = frequencies.iter().collect();
            ranked.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
            for (rank, (word, count)) in ranked.iter().take(TOP_N).enumerate() {
                println!("{:>2}. {word} — {count} times", rank + 1);
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_top10(args: &[String]) -> ExitCode {
    let path = arg_or(args, 0, DEFAULT_TEXT);
    println!("== task 3: top {TOP_N} words as a sorted linked list — {path} ==");
    match word_count::word_frequencies_file(path) {
        Ok(frequencies) => {
            let mut list = word_list::WordList::top_n(&frequencies, TOP_N);
            list.sort_alphabetically();
            println!("linked list (alphabetical order):");
            println!("  {list}");
            println!();
            println!("top {TOP_N} most common words:");
            for (rank, (word, count)) in list.iter().enumerate() {
                println!("{:>2}. {word} — {count} occurrences", rank + 1);
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_connected() -> ExitCode {
    println!("== task 4: fully-connected check ==");

    let mut connected: UnGraph<(), ()> = UnGraph::new_undirected();
    for _ in 0..4 {
        connected.add_node(());
    }
    for (a, b) in [(0u32, 1), (1, 2), (2, 0), (2, 3)] {
        connected.add_edge(a.into(), b.into(), ());
    }

    // Same nodes, but the bridge edge (2, 3) is missing.
    let mut split: UnGraph<(), ()> = UnGraph::new_undirected();
    for _ in 0..4 {
        split.add_node(());
    }
    for (a, b) in [(0u32, 1), (1, 2), (2, 0)] {
        split.add_edge(a.into(), b.into(), ());
    }

    println!(
        "triangle + tail (4 nodes):  is_fully_connected = {}",
        graph_connected::is_fully_connected(&connected)
    );
    println!(
        "triangle + isolated node:   is_fully_connected = {}",
        graph_connected::is_fully_connected(&split)
    );
    ExitCode::SUCCESS
}

fn run_kosaraju() -> ExitCode {
    println!("== task 5: Kosaraju's strongly connected components ==");

    let mut graph = DiGraph::<(), ()>::new();
    for _ in 0..6 {
        graph.add_node(());
    }
    for (a, b) in [(0u32, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 3), (4, 5)] {
        graph.add_edge(a.into(), b.into(), ());
    }

    let components = kosaraju::strongly_connected_components(&graph);
    println!("{} strongly connected component(s):", components.len());
    for component in &components {
        let mut nodes: Vec<usize> = component.iter().map(|node| node.index()).collect();
        nodes.sort_unstable();
        println!(
            "  [{}]",
            nodes
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    ExitCode::SUCCESS
}

fn arg_or<'a>(args: &'a [String], index: usize, default: &'a str) -> &'a str {
    args.get(index).map_or(default, String::as_str)
}

fn print_usage() {
    eprintln!(
        "usage: challange-week1 <command> [args]\n\n\
         commands:\n\
          csv [file] [column]  task 1: min/max/mean of a CSV column (default: {DEFAULT_CSV}, `{DEFAULT_COLUMN}`)\n\
         words [file]          task 2: word frequencies (default: {DEFAULT_TEXT})\n\
         top10 [file]          task 3: top {TOP_N} words via a sorted linked list (default: {DEFAULT_TEXT})\n\
         connected             task 4: connectedness of two demo graphs\n\
         kosaraju              task 5: strongly connected components of a demo graph\n\
         all                   every task on the bundled sample data"
    );
}
