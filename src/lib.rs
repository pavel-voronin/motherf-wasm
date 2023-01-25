use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

const MEM_SIZE: usize = 30000;

enum SearchDirection {
    FORWARD,
    BACKWARD,
}

#[wasm_bindgen]
pub fn interpret(program: &str, input: &str) -> String {
    console_error_panic_hook::set_once();

    let mut memory = [0; MEM_SIZE];
    let mut pointer: usize = 0;

    let mut position: usize = 0;
    let mut output = String::from("");

    let mut input_position: usize = 0;

    while position < program.len() {
        let current_instruction = program.chars().nth(position).unwrap();

        match current_instruction {
            '>' => {
                pointer += 1;
                if pointer == MEM_SIZE {
                    pointer = 0;
                }
            }
            '<' => {
                if pointer == 0 {
                    pointer = MEM_SIZE - 1;
                } else {
                    pointer -= 1;
                }
            }
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '.' => output.push(memory[pointer] as u8 as char),
            ',' => {
                memory[pointer] = input.chars().nth(input_position).unwrap() as u8;
                input_position += 1;
            }
            '[' => {
                if memory[pointer] == 0 {
                    position = find_matching_bracket_for_opening(program, position);
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    position = find_matching_bracket_for_closing(program, position);
                }
            }
            _ => (),
        }

        position += 1;
    }

    output
}

fn find_matching_bracket(program: &str, mut position: usize, direction: SearchDirection) -> usize {
    let mut bracket_counter = 1;

    while bracket_counter > 0 {
        match direction {
            SearchDirection::FORWARD => position += 1,
            SearchDirection::BACKWARD => position -= 1,
        }

        let current_instruction = program
            .chars()
            .nth(position)
            .unwrap_or_else(|| panic!("out of code"));

        if current_instruction == '[' {
            bracket_counter -= 1;
        } else if current_instruction == ']' {
            bracket_counter += 1;
        }
    }

    position
}

fn find_matching_bracket_for_opening(program: &str, position: usize) -> usize {
    find_matching_bracket(program, position, SearchDirection::FORWARD)
}

fn find_matching_bracket_for_closing(program: &str, position: usize) -> usize {
    find_matching_bracket(program, position, SearchDirection::BACKWARD)
}
