use itertools::Itertools;

fn get_nth_char(str: &str, n: usize) -> char {
    str.chars().nth(n).expect("No character found")
}

fn count_ones_zeros(x: &Vec<char>) -> (usize, usize) {
    x.iter().fold((0, 0), |mut count, c| {
        match c {
            '0' => count.0 += 1,
            '1' => count.1 += 1,
            a => panic!("Invalid character: {}", a)
        };
        return count;
    })
}

fn get_most_common_character(x: &Vec<char>) -> char {
    let counts = count_ones_zeros(x);

    match counts {
        (zeros, ones) if ones > zeros => '1',
        (zeros, ones) if zeros > ones => '0',
        (zeros, ones) if zeros == ones => '=',
        _ => panic!()
    }
}

fn main() {
    let rows = include_str!("../input.txt")
        .lines()
        .collect_vec();

    println!("* Part 1");

    part_1(&rows);

    println!("* Part 2");

    part_2(&rows);
}


fn part_1(rows: &Vec<&str>) {
    let number_of_bits = rows.get(0).expect("No data provided").len();

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for bit in 0..number_of_bits {
        let characters = rows.iter()
            .map(|s| get_nth_char(s, bit))
            .collect_vec();
        let most_common_character = get_most_common_character(&characters);

        match most_common_character {
            '0' => {
                gamma += "0";
                epsilon += "1";
            }
            '1' => {
                gamma += "1";
                epsilon += "0";
            }
            _ => panic!()
        }
    }

    let gamma_decimal = usize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_decimal = usize::from_str_radix(epsilon.as_str(), 2).unwrap();

    println!("Gamma: {} = {}", gamma, gamma_decimal);
    println!("Epsilon: {} = {}", epsilon, epsilon_decimal);
    println!("Power Consumption: {}", gamma_decimal * epsilon_decimal);
}

fn part_2(rows: &Vec<&str>) {
    let number_of_bits = rows.get(0).expect("No data provided").len();

    let mut input = rows.clone();

    // Oxygen

    for bit in 0..number_of_bits {
        let characters = input.iter()
            .map(|s| get_nth_char(s, bit))
            .collect_vec();

        let most_common_character = get_most_common_character(&characters);

        match most_common_character {
            '=' => input.retain(|&s| get_nth_char(s, bit) == '1'),
            _ => input.retain(|&s| get_nth_char(s, bit) == most_common_character)

        }

        if input.len() <= 1 {
            break;
        }
    }

    assert_eq!(input.len(), 1, "Got more than one filtered input!");

    let oxygen_generator_rating = input.first().unwrap();
    let oxygen_generator_rating_decimal = usize::from_str_radix(oxygen_generator_rating, 2).unwrap();

    println!("oxygen generator rating: {} = {}", oxygen_generator_rating, oxygen_generator_rating_decimal);

    // CO2 Scrubber

    let mut input = rows.clone();

    for bit in 0..number_of_bits {
        let characters = input.iter()
            .map(|s| get_nth_char(s, bit))
            .collect_vec();

        let most_common_character = get_most_common_character(&characters);

        match most_common_character {
            '=' => input.retain(|&s| get_nth_char(s, bit) == '0'),
            _ => input.retain(|&s| get_nth_char(s, bit) != most_common_character)

        }

        if input.len() <= 1 {
            break;
        }
    }

    assert_eq!(input.len(), 1, "Got more than one filtered input!");

    let co_2_scrubber_rating = input.first().unwrap();
    let co_2_scrubber_rating_decimal = usize::from_str_radix(co_2_scrubber_rating, 2).unwrap();

    println!("co2 scrubber rating: {} = {}", co_2_scrubber_rating, co_2_scrubber_rating_decimal);
    println!("product: {}", oxygen_generator_rating_decimal * co_2_scrubber_rating_decimal);
}