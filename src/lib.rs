use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

const MEM_SIZE: usize = 30000;

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
                    position = matching_bracket(program, position);
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    position = matching_bracket(program, position);
                }
            }
            _ => (),
        }

        position += 1;
    }

    output
}

fn matching_bracket(program: &str, mut position: usize) -> usize {
    let current_instruction = program.chars().nth(position).unwrap();
    if current_instruction == ']' {
        let mut bracket_counter = 1;
        while bracket_counter > 0 {
            position -= 1;
            let current_instruction = program.chars().nth(position).unwrap();
            if current_instruction == '[' {
                bracket_counter -= 1;
            } else if current_instruction == ']' {
                bracket_counter += 1;
            }
        }
    } else {
        let mut bracket_counter = 1;
        while bracket_counter > 0 {
            position += 1;
            let current_instruction = program.chars().nth(position).unwrap();
            if current_instruction == ']' {
                bracket_counter += 1;
            } else if current_instruction == '[' {
                bracket_counter -= 1;
            }
        }
    }
    position
}
