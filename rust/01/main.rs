
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let handle = io::stdin();
    let mut iter = handle.lock()
        .lines()
        .map(|l| l.expect("Could not read from stdin!"))
        .map(|l| l.parse::<u32>()
                  .expect(format!("Could not convert input to u32: {}", l).as_str()))
        .map(|l| { eprintln!("{}", l); l });

    let init_value = iter.next().unwrap();

    let count = iter
        .scan(init_value, |prev, current| {
            let res = Some(current > *prev);
            *prev = current;
            res
        })
        .fold(0u32, |acc, greater| if greater { acc+1 } else { acc });

    println!("{}", count);

    Ok(())
}
