mod commands;
// Items from traits can only be used is trait is in scope
use clap::Parser;
fn main() {
    let cli = commands::Args::parse();

    println!("Hello, {}",cli.name);
}
