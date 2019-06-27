use std::iter::repeat;


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
