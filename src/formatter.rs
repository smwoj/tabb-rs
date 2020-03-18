use std::iter::repeat;
use unicode_width::UnicodeWidthStr;

/// Scans n first lines to collect info about longest values in each column.
pub fn analyze(lines: &[String], sep: char) -> Vec<usize> {
    let mut columns_to_lengths: Vec<Vec<usize>> = Vec::new();

    for line in lines {
        let column_value_lenghts = line
            .split(sep)
            .map(|substr| {
                let trimmed = substr.trim();
                let seg_width = UnicodeWidthStr::width(trimmed);
                dbg!(seg_width, trimmed);

                seg_width
            })
            .collect::<Vec<usize>>();

        if columns_to_lengths.len() < column_value_lenghts.len() {
            columns_to_lengths.resize_with(column_value_lenghts.len(), Vec::new)
        }

        for (i, col_value_length) in column_value_lenghts.iter().enumerate() {
            columns_to_lengths[i].push(*col_value_length)
        }
    }

    let c = columns_to_lengths
        .into_iter()
        .map(|v| *v.iter().max().unwrap())
        .collect::<Vec<_>>();
    dbg!(&c);
    c
}

/// Assigns widths to columns.
pub fn split_available_width(max_lengths: &[usize], available_width: usize) -> Vec<usize> {
    let n_columns = max_lengths.len();
    let n_separators = if n_columns >= 1 { n_columns - 1 } else { 0 };
    let width_to_alloc = (available_width - n_separators) as f64;

    let max_len_sum = max_lengths.iter().sum::<usize>() as f64;

    let available_chars_per_column: Vec<usize> = if width_to_alloc > max_len_sum {
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

    dbg!(&available_chars_per_column);
    available_chars_per_column
}

// 7+11|12+18|7+8|12+18|
// อินทรีทอง           |Golden Eagle                  |อินทรีทอง        |Golden Eagle
// 16+2|8+22|7+8|12+18|
// Szklana pułapka  |Die hard                      |อินทรีทอง        |Golden Eagle
// 18+0|14+16|15+0|13+17|
// 아가씨아가아가아가|The handmaiden                |食卓  休紙 片紙|these neither
// 14+4|30+0|14+1|30+0|
// 取締 取締 取締    |no idea what these glyphs mean|取締 取締 取締 |no idea what these glyphs mean
// 15+3|13+17|7+8|12+18|
// 食卓  休紙 片紙   |these neither                 |อินทรีทอง        |Golden Eagle

///Builds a line string using 'split info' as padding controller.
pub fn format_line(
    input_line: String,
    split_info: &[usize],
    input_sep: char,
    output_sep: char,
    padding: char,
) -> String {
    // print!("\n");
    let res = input_line
        .trim()
        .split(input_sep)
        .zip(split_info.iter())
        .map(|(orig_col_value, width_to_fill)| {
            let mut col_value: String = orig_col_value.trim().to_string();

            let col_value_len = UnicodeWidthStr::width(&col_value as &str);
            let padding_chars_no = width_to_fill - UnicodeWidthStr::width(&col_value as &str);
            // dbg!(&col_value, col_value_len, padding_chars_no);
            // print!("{}+{}|", col_value_len, padding_chars_no);
            col_value.extend(repeat(padding).take(padding_chars_no));

            col_value
            // .chars()
            // .chain()
            // .collect()

            // if UnicodeWidthStr::width(col_value) <= *width_to_fill {
            // chars.take(*width_to_fill).collect::<String>()
            // } else {
            //     chars
            //         .take(if *width_to_fill >= 1 {
            //             *width_to_fill - 1
            //         } else {
            //             0
            //         })
            //         .chain(if *width_to_fill > 0 { "*" } else { "" }.chars())
            //         .collect::<String>()
            // }
        })
        .collect::<Vec<String>>()
        .join(&output_sep.to_string());
    // print!("\n");
    // println!("{}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pl_length() {
        assert_eq!(UnicodeWidthStr::width("pułapka"), 7)
    }

    parameterized_test! {
        analyze;
        {
            empty_input:
            (&vec![], '\t') => vec![] as Vec<usize>,

            one_empty_string:
            (&vec!["".to_string()], '\t') => vec![0],

            four_empty_strings:
            (&vec!["\t\t\t".to_string()], '\t') => vec![0, 0, 0, 0],

            simple_tsv: (&vec![
                vec!["sample", "tsv", "header"].join("\t"),
                vec!["first", "tsv", "row"].join("\t"),
                vec!["another", "tsvvvvvvvv", "row"].join("\t"),
            ], '\t') => vec![7, 10, 6],

            simple_ssv: (&vec![
                vec!["sample", "tsv", "header"].join(" "),
                vec!["first", "tsv", "row"].join(" "),
                vec!["another", "tsvvvvvvvv", "row"].join(" "),
            ], ' ') => vec![7, 10, 6],
        }
    }

    parameterized_test! {
        split_available_width;
        {
            empty_input:
            (&vec![], 50) => vec![] as Vec<usize>,

            no_limiting_needed:
            (&vec![5, 6, 7], 50) => vec![5, 6, 7],
        }
    }

    parameterized_test! {
        format_line;
        {
            empty_line:
            ("".to_string(), &vec![], '\t', '|', ' ') => "".to_string(),

            simple_2_columns:
            ("123\t12 ".to_string(), &vec![5, 5], '\t', '|', ' ') => "123  |12   ".to_string(),

            columns_compressed:
            ("12345689\tabcdefgh ".to_string(), &vec![4, 4], '\t', '|', ' ') => "123*|abc*".to_string(),


            columns_supercompressed:
            ("12345689\tabcdefgh ".to_string(), &vec![1, 0], '\t', '|', ' ') => "*|".to_string(),

            doesnt_panic_with_0width_colums:
            ("12345689\t12345 ".to_string(), &vec![0, 0], '\t', '|', ' ') => "|".to_string(),

            columns_with_spare_space:
            ("123456\tabcd".to_string(), &vec![8, 8], '\t', '|', ' ') => "123456  |abcd    ".to_string(),

            with_exotic_alphabets:
            ("อินทรีทอง\tGolden Eagle
             Szklana pułapka\tDie hard
             아가씨\tThe handmaiden\
             取締 取締 取締\tno idea what these glyphs mean\
             食卓  休紙 片紙 \tthese neither".to_string(), &vec![8, 8], '\t', '|', ' ')
             => "123456  |abcd    ".to_string(),
        }
    }
}
