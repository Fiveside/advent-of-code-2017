/*
You're standing in a room with "digitization quarantine" written in LEDs along one wall. The only
door is locked, but it includes a small interface. "Restricted Area - Strictly No Digitized Users
Allowed."

It goes on to explain that you may only leave by solving a captcha to prove you're not a human.
Apparently, you only get one millisecond to solve the captcha: too fast for a normal human, but it
feels like hours to you.

The captcha requires you to review a sequence of digits (your puzzle input) and find the sum of all
digits that match the next digit in the list. The list is circular, so the digit after the last
digit is the first digit in the list.

For example:

* 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the
  third digit (2) matches the fourth digit.
* 1111 produces 4 because each digit (all 1) matches the next.
* 1234 produces 0 because no digit matches the next.
* 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.

*/
use itertools;
use itertools::Itertools;
use std::str::FromStr;

pub fn day1(input: &str) {
    println!("part 1 solution: {}", part1(input));
    if input.len() % 2 == 0 {
        println!("part 2 solution: {}", part2(input));
    }
}

fn part1(input: &str) -> u64 {
    let mut input: Vec<_> = input.chars()
        .map(|x| u64::from_str(&x.to_string()).unwrap())
        .collect();
    let first = input[0];
    input.push(first);

    input.into_iter()
        .tuple_windows()
        .filter_map(|(x, y)| if x == y {Some(x)} else {None})
        .sum()
}

fn part2(input: &str) -> u64 {
    let nums: Vec<_> = (0..input.len())
        .map(|start| u64::from_str_radix(&input[start..start+1], 10).unwrap())
        .collect();

    let half = nums.len() / 2;

    itertools::zip(&nums[..half], &nums[half..])
        .filter_map(|(&x, &y)| if x == y {Some(x*2)} else {None})
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1_computes_triple_adjacent() {
        assert_eq!(part1("0111"), 2);
    }

    #[test]
    fn test_day1_part1_wraps() {
        assert_eq!(part1("11"), 2);
    }

    #[test]
    fn test_day1_part1_skips_non_equal() {
        assert_eq!(part1("123"), 0);
    }

    #[test]
    fn test_day1_part2_computes_skip2() {
        assert_eq!(part2("1212"), 6);
    }

    #[test]
    fn test_day1_part2_skips_no_match() {
        assert_eq!(part2("1221"), 0);
    }

    #[test]
    fn test_day1_part2_extras() {
        assert_eq!(part2("123425"), 4);
        assert_eq!(part2("123123"), 12);
        assert_eq!(part2("12131415"), 4);
    }
}