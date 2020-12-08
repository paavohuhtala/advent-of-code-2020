use std::collections::HashSet;

fn main() {
    let input = include_str!("input8.txt");
    println!("a: {}", day8a(input));
    println!("b: {}", day8b(input));
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

struct Vm {
    instructions: Vec<Instruction>,
    acc: i32,
    pc: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VmState {
    Running,
    Terminated,
}

impl Vm {
    fn from_instructions(instructions: Vec<Instruction>) -> Vm {
        Vm {
            instructions,
            acc: 0,
            pc: 0,
        }
    }

    fn run_instruction(&mut self) -> VmState {
        if self.pc >= self.instructions.len() {
            return VmState::Terminated;
        }

        let instruction = &self.instructions[self.pc];

        match instruction {
            Instruction::Nop(_) => {
                self.pc += 1;
            }
            Instruction::Acc(x) => {
                self.pc += 1;
                self.acc += x;
            }
            Instruction::Jmp(offset) => {
                self.pc = (self.pc as i64 + *offset as i64) as usize;
            }
        };

        VmState::Running
    }
}

fn decode_program(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();

            let number: i32 = parts
                .get(1)
                .unwrap()
                .trim_start_matches('+')
                .parse()
                .unwrap();

            match *parts.get(0).unwrap() {
                "nop" => Instruction::Nop(number),
                "acc" => Instruction::Acc(number),
                "jmp" => Instruction::Jmp(number),
                _ => panic!(),
            }
        })
        .collect()
}

fn day8a(input: &str) -> i32 {
    let instructions = decode_program(input);
    let mut vm = Vm::from_instructions(instructions);
    let mut visited_addresses = HashSet::new();

    loop {
        let address = vm.pc;

        if !visited_addresses.insert(address) {
            return vm.acc;
        }

        vm.run_instruction();
    }
}

fn day8b(input: &str) -> i32 {
    let instructions = decode_program(input);

    'select_instruction: for i in 0..instructions.len() {
        let mutated_instruction = match &instructions[i] {
            Instruction::Acc(_) => continue,
            Instruction::Nop(n) => Instruction::Jmp(*n),
            Instruction::Jmp(n) => Instruction::Nop(*n),
        };

        let mut instructions = instructions.clone();
        instructions[i] = mutated_instruction;

        let mut vm = Vm::from_instructions(instructions.clone());
        let mut visited_addresses = HashSet::new();

        loop {
            let address = vm.pc;

            if !visited_addresses.insert(address) {
                continue 'select_instruction;
            }

            match vm.run_instruction() {
                VmState::Running => {}
                VmState::Terminated => return vm.acc,
            };
        }
    }

    0
}
