use std::io;
use std::io::BufRead;

#[macro_use]
mod macros;
mod config;
mod formatter;

fn main() {
    // parse cl args and inspect execution context (are we writing to a tty?)
    let config = config::Config::new();

    // acquire std{in,out} locks
    let stdin = io::stdin();
    // let stdout = io::stdout();
    let in_handle = stdin.lock();
    // let _out_handle = stdout.lock();

    // create iterator over stdin lines
    let mut lines_iter = io::BufReader::new(in_handle).lines().filter_map(|x| x.ok());

    // collect and analyze first n lines
    let first_lines: Vec<String> = (&mut lines_iter).take(config.n).collect();
    let col_sizes = formatter::analyze(&first_lines, config.input_sep);

    let split_info = formatter::split_available_width(&col_sizes, config.width);

    for l in first_lines {
        println!(
            "{}",
            &formatter::format_line(
                l,
                &split_info,
                config.input_sep,
                config.output_sep,
                config.padding,
            )
        );
    }
    for l in lines_iter {
        // println!(
        //     "{}",
        formatter::format_line(
            l,
            &split_info,
            config.input_sep,
            config.output_sep,
            config.padding,
        );
        // );
    }
}
