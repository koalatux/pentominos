use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, next_line_help = true, arg_required_else_help(true))]
pub struct Args {
    /// List and print all individual pieces, including symmetry.
    #[arg(short, long)]
    pub pieces: bool,

    /// Solve the board and print all solutions.
    #[arg(short, long)]
    pub solve: bool,

    /// Solve the board and count (but do not print) the solutions.
    #[arg(short, long)]
    pub count: bool,
}
