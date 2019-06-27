use std::io;
use std::io::BufRead;

mod config;
mod formatter;


fn main() {
    let config = config::Config::new();
    println!("{:?}", config);

    // acquire std{in,out} locks
    let stdin = io::stdin();
    let stdout= io::stdout();
    let in_handle = stdin.lock();
    let out_handle = stdout.lock();

    // create iterator over stdin lines
    let mut lines_iter = io::BufReader::new(in_handle)
        .lines()
        .filter_map(|x| x.ok());

    let first_lines: Vec<String> = (&mut lines_iter).take(50).collect();
    let result = formatter::analyze(&first_lines, config.input_sep);

    println!("Analysis result: {:?}", result);
    let split_info = formatter::split_proportionally(
        &result, config.width,);


    println!("----------------------------------");
    for l in first_lines {
        println!("first: {}", formatter::format_line(
            l, &split_info, config.input_sep, config.output_sep));
    };
    for l in lines_iter {
        println!("more: {}", formatter::format_line(
            l, &split_info, config.input_sep, config.output_sep));
    };
}

// TODO
// finish 0.1.0
// release so it can be `cargo install`ed
// use rustfmt
// use clippy for linting
