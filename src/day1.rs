#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.parse::<u16>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u16]) -> i32 {
    let mut prev = input[0];
    let mut ct = 0;
    for v in input.iter().skip(1) {
        if *v > prev {
            ct = ct + 1;
        }
        prev = *v;
    }
    ct
}

#[aoc(day1, part2)]
pub fn part2(input: &[u16]) -> i32 {
    let mut prev = input[0] + input[1] + input[2];
    let mut ct = 0;
    for (i, _) in input.iter().skip(1).enumerate() {
        if i + 3 > input.len() {
            break
        }
        if input[i] + input[i+1] + input[i+2] > prev {
            ct = ct + 1
        }
        prev = input[i] + input[i+1] + input[i+2];
    }
    ct
}