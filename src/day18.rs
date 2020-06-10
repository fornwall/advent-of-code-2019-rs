use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

type Key = u8;
type KeyBitset = u32;

struct Edge {
    steps: usize,
    needed_keys: KeyBitset,
    target_key: Key,
}

pub fn part1(input_string: &str) -> String {
    steps_to_gather_all_keys(input_string).to_string()
}

pub fn steps_to_gather_all_keys(input_string: &str) -> usize {
    type Position = (i32, i32);

    let mut map: HashMap<Position, char> = HashMap::new();
    let mut found_keys = HashMap::new();
    let mut all_keys_bitset = 0 as KeyBitset;

    input_string.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let current_position = (x as i32, y as i32);
            let char_to_insert = match c {
                '@' => {
                    // The single entrance is represented by '@'.
                    found_keys.insert(b'@', current_position);
                    '.'
                }
                'a'..='z' => {
                    // Keys are represented by lowercase letters.
                    let found_key = c as Key;
                    all_keys_bitset |= 1 << (found_key as usize - 'a' as usize);
                    found_keys.insert(found_key, current_position);
                    c
                }
                '#' => {
                    // Stone walls are represented as '#'.
                    return;
                }
                _ => c,
            };
            map.insert(current_position, char_to_insert);
        });
    });

    // Mapping to (other_key, needed_keys_to_reach, steps):
    let mut adjacency_list: HashMap<Key, Vec<Edge>> = HashMap::new();

    for (&this_key, &this_key_position) in found_keys.iter() {
        // Find path from this key to all other keys.

        // (position, bitset_of_needed_keys, steps):
        let mut to_visit = VecDeque::new();
        to_visit.push_back((this_key_position, 0u32, 0u32));

        let mut visited_positions = HashSet::new();
        visited_positions.insert(this_key_position);

        while let Some((position, needed_keys, steps)) = to_visit.pop_front() {
            'key_direction_loop: for direction in DIRECTIONS.iter() {
                let new_position = (position.0 + direction.0, position.1 + direction.1);
                let mut new_needed_keys = needed_keys;
                let mut found_key = None;

                match map.get(&new_position) {
                    Some(&char_at_position @ 'A'..='Z') => {
                        let needed_key = char_at_position.to_ascii_lowercase() as Key;
                        if found_keys.contains_key(&needed_key) {
                            // Only consider door as necessary if key is in quadrant.
                            // Needed by part 2, where we can wait until key is picked
                            // up in other quadrant.
                            let bit_value = 1 << (needed_key - b'a');
                            new_needed_keys |= bit_value;
                        }
                    }
                    Some(&char_at_position @ 'a'..='z') => {
                        found_key = Some(char_at_position as Key);
                    }
                    Some('.') => {
                        // Free to enter.
                    }
                    None => {
                        continue 'key_direction_loop;
                    }
                    Some(c) => {
                        panic!("Invalid map entry: {}", c);
                    }
                }

                let new_steps = steps + 1;
                let new_state = (new_position, new_needed_keys, new_steps);
                if visited_positions.insert(new_position) {
                    to_visit.push_back(new_state);

                    if let Some(target_key) = found_key {
                        adjacency_list
                            .entry(this_key)
                            .or_insert_with(Vec::new)
                            .push(Edge {
                                steps: new_steps as usize,
                                needed_keys: new_needed_keys,
                                target_key,
                            });
                    }
                }
            }
        }
    }

    shortest_path(&adjacency_list, all_keys_bitset).unwrap()
}

fn shortest_path(adjacency_list: &HashMap<Key, Vec<Edge>>, all_keys: KeyBitset) -> Option<usize> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Vertex {
        at_key: Key,
        steps: usize,
        gathered_keys: KeyBitset,
    }

    impl Ord for Vertex {
        fn cmp(&self, other: &Vertex) -> Ordering {
            other
                .steps
                .cmp(&self.steps)
                .then_with(|| self.gathered_keys.cmp(&other.gathered_keys))
                .then_with(|| self.at_key.cmp(&other.at_key))
        }
    }

    impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // From (key, gathered_keys) to total steps required to reach there.
    let mut cost_for_keys: HashMap<(Key, KeyBitset), usize> = HashMap::new();
    let mut to_visit = BinaryHeap::new();

    // We're at `@`, with a zero cost
    to_visit.push(Vertex {
        at_key: b'@',
        steps: 0,
        gathered_keys: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(Vertex {
        at_key: position,
        steps,
        gathered_keys,
    }) = to_visit.pop()
    {
        if gathered_keys == all_keys {
            return Some(steps);
        }

        for edge in adjacency_list.get(&position).unwrap() {
            if edge.needed_keys & gathered_keys != edge.needed_keys {
                // It's not possible to visit the target key if not all required keys has been gathered.
                continue;
            }

            let next = Vertex {
                steps: steps + edge.steps,
                at_key: edge.target_key,
                gathered_keys: gathered_keys | (1 << ((edge.target_key - b'a') as KeyBitset)),
            };

            let current_cost = cost_for_keys
                .entry((edge.target_key, next.gathered_keys))
                .or_insert(usize::max_value());
            let found_improved_path = next.steps < *current_cost;

            if found_improved_path {
                to_visit.push(next);
                *current_cost = next.steps;
            }
        }
    }

    // If we come here it's not possible to gather all keys.
    None
}

pub fn part2(input_string: &str) -> String {
    let mut map_top_left = String::new();
    let mut map_top_right = String::new();
    let mut map_bottom_left = String::new();
    let mut map_bottom_right = String::new();

    let num_rows = input_string.lines().count();
    let num_columns = input_string.lines().next().unwrap().len();
    let center_y = num_rows / 2;
    let center_x = num_columns / 2;

    input_string.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let replaced_char = match (center_x as i32 - x as i32, center_y as i32 - y as i32) {
                (0, 0) | (1, 0) | (-1, 0) | (0, 1) | (0, -1) => '#',
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => '@',
                _ => c,
            };

            if y <= center_y {
                if x <= center_x {
                    &mut map_top_left
                } else {
                    &mut map_top_right
                }
            } else if x <= center_x {
                &mut map_bottom_left
            } else {
                &mut map_bottom_right
            }
            .push(replaced_char);
        });
        if y <= center_y {
            map_top_left.push('\n');
            map_top_right.push('\n');
        } else {
            map_bottom_left.push('\n');
            map_bottom_right.push('\n');
        }
    });

    let result = steps_to_gather_all_keys(&map_top_left)
        + steps_to_gather_all_keys(&map_top_right)
        + steps_to_gather_all_keys(&map_bottom_left)
        + steps_to_gather_all_keys(&map_bottom_right);
    result.to_string()
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        part1(
            "#########
    #b.A.@.a#
    #########"
        ),
        "8"
    );

    assert_eq!(
        part1(
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
        ),
        "86"
    );

    assert_eq!(part1(include_str!("day18_input.txt")), "4248");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day18_input.txt")), "1878");
}
