
use std::io;
use std::io::BufRead;

type Unit = u32;

#[derive(Debug)]
enum Movement
{
    Forward(Unit),
    Down(Unit),
    Up(Unit),
}

#[derive(Debug)]
enum ParseMovementError
{
    InvalidString,
    IntError(std::num::ParseIntError)
}

#[derive(Debug)]
struct Pos
{
    longitudinal: u32,
    depth: u32,
    aim: u32,
}

impl Pos
{
    fn apply(mut self, movement: &Movement) -> Pos
    {
        match movement {
            &Movement::Forward(longi) => {
                self.longitudinal += longi;
                self.depth += self.aim * longi;
            },
            &Movement::Down(depth) => self.aim += depth,
            &Movement::Up(depth) => self.aim -= depth,
        }
        self
    }
}

impl Default for Pos
{
    fn default() -> Self { Pos {longitudinal: 0, depth: 0, aim: 0} }
}

impl std::str::FromStr for Movement
{
    type Err = ParseMovementError;

    fn from_str(line: &str) -> Result<Self, Self::Err>
    {
        line.split_once(' ')
            .ok_or(ParseMovementError::InvalidString)
            .and_then(|(direction, amount)| {
                amount.parse::<Unit>()
                    .map_err(|e| ParseMovementError::IntError(e))
                    .map(|r| (direction, r))
            })
            .and_then(|(direction, amount)| {
                match direction {
                    "forward" => Ok(Movement::Forward(amount)),
                    "down"    => Ok(Movement::Down(amount)),
                    "up"      => Ok(Movement::Up(amount)),
                    _         => Err(ParseMovementError::InvalidString)
                }
            })
    }
}

fn main()
{
    let handle = io::stdin();
    let mut iter = handle.lock()
        .lines()
        .map(|l| l.expect("Could not read from stdin!"))
        .map(|l| l.parse::<Movement>()
                  .expect(format!("Could not parse input: [{}]", l).as_str()))
        .map(|l| { eprintln!("{:?}", l); l });

    let final_pos = iter.fold(Pos::default(), |pos, movement| (pos.apply(&movement)));

    eprintln!("{:?}", final_pos);

    println!("{}", final_pos.longitudinal * final_pos.depth);

}
