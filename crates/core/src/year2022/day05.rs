use std::num::NonZeroU8;

use crate::input::Input;

struct Crates {
    stacks: [[u8; Self::MAX_STACK_SIZE]; Self::MAX_STACKS],
    stack_sizes: [u8; Self::MAX_STACKS],
}

impl Crates {
    const MAX_STACKS: usize = 10;
    const MAX_STACK_SIZE: usize = 50;

    const fn new() -> Self {
        Self {
            stacks: [[0; Self::MAX_STACK_SIZE]; Self::MAX_STACKS],
            stack_sizes: [0; Self::MAX_STACKS],
        }
    }

    fn push(&mut self, stack_idx: usize, crate_char: u8) -> Result<(), String> {
        if stack_idx >= Self::MAX_STACKS {
            return Err(format!(
                "Too high stack index - only {} supported",
                Self::MAX_STACKS
            ));
        }
        let stack_size = self.stack_sizes[stack_idx];
        if usize::from(stack_size) == Self::MAX_STACK_SIZE {
            return Err(format!(
                "Stack overflow on push - max stack size is {}",
                Self::MAX_STACK_SIZE
            ));
        }
        self.stacks[stack_idx][usize::from(stack_size)] = crate_char;
        self.stack_sizes[stack_idx] += 1;
        Ok(())
    }

    fn move_crates(
        &mut self,
        num: u8,
        from_stack_idx: usize,
        to_stack_idx: usize,
        model_9001: bool,
    ) -> Result<(), String> {
        if from_stack_idx >= Self::MAX_STACKS || to_stack_idx >= Self::MAX_STACKS {
            return Err(format!(
                "Too high stack index - only {} supported",
                Self::MAX_STACKS
            ));
        } else if self.stack_sizes[from_stack_idx] < num {
            return Err("Stack underflow on move".to_string());
        } else if usize::from(num) > Self::MAX_STACK_SIZE
            || self.stack_sizes[to_stack_idx] > Self::MAX_STACK_SIZE as u8 - num
        {
            return Err(format!(
                "Stack overflow on move - max stack size is {}",
                Self::MAX_STACK_SIZE
            ));
        }

        for i in 0..num {
            let from_stack_size_idx = i32::from(self.stack_sizes[from_stack_idx])
                + if model_9001 {
                    -i32::from(num) + i32::from(i)
                } else {
                    -1 - i32::from(i)
                };
            let crate_char = self.stacks[from_stack_idx][from_stack_size_idx as usize];
            self.stacks[to_stack_idx][usize::from(self.stack_sizes[to_stack_idx] + i)] = crate_char;
        }
        self.stack_sizes[from_stack_idx] -= num;
        self.stack_sizes[to_stack_idx] += num;
        Ok(())
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .enumerate()
            .filter_map(|(stack_idx, stack)| {
                let stack_size = self.stack_sizes[stack_idx];
                if stack_size > 0 {
                    Some(stack[usize::from(self.stack_sizes[stack_idx] - 1)] as char)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut stacks = Crates::new();

    for line in input.text.lines().rev() {
        if line.contains('[') {
            for (char_offset, crate_char) in line.bytes().enumerate() {
                if crate_char.is_ascii_uppercase() || crate_char.is_ascii_digit() {
                    let stack_idx = char_offset / 4;
                    stacks.push(stack_idx, crate_char)?;
                }
            }
        }
    }

    for line in input.text.lines() {
        if line.starts_with("move ") {
            let error_mapper =
                || "Invalid input: Not of the form 'move u8 from u8 to u8'".to_string();
            let mut words = line.split(' ');
            let num = words
                .nth(1)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or_else(error_mapper)?;
            let from = u8::from(
                words
                    .nth(1)
                    .and_then(|s| s.parse::<NonZeroU8>().ok())
                    .ok_or_else(error_mapper)?,
            ) - 1;
            let to = u8::from(
                words
                    .nth(1)
                    .and_then(|s| s.parse::<NonZeroU8>().ok())
                    .ok_or_else(error_mapper)?,
            ) - 1;

            stacks.move_crates(num, from as usize, to as usize, input.is_part_two())?;
        }
    }

    Ok(stacks.top_crates())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    let test_input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    test_part_one!(test_input => "CMZ".to_string());
    test_part_two!(test_input => "MCD".to_string());

    let real_input = include_str!("day05_input.txt");
    test_part_one!(real_input => "ZBDRNPMVH".to_string());
    test_part_two!(real_input => "WDLPFNNNB".to_string());

    let test_input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 3 from 2 to 1";
    test_part_one!(test_input => "MP".to_string());
    let test_input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 4 from 2 to 1";
    test_part_one_error!(test_input => "Stack underflow on move".to_string());

    let mut stacks = Crates::new();
    for _ in 0..Crates::MAX_STACK_SIZE {
        assert!(stacks.push(0, 1).is_ok());
    }
    assert!(stacks.push(0, 1).is_err());
}
