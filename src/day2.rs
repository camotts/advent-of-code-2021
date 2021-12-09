pub struct Command {
    keyword: String,
    value: i32
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input.lines().map(|l| {
        let spl: Vec<&str> = l.split(" ").collect();
        Command{
            keyword: spl[0].to_string(),
            value: spl[1].parse().unwrap(),
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    input.iter().enumerate().for_each(|(i, c)| {
        match c.keyword.as_str() {
            "forward" => horizontal = horizontal + c.value,
            "down" => depth = depth + c.value,
            "up" => depth = depth - c.value,
            _ => println!("Unexpected! {}", c.keyword)
        }
    });
    horizontal * depth
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    input.iter().enumerate().for_each(|(i, c)| {
        match c.keyword.as_str() {
            "forward" => {
                horizontal = horizontal + c.value;
                depth = depth + (aim * c.value)
            },
            "down" => aim = aim + c.value,
            "up" => aim = aim - c.value,
            _ => println!("Unexpected! {}", c.keyword)
        }
    });
    horizontal * depth
}