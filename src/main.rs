extern crate termion;
extern crate atty;
#[macro_use]
extern crate structopt;

use std::io;
use std::io::BufRead;
use std::iter::repeat;


#[derive(Debug, Clone, Copy)]
struct ExecutionContext {
    is_stdout_tty: bool,
    out_stream_width: Option<usize>,
}

impl ExecutionContext {
    fn new() -> ExecutionContext {
        let is_stdout_tty = atty::is(atty::Stream::Stdout);

        ExecutionContext{
            is_stdout_tty,
            out_stream_width: match is_stdout_tty {
                true => Some(
                    termion::terminal_size().unwrap().0 as usize),
                false => None,
            }
        }
    }
}

const COLUMN_SEP: &str = " ";


fn analyze(lines: &Vec<String>) -> Vec<usize>{
    let mut columns_to_lengths: Vec<Vec<usize>> = Vec::new();

    for line in lines {
        let column_value_lenghts = line
            .trim()
            .split(COLUMN_SEP)
            .map(|substr| substr.chars().count())
            .collect::<Vec<usize>>();

        if columns_to_lengths.len() < column_value_lenghts.len() {
            columns_to_lengths
                .resize_with(column_value_lenghts.len(), || Vec::new())
        }  //or abort due to malformed tsv

        for (i, col_value_length) in column_value_lenghts.iter().enumerate() {
            columns_to_lengths[i].push(*col_value_length)
        };
    };

    columns_to_lengths.into_iter()
        .map(|v| v.iter().max().unwrap().clone())
        .collect::<Vec<_>>()
}


fn split_proportionally(max_lengths: &Vec<usize>, available_width: usize) -> Vec<usize> {
    let width_to_alloc = (available_width - max_lengths.len()) as f64;
    let max_len_sum = max_lengths.iter().sum::<usize>() as f64;
//    let chars_required_for_full_display = max_lengths.iter().sum::<usize>() + max_lengths.len() - 1;

//    if chars_required_for_full_display < available_width {
//        return max_lengths.clone()
//    }

    let res = max_lengths.iter().map(
        |&l| {
        (width_to_alloc * (l as f64) / max_len_sum ).floor() as usize
        }).collect();
    println!("got lengths {:?}", &max_lengths);
    println!("returning   {:?}", &res);
    res
}

fn format_line(_line: String, split_info: &Vec<usize>)-> String {
    _line.trim()
        .split(COLUMN_SEP)
        .zip(split_info.iter())
        .map(|(col_value, width_to_fill)|{
            let chars = col_value
                .chars()
                .chain(repeat(' '));

            if col_value.len() - 1 <= *width_to_fill {
                chars
                    .take(*width_to_fill-1)
                    .chain("|".chars())
                    .collect::<String>()
            } else {
                chars
                    .take(*width_to_fill-2)
                    .chain("*".chars())
                    .chain("|".chars())
                    .collect::<String>()
            }
        })
        .collect()
}

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tab", about = "
Formatting and padding utility for tabular data.

This program:
- reads and caches n first lines (default n=50),
- calculates columns widths necessary for pretty printing,
- prints cached lines and any all consecutive ones.

Tab switches between two modes:
 - printer (when there's a terminal serving as stdout device),
 - formatter (otherwise - when output is e.g. piped or redirected to file).
")]
struct Args{
    /// Column separator char
    #[structopt(short = "s", long = "separator", default_value = "\t")]
    sep: char,

    /// Padding character
    #[structopt(short = "p", long = "padding", default_value = " ")]
    padding: char,

    /// Cache n first lines to calculate columns' widths
    #[structopt(short = "n", default_value = "50")]
    n: i16,

    // If in printer mode, take as little screen width as possible
    // Option ignored in formatter mode.
    #[structopt(long = "shrink")]
    shrink: bool,

    // Use fixed row width
    // This option overrides automatically determined terminal width (if one is available).
    #[structopt(short = "w", long = "width", default_value = "119")]
    width: i16,
}


fn main() {
    let cl_args = Args::from_args();
    println!("{:?}", cl_args);

    let context = ExecutionContext::new();
    println!("Execution context: {:?}", &context);
    let stdin = io::stdin();
    let in_handle = stdin.lock(); // TODO: test perf
//    let stdout= io::Stdout::lock(); // TODO: test perf

    let mut lines = io::BufReader::new(in_handle)
        .lines()
        .filter_map(|x| x.ok());

    let first_lines = (&mut lines).take(50).collect::<Vec<String>>();

    let result = analyze(&first_lines);
    println!("Result: {:?}", result);
    let split_info = split_proportionally(
        &result, context.out_stream_width.unwrap());

    println!("----------------------------------");
    for l in first_lines {
        println!("{}", format_line(l, &split_info));
    };

    println!("fin first lines, printing remaining");
    for l in lines {
        println!("{}", format_line(l, &split_info));
    };
}

// TODO: setup repo na githubie, release-0.0.1
// skończenie 0.1.0
// wypaczkowanie