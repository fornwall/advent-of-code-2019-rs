#[cfg(feature = "visualization")]
use svgplot::{Coordinate, SvgColor, SvgImage, SvgPath, SvgRect, SvgScript, SvgStyle};

use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    const DIRECTIONS: [(i16, i16); 8] = [
        // NW, N, NE
        (-1, 1),
        (0, 1),
        (1, 1),
        // E, SE
        (1, 0),
        (1, -1),
        // S, SW
        (0, -1),
        (-1, -1),
        // W
        (-1, 0),
    ];

    const RULES: [(i16, (i16, i16)); 4] = [
        // "If there is no elf in the n, ne, or nw adjacent positions, the elf proposes moving north one step."
        (0b0000_0111, (0, 1)),
        // "if there is no elf in the s, se, or sw adjacent positions, the elf proposes moving south one step."
        (0b0111_0000, (0, -1)),
        // "if there is no elf in the w, nw, or sw adjacent positions, the elf proposes moving west one step."
        (0b1100_0001, (-1, 0)),
        // "if there is no elf in the e, ne, or se adjacent positions, the elf proposes moving east one step."
        (0b0001_1100, (1, 0)),
    ];

    const MAX_SIZE: usize = 384;
    const MAX_ELVES: usize = 10_000;
    const OFFSET: usize = MAX_SIZE / 2;
    const NO_CHOICE: (i16, i16) = (i16::MAX, i16::MAX);
    const NO_ELF: u16 = u16::MAX;

    let is_outside_max = |position: (i16, i16)| {
        position.0 < 0
            || position.1 < 0
            || position.0 >= MAX_SIZE as i16
            || position.1 >= MAX_SIZE as i16
    };

    let mut elves = input
        .text
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, c)| {
                (c == b'#').then_some(Elf {
                    position: (x as i16 + OFFSET as i16, y as i16 + OFFSET as i16),
                    to_move_choice: NO_CHOICE,
                })
            })
        })
        .collect::<Vec<_>>();

    if elves.len() > MAX_ELVES {
        return Err(format!("Too many elves - max {MAX_ELVES} supported"));
    }

    let mut elf_grid = vec![NO_ELF; MAX_SIZE * MAX_SIZE];
    for (elf_idx, elf) in elves.iter().enumerate() {
        if is_outside_max(elf.position) {
            return Err(format!("Elf is outside of [0,{MAX_SIZE})"));
        }
        elf_grid[elf.position.1 as usize * MAX_SIZE + elf.position.0 as usize] = elf_idx as u16;
    }

    #[cfg(feature = "visualization")]
    let mut max_coords = (0, 0);
    #[cfg(feature = "visualization")]
    let mut min_coords = (i16::MAX, i16::MAX);
    #[cfg(feature = "visualization")]
    let mut stable_elf_positions = String::from("const elfPositions = [");
    #[cfg(feature = "visualization")]
    let mut elf_initial_positions = Vec::new();
    #[cfg(feature = "visualization")]
    let mut elf_position_rect_ids = Vec::new();
    #[cfg(feature = "visualization")]
    {
        stable_elf_positions.push('[');
        for (idx, elf) in elves.iter().enumerate() {
            if idx > 0 {
                stable_elf_positions.push(',');
            }
            min_coords.0 = elf.position.0.min(min_coords.0);
            min_coords.1 = elf.position.0.min(min_coords.1);
            max_coords.0 = elf.position.0.max(max_coords.0);
            max_coords.1 = elf.position.0.max(max_coords.1);
            elf_initial_positions.push(elf.position);
            stable_elf_positions.push_str(&format!("[{},{}]", elf.position.0, elf.position.1));
        }
        stable_elf_positions.push(']');
    }

    for round in 0..input.part_values(10, 10000) {
        let mut num_moves = 0;

        for elf in elves.iter_mut() {
            let adjacent_bitmask = DIRECTIONS
                .iter()
                .enumerate()
                .fold(0, |acc, (idx, (dx, dy))| {
                    acc | if elf_grid
                        [(elf.position.1 + dy) as usize * MAX_SIZE + (elf.position.0 + dx) as usize]
                        == NO_ELF
                    {
                        0
                    } else {
                        1 << idx
                    }
                });

            // "During the first half of each round, each Elf considers the eight positions adjacent to themself.
            // If no other Elves are in one of those eight positions, the Elf does not do anything during this round."
            if adjacent_bitmask != 0 {
                for rule_offset in 0..RULES.len() {
                    let (check_mask, to_move) = RULES[(round + rule_offset) % RULES.len()];
                    if (check_mask & adjacent_bitmask) == 0 {
                        elf.to_move_choice = to_move;
                        break;
                    }
                }
            }
        }

        for elf_idx in 0..elves.len() {
            let elf = &mut elves[elf_idx];
            if elf.to_move_choice != NO_CHOICE {
                let to_move = elf.to_move_choice;
                elf.to_move_choice = NO_CHOICE;

                let new_position = (elf.position.0 + to_move.0, elf.position.1 + to_move.1);
                if is_outside_max(new_position) {
                    return Err(format!(
                        "Elf tried to moved outside of [0,{}): {:?}",
                        MAX_SIZE, new_position
                    ));
                }

                let elf_idx_at_position =
                    elf_grid[new_position.1 as usize * MAX_SIZE + new_position.0 as usize];

                if elf_idx_at_position == NO_ELF {
                    elf_grid[elf.position.1 as usize * MAX_SIZE + elf.position.0 as usize] = NO_ELF;
                    elf.position = new_position;
                    elf_grid[elf.position.1 as usize * MAX_SIZE + elf.position.0 as usize] =
                        elf_idx as u16;
                    num_moves += 1;
                } else {
                    // Position was occupied - stand still and push other elf (which must be coming from other direction) back:
                    elf_grid[new_position.1 as usize * MAX_SIZE + new_position.0 as usize] = NO_ELF;
                    let pushed_back_position =
                        (new_position.0 + to_move.0, new_position.1 + to_move.1);
                    elves[elf_idx_at_position as usize].position = pushed_back_position;
                    elf_grid[pushed_back_position.1 as usize * MAX_SIZE
                        + pushed_back_position.0 as usize] = elf_idx_at_position;
                    num_moves -= 1;
                }
            }
        }

        #[cfg(feature = "visualization")]
        {
            stable_elf_positions.push_str(",[");
            for (idx, elf) in elves.iter().enumerate() {
                if idx > 0 {
                    stable_elf_positions.push(',');
                }
                stable_elf_positions.push_str(&format!("[{},{}]", elf.position.0, elf.position.1));
                min_coords.0 = elf.position.0.min(min_coords.0);
                min_coords.1 = elf.position.1.min(min_coords.1);
                max_coords.0 = elf.position.0.max(max_coords.0);
                max_coords.1 = elf.position.1.max(max_coords.1);
            }
            stable_elf_positions.push(']');

            if num_moves == 0 || (input.is_part_one() && round == 9) {
                let mut svg = SvgImage::new();
                let step_duration_ms = 300;
                let animation_duration_ms = step_duration_ms - 100;
                svg.add(SvgStyle::new(format!("\n\
                    rect {{ fill: #00B1D2; transition: x {}ms, y {}ms, fill {}ms; }} rect.moving {{ fill: #FDDB27 !important; }}
                ", animation_duration_ms, animation_duration_ms, animation_duration_ms)));
                for initial_pos in elf_initial_positions.iter() {
                    elf_position_rect_ids.push(
                        svg.add_with_id(
                            SvgRect::default()
                                .x(initial_pos.0 as Coordinate)
                                .y(initial_pos.1 as Coordinate)
                                .width(1)
                                .height(1),
                        ),
                    );
                }

                stable_elf_positions.push_str("];");
                svg.add(SvgScript::new(format!("{}{}", stable_elf_positions, format!(
                    "\nconst elfRects = document.querySelectorAll('rect');\n\
                window.onNewStep = (step) => {{\n\
                        const prevPos = (step == 0) ? null : elfPositions[step-1];\n\
                        const pos = elfPositions[step];\n\
                        for (let i = 0; i < {}; i++) {{\n\
                            const e = elfRects[i];
                            e.setAttribute('x', pos[i][0]);\n\
                            e.setAttribute('y', pos[i][1]);\n\
                            if (prevPos === null || (prevPos[i][0] === pos[i][0] && prevPos[i][1] === pos[i][1])) {{\n\
                               e.classList.remove('moving');\n\
                            }} else {{\n\
                               e.classList.add('moving');\n\
                            }}\n\
                        }}\n\
                }};",
                    elves.len(),
                ))));
                input.rendered_svg.replace(
                    svg.view_box((
                        min_coords.0 as i64,
                        min_coords.1 as i64,
                        (max_coords.0 - min_coords.0) as i64,
                        (max_coords.1 - min_coords.1) as i64,
                    ))
                    .style("background: black;")
                    .data_attribute("steps".to_string(), format!("{}", round + 1))
                    .data_attribute("step-duration".to_string(), format!("{}", step_duration_ms))
                    .to_svg_string(),
                );
            }
        }

        if num_moves == 0 {
            return Ok(round + 1);
        }
    }

    let (min_x, max_x, min_y, max_y) =
        elves
            .iter()
            .fold((i16::MAX, i16::MIN, i16::MAX, i16::MIN), |acc, e| {
                (
                    acc.0.min(e.position.0),
                    acc.1.max(e.position.0),
                    acc.2.min(e.position.1),
                    acc.3.max(e.position.1),
                )
            });
    let rectangle_size = ((max_x + 1 - min_x) * (max_y + 1 - min_y)) as usize;
    Ok(rectangle_size - elves.len())
}

#[derive(Copy, Clone)]
struct Elf {
    position: (i16, i16),
    to_move_choice: (i16, i16),
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    test_part_one!(test_input => 110);
    test_part_two!(test_input => 20);

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => 3920);
    test_part_two!(real_input => 889);
}
