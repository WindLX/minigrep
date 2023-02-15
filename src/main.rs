use colored::Colorize;
use minigrep;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!(
            "{}{}{}",
            " ERROR ".on_red().bold(),
            " Problem happens when parsing arguments: ".truecolor(207, 164, 10),
            err
        );
        process::exit(1);
    });

    println!(
        "{}{} {} {} {}",
        " INFO ".on_blue().bold(),
        " Search for".green(),
        config.query,
        "in file".green(),
        config.file_name
    );

    if let Err(err) = minigrep::run(config) {
        eprintln!(
            "{}{} {}:",
            " ERROR ".on_red().bold(),
            " Problem happens when running:".truecolor(207, 164, 10),
            err
        );
        process::exit(1);
    }
}
