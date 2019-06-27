use std::iter::repeat;
// TODO: use rstest or write a generic macro for parameterized tests


pub fn analyze(
    lines: &Vec<String>,
    sep: &str
) -> Vec<usize>{
    let mut columns_to_lengths: Vec<Vec<usize>> = Vec::new();

    for line in lines {
        let column_value_lenghts = line
            .trim()
            .split(sep)
            .map(|substr| substr.chars().count())
            .collect::<Vec<usize>>();

        if columns_to_lengths.len() < column_value_lenghts.len() {
            columns_to_lengths
                .resize_with(column_value_lenghts.len(), || Vec::new())
        }

        for (i, col_value_length) in column_value_lenghts.iter().enumerate() {
            columns_to_lengths[i].push(*col_value_length)
        };
    };

    columns_to_lengths.into_iter()
        .map(|v| v.iter().max().unwrap().clone())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test_analyze {
    use super::*;

    macro_rules! test_analyze {
    ($($test_name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $test_name() {
            let (sep, expected_results, lines) = $value;
            assert_eq!(analyze(lines, sep), expected_results);
        }
    )*
    }
}

    test_analyze! {
        empty_input: ("\t", vec![], &vec![]),

        simple_tsv: ("\t", vec![7, 10, 6], &vec![
            vec!["sample", "tsv", "header"].join("\t"),
            vec!["first", "tsv", "row"].join("\t"),
            vec!["another", "tsvvvvvvvv", "row"].join("\t"),
        ]),

        simple_ssv: (" ", vec![7, 10, 6], &vec![
            vec!["sample", "tsv", "header"].join(" "),
            vec!["first", "tsv", "row"].join(" "),
            vec!["another", "tsvvvvvvvv", "row"].join(" "),
        ]),
    }
}


pub fn split_available_width(
    max_lengths: &Vec<usize>,
    available_width: usize,
    output_sep_len: usize,
    expand: bool,
) -> Vec<usize> {
    let n_columns = max_lengths.len();
    let width_to_alloc = (available_width - (n_columns - 1)*output_sep_len) as f64;
    // FIXME attempt to subtract with overflow
    // wojtek@lapwoj:~/repos/tab-rs$ cat input-example.txt | cargo run -- --output-sep ' |        '

    let max_len_sum = max_lengths.iter().sum::<usize>() as f64;

    let available_chars_per_column = match !expand && width_to_alloc > max_len_sum {
        true => max_lengths.clone(),  // no need to limit available space
        false => {                    // split available space proportionally
            max_lengths.iter().map(
                |&l| {
                    (width_to_alloc * (l as f64) / max_len_sum ).floor() as usize
                }).collect::<Vec<usize>>()
        },
    };
    let total_cols_width = available_chars_per_column.iter().sum::<usize>();
    assert!(width_to_alloc >= total_cols_width as f64);

    available_chars_per_column
}


#[cfg(test)]
mod test_split_available_width {
    use super::*;

    macro_rules! test_split_available_width {
    ($($test_name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $test_name() {
            let (max_lengths, available_width, output_sep_len, expand, expected_results) = $value;
            assert_eq!(
                split_available_width(max_lengths, available_width, output_sep_len, expand),
                expected_results);
        }
    )*
    }
}

    test_split_available_width! {
        empty_input: (
           &vec![], 50, 5, false,
           vec![],
        ),

        no_limiting_needed: (
           &vec![5, 6, 7], 50, 5, false,
           vec![5, 6, 7],
        ),
    }
}


pub fn format_line(
    input_line: String,
    split_info: &Vec<usize>,
    input_sep: &str,
    output_sep: &str,
    fill_value: char,
)-> String {
    input_line
        .trim()
        .split(input_sep)
        .zip(split_info.iter())
        .map(|(col_value, width_to_fill)|{
            let chars = col_value
                .chars()
                .chain(repeat(fill_value));

            if col_value.chars().count() <= *width_to_fill {
                chars
                    .take(*width_to_fill)
                    .collect::<String>()
            } else {
                chars
                    .take(*width_to_fill-1)
                    .chain("*".chars())
                    .collect::<String>()
            }
        })
        .collect::<Vec<String>>()
        .join(output_sep)
}

