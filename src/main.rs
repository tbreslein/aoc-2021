use clap::Parser;

/// Oh, hi...
#[derive(Parser)]
#[clap(version = "0.0.1", author = "tommyb")]
struct Opts {
    /// which day of the AoC you wanna run; valid values: {1, 2, ..}
    #[clap(short, long)]
    day: i32,

    /// which part of that day you wanna run; valid values: {1, 2}
    #[clap(short, long)]
    part: i32,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("Value for day: {}", opts.day);
    println!("Value for part: {}", opts.part);

    match opts.day {
        1 => match opts.part {
            1 => println!("Day 1, part 1!"),
            2 => println!("Day 1, part 2!"),
            _ => println!(
                "You chose part {}, but there only 2 parts per day!",
                opts.part
            ),
        },
        _ => println!("Have not finished day {} (yet)...", opts.day),
    };
}
