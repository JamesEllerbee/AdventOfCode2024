mod instruction;

use crate::mull_it_over::instruction::{Instruction, Operation};
use regex::Regex;
use std::fs;

pub fn solve_mull_it_over() {
    let input = fs::read_to_string("day_3/input.txt").expect("Could not read input.txt.");
    let instructions = parse_input(input);
    let sum_of = instructions
        .iter()
        .map(|instr| instr.execute())
        .sum::<i32>();

    println!("sum of mults {}", sum_of);
}

pub fn parse_input(input: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let input_re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").expect("Could not compile regex");
    let operands_re = Regex::new(r"\d+").expect("Could not compile regex");

    let mut do_op = true;
    for captures in input_re.captures_iter(&input) {
        let (instruction, []) = captures.extract();

        println!("instruction {}", instruction);

        if instruction.contains("mul") && do_op {
            let mut operands: Vec<i32> = Vec::new();
            for operands_caps in operands_re.captures_iter(instruction) {
                let (operand, []) = operands_caps.extract();

                operands.push(operand.parse().expect("Could not parse operand."));
            }

            instructions.push(Instruction {
                operation: Operation::Multiply,
                operands,
            });
        } else if instruction.contains("don't") {
            do_op = false;
        } else if instruction.contains("do") {
            do_op = true;
        }
    }

    instructions
}
