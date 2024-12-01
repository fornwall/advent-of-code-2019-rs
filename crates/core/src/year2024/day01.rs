use crate::common::array_stack::ArrayStack;
use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut left_list = ArrayStack::<1024, u32>::new();
    let mut right_list = ArrayStack::<1024, u32>::new();

    for line in input.text.lines() {
        let (l, r) = line.split_once("   ").ok_or_else(on_error)?;
        left_list.push(l.parse().map_err(|_| on_error())?)?;
        right_list.push(r.parse().map_err(|_| on_error())?)?;
    }

    Ok(if input.is_part_one() {
        right_list.slice_mut().sort_unstable();
        left_list.slice_mut().sort_unstable();
        left_list
            .slice()
            .iter()
            .zip(right_list.slice().iter())
            .map(|(l, &r)| l.abs_diff(r))
            .sum::<u32>() as u64
    } else {
        left_list
            .slice()
            .iter()
            .map(|&l| u64::from(l) * right_list.slice().iter().filter(|&&r| r == l).count() as u64)
            .sum::<u64>()
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";
    test_part_one_no_allocations!(test_input => 11);
    test_part_two_no_allocations!(test_input => 31);

    let real_input = include_str!("day01_input.txt");
    test_part_one_no_allocations!(real_input => 1_882_714);
    test_part_two_no_allocations!(real_input => 19_437_052);
}