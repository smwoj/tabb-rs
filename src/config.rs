use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tab",
    about = "
Formatting and padding utility for tabular data.

This program reprints rows of tabular data based on widths necessary
for pretting printing of n first lines (default=50). 


tab switches between two modes:
 - printer (when there's a terminal serving as stdout device),
 - formatter (otherwise - when output is e.g. piped or redirected to file).
"
)]
struct Args {
    /// Column separator char [default: \t]
    #[structopt(short = "s", long = "input-sep")]
    input_sep: Option<String>,

    /// Padding character [default: space]
    #[structopt(short = "p", long = "padding")]
    padding: Option<char>,
    // Defaults for sep & padding applied outside stuctops
    // to avoid confusing printing of '... [default:  ]'
    /// Output separator char
    #[structopt(long = "output-sep", default_value = " ")]
    output_sep: String,

    /// Capture n first lines for calculating columns' widths
    #[structopt(short = "n", default_value = "50")]
    n: usize,

    /// Use fixed row width.
    /// This option overrides automatically determined terminal width
    #[structopt(short = "w", long = "width")]
    width: Option<usize>,

    /// Use as much screen width as possible (ignored in the formatter mode)
    #[structopt(long = "expand")]
    expand: bool,
    // TODO: add option for reading from a file
}

#[derive(Debug, Clone)]
pub struct Config {
    pub is_stdout_tty: bool,
    pub width: usize,
    pub n: usize,

    pub input_sep: String,
    pub output_sep: String,
    pub padding: char,
    pub expand: bool,
}

impl Config {
    pub fn new() -> Self {
        let is_stdout_tty = atty::is(atty::Stream::Stdout);
        let args = Args::from_args();

        Self {
            is_stdout_tty,
            width: args
                .width // specified stream width trumps all
                .or_else(|| {
                    termion::terminal_size() // else check available tty width
                        .ok()
                        .map(|(width, _h)| width as usize)
                })
                .unwrap_or(119), // default width for formatter mode
            n: args.n,

            input_sep: args.input_sep.unwrap_or_else(|| '\t'.to_string()),
            output_sep: args.output_sep,
            padding: args.padding.unwrap_or(' '),
            expand: args.expand,
        }
    }
}
