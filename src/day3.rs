#[aoc_generator(day3, part1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    const RADIX: u32 = 10;
    input.lines()
        .map(|l|
            l.chars().map(|c|
                c.to_digit(RADIX).unwrap()
            ).collect()
        ).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut gamma: u32 = 0b0000_0000;
    let mut epsilon: u32 = 0b0000_0000;
    for i in (0..input[0].len()) {
        match input.iter().filter(|&n|{
            n[i] == 1
        } ).count() {
            ct if ct > input.len()/2 => gamma |= 0b0000_0001,
            _ => epsilon |= 0b0000_0001
        }
        if i < input[0].len() - 1{
            gamma <<= 1;
            epsilon <<= 1
        }
    }
    println!("gamma: 0b{:08b}, {}", gamma, gamma);
    println!("epsilon: 0b{:08b}, {}", epsilon, epsilon);
    gamma * epsilon
}

#[aoc_generator(day3, part2)]
pub fn input_generator2(input: &str) -> Vec<u32> {
    input.lines().map(|l| u32::from_str_radix(l, 2).unwrap()).collect()
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<u32>) -> u32 {
    let mut oxygen = input.to_owned();
    let mut co2 = input.to_owned();
    println!("Start oxygen: {:?}", bytes_str(&oxygen));
    println!("Start co2: {:?}", bytes_str(&co2));
     println!("");
    for i in (0..12).rev() {
        if oxygen.len() > 1 {
            let bit_ct = oxygen.iter().filter(|&n| get_bit_at(*n, i)).count();
            let cur_len = oxygen.len();
             oxygen.retain(|&n| {
                if bit_ct > (cur_len + 1) / 2 {
                    get_bit_at(n, i) == true
                } else if bit_ct < (cur_len + 1) / 2 {
                    get_bit_at(n, i) == false
                } else {
                    get_bit_at(n, i) == true
                }
            });
        }
        if co2.len() > 1 {
            let bit_ct = co2.iter().filter(|&n| get_bit_at(*n, i)).count();
            let cur_len = co2.len();
             co2.retain(|&n| {
                if bit_ct > (cur_len + 1) / 2 {
                    get_bit_at(n, i) == false
                } else if bit_ct < (cur_len + 1) / 2 {
                    get_bit_at(n, i) == true
                } else {
                    get_bit_at(n, i) == false
                }
            });
        }
        println!("Run {} oxygen: {:?}", i, bytes_str(&oxygen));
        println!("Run {} co2: {:?}", i, bytes_str(&co2));
        println!("")
    }
    println!("oxygen: 0b{:08b}, {}", oxygen[0], oxygen[0]);
    println!("co2: 0b{:08b}, {}", co2[0], co2[0]);
    oxygen[0] * co2[0]
}

fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn bytes_str(input: &Vec<u32>) -> String {
    let strs: Vec<String> = input.iter().map(|v| {
        format!("0b{:08b}", v)
    }).collect();
    strs.join(", ")
}