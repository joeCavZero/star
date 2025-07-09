use star::cli::*;

fn main() {
    let mut cli = Cli::new();
    cli.scan();
    cli.run();
}
