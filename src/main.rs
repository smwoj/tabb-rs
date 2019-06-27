use std::io;
use std::io::BufRead;
use std::iter::repeat;


mod config;


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


fn main() {
    let config = config::Config::new();
    println!("{:?}", config);

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
        &result, config.width);

    println!("----------------------------------");
    for l in first_lines {
        println!("{}", format_line(l, &split_info));
    };

    println!("fin first lines, printing remaining");
    for l in lines {
        println!("{}", format_line(l, &split_info));
    };
}

// skończenie 0.1.0
// wypaczkowanie