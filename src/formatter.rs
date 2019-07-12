use std::iter::repeat;

pub fn analyze(lines: &[String], sep: &str) -> Vec<usize> {
    let mut columns_to_lengths: Vec<Vec<usize>> = Vec::new();

    for line in lines {
        let column_value_lenghts = line
            .split(sep)
            .map(|substr| substr.trim().chars().count())
            .collect::<Vec<usize>>();

        if columns_to_lengths.len() < column_value_lenghts.len() {
            columns_to_lengths.resize_with(column_value_lenghts.len(), Vec::new)
        }

        for (i, col_value_length) in column_value_lenghts.iter().enumerate() {
            columns_to_lengths[i].push(*col_value_length)
        }
    }

    columns_to_lengths
        .into_iter()
        .map(|v| *v.iter().max().unwrap())
        .collect::<Vec<_>>()
}

/// If available width can't be reasonably splitten, return error
pub fn split_available_width(
    max_lengths: &[usize],
    available_width: usize,
    output_sep_len: usize,
    expand: bool,
    //) -> Result<Vec<usize>, _> {
) -> Vec<usize> {
    let n_columns = max_lengths.len();
    let n_separators = if n_columns >= 1 { n_columns - 1 } else { 0 };
    let width_to_alloc = (available_width - n_separators * output_sep_len) as f64;

    let max_len_sum = max_lengths.iter().sum::<usize>() as f64;

    let available_chars_per_column: Vec<usize> = if !expand && width_to_alloc > max_len_sum {
        max_lengths.into() // no need to limit available space
    } else {
        // split available space proportionally
        max_lengths
            .iter()
            .map(|&l| (width_to_alloc * (l as f64) / max_len_sum).floor() as usize)
            .collect()
    };
    let total_cols_width = available_chars_per_column.iter().sum::<usize>();
    assert!(width_to_alloc >= total_cols_width as f64);

    available_chars_per_column
}

///
pub fn format_line(
    input_line: String,
    split_info: &[usize],
    input_sep: &str,
    output_sep: &str,
    fill_value: char,
) -> String {
    input_line
        .trim()
        .split(input_sep)
        .zip(split_info.iter())
        .map(|(col_value, width_to_fill)| {
            let chars = col_value.chars().chain(repeat(fill_value));

            if col_value.chars().count() <= *width_to_fill {
                chars.take(*width_to_fill).collect::<String>()
            } else {
                chars
                    .take(if *width_to_fill >= 1 {
                        *width_to_fill - 1
                    } else {
                        0
                    })
                    .chain(if *width_to_fill > 0 { "*" } else { "" }.chars())
                    .collect::<String>()
            }
        })
        .collect::<Vec<String>>()
        .join(output_sep)
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test! {
        analyze;
        {
            empty_input: (&vec![], "\t") => vec![] as Vec<usize>,

            one_empty_string: (&vec!["".to_string()], "\t") => vec![0],

            four_empty_strings: (&vec!["\t\t\t".to_string()], "\t") => vec![0, 0, 0, 0],

            simple_tsv: (&vec![
                vec!["sample", "tsv", "header"].join("\t"),
                vec!["first", "tsv", "row"].join("\t"),
                vec!["another", "tsvvvvvvvv", "row"].join("\t"),
            ], "\t") => vec![7, 10, 6],

            simple_ssv: (&vec![
                vec!["sample", "tsv", "header"].join(" "),
                vec!["first", "tsv", "row"].join(" "),
                vec!["another", "tsvvvvvvvv", "row"].join(" "),
            ], " ") => vec![7, 10, 6],
        }
    }

    parameterized_test! {
        split_available_width;
        {
            empty_input: (&vec![], 50, 5, false) => vec![] as Vec<usize>,

            no_limiting_needed: (&vec![5, 6, 7], 50, 5, false) => vec![5, 6, 7],
        }
    }

    parameterized_test! {
        format_line;
        {
            empty_line:
            ("".to_string(), &vec![], "\t", "|", ' ') => "".to_string(),

            simple_2_columns:
            ("123\t12 ".to_string(), &vec![5, 5], "\t", "|", ' ') => "123  |12   ".to_string(),

            columns_compressed:
            ("12345689\tabcdefgh ".to_string(), &vec![4, 4], "\t", "|", ' ') => "123*|abc*".to_string(),


            columns_supercompressed:
            ("12345689\tabcdefgh ".to_string(), &vec![1, 0], "\t", "|", ' ') => "*|".to_string(),

            doesnt_panic_with_0width_colums:
            ("12345689\t12345 ".to_string(), &vec![0, 0], "\t", "|", ' ') => "|".to_string(),

            columns_with_spare_space:
            ("123456\tabcd".to_string(), &vec![8, 8], "\t", "|", ' ') => "123456  |abcd    ".to_string(),

        }
    }
}
