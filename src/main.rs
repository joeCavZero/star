use star_std::cli::Scannable;
use star_std::cli::Cli;

fn main() {
    let mut cli = Cli::new();
    cli.scan();
    cli.run();
}
