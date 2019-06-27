use std::io;
use std::io::BufRead;

mod config;
mod formatter;

fn main() {
    // parse cl args and inspect execution context (are we writing to a tty?)
    let config = config::Config::new();
    dbg!(&config);

    // acquire std{in,out} locks
    let stdin = io::stdin();
    let stdout= io::stdout();
    let in_handle = stdin.lock();
    let _out_handle = stdout.lock();

    // create iterator over stdin lines
    let mut lines_iter = io::BufReader::new(in_handle)
        .lines()
        .filter_map(|x| x.ok());

    // collect and analyze first n lines
    let first_lines: Vec<String> = (&mut lines_iter).take(config.n).collect();
    let col_sizes = formatter::analyze(&first_lines, &config.input_sep);

    dbg!(&col_sizes, col_sizes.iter().sum::<usize>());
    let split_info = formatter::split_available_width(
        &col_sizes, config.width,
        config.output_sep.chars().count(), config.expand);
    dbg!(&split_info, split_info.iter().sum::<usize>());

    for l in first_lines {
        println!("{}", formatter::format_line(
            l, &split_info, &config.input_sep, &config.output_sep, config.padding));
    };
    for l in lines_iter {
        println!("{}", formatter::format_line(
            l, &split_info, &config.input_sep, &config.output_sep, config.padding));
    };
}

// TODO
// finish 0.1.0
// release as `cargo install`able
// use rustfmt
// use clippy
