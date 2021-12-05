
use itertools::Itertools;

use std::io;
use std::io::BufRead;

fn main()
{
    let handle = io::stdin();
    let mut iter = handle.lock()
        .lines()
        .map(|l| l.expect("Could not read from stdin!"))
        .map(|l| l.parse::<u32>()
                  .expect(format!("Could not convert input to u32: {}", l).as_str()));
        // .map(|l| { eprintln!("{}", l); l });

    let count = iter.tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| b > a)
        .filter(|b| *b)
        .count();
        // .for_each(|n| eprintln!("{}", n));

    println!("{}", count);
}
