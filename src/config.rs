use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tabb",
    about = "
Formatting and padding utility for tabular data.

This program reprints rows of tabular data based on widths necessary
for pretting printing of n first lines (default=50). 

Terminal width is the default for formatting output.
It can be overridden via -w, --width option
"
)]
struct Args {
    /// Column separator char [default: tab]
    #[structopt(short = "s", long = "in-sep", default_value = "\t")]
    input_sep: char,

    /// Capture n first lines for calculating columns' widths
    #[structopt(short = "n", default_value = "50")]
    n: usize,

    /// Padding character [default: space]
    #[structopt(short = "p", long = "padding", default_value = " ")]
    padding: char,

    // Defaults for sep & padding applied outside stuctops   # default_value = "\t")
    // to avoid confusing printing of '... [default:  ]'
    /// Output separator char [default: tab]
    #[structopt(long = "out-sep", default_value = "|")]
    output_sep: char,

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

    pub input_sep: char,
    pub output_sep: char,
    pub padding: char,
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

            input_sep: args.input_sep,
            output_sep: args.output_sep,
            padding: args.padding,
        }
    }
}
