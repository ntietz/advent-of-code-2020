use std::collections::HashSet;
use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let program = load_instructions("inputs/day8.txt");
    let (_, acc) = run_program(program);
    println!("day08.part1.solution = {}", acc);
}

pub fn part2() {
    let program = load_instructions("inputs/day8.txt");

    for (idx, instruction) in program.iter().enumerate() {
        let op = &instruction.0;
        if *op == Operation::Acc {
            continue;
        }

        let mut new_program = program.to_vec();
        new_program[idx].0 = match op {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            _ => panic!("Unmatched operation!"),
        };
        let (finished, acc) = run_program(new_program);

        if finished {
            println!("day08.part2.solution = {}", acc);
            break;
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}
type Instruction = (Operation, i32);

fn load_instructions(filename: &str) -> Vec<Instruction> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Instruction {
    let parts: Vec<_> = line.split(" ").collect();
    let op = match parts[0] {
        "nop" => Operation::Nop,
        "acc" => Operation::Acc,
        "jmp" => Operation::Jmp,
        _ => panic!("Unknown instruction"),
    };
    let arg = parts[1].parse().unwrap();
    (op, arg)
}

fn run_program(program: Vec<Instruction>) -> (bool, i32) {
    let mut pc: i32 = 0;
    let mut acc: i32 = 0;

    let mut previous_pcs = HashSet::new();

    while !previous_pcs.contains(&pc) {
        previous_pcs.insert(pc);

        if pc < 0 || (pc as usize) >= program.len() {
            break;
        }

        let (op, arg) = &program[pc as usize];
        //println!("pc={}, acc={}, op={:#?}, arg={}", pc, acc, op, arg);
        match op {
            Operation::Nop => {
                pc += 1;
            }
            Operation::Acc => {
                acc += arg;
                pc += 1;
            }
            Operation::Jmp => {
                pc += arg;
            }
        }
    }

    ((pc as usize) == program.len(), acc)
}
