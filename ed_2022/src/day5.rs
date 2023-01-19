use regex::Regex;

#[derive(Debug)]
pub struct CrateStack {
    crates: Vec<String>,
}

impl CrateStack {
    fn new() -> CrateStack {
        CrateStack { crates: vec![] }
    }
}

#[derive(Debug)]
pub struct Instruction {
    num_crates: i32,
    from: i32,
    to: i32,
}

impl Instruction {
    fn new(num_crates: i32, from: i32, to: i32) -> Instruction {
        Instruction {
            num_crates,
            from,
            to,
        }
    }
}

pub fn get_top_crates(data: String) -> String {
    let mut stacks: Vec<CrateStack> = get_crate_stacks(data.to_owned());
    let instructions = get_crane_instructions(data);

    for instr in instructions {
        let from_stack = stacks.get_mut((instr.from as usize) - 1).unwrap();
        let mut crates_to_move: Vec<String> = vec![];

        for _ in 0..instr.num_crates {
            let item = from_stack.crates.pop().unwrap();
            crates_to_move.push(item.to_owned());
        }

        let to_stack = stacks.get_mut((instr.to as usize) - 1).unwrap();
        for c in crates_to_move {
            to_stack.crates.push(c);
        }
    }

    let mut top_crates = "".to_owned();
    for stack in stacks {
        top_crates += stack.crates.last().unwrap();
    }
    top_crates
}

pub fn get_top_crates_9001(data: String) -> String {
    let mut stacks: Vec<CrateStack> = get_crate_stacks(data.to_owned());
    let instructions = get_crane_instructions(data);

    for instr in instructions {
        let from_stack = stacks.get_mut((instr.from as usize) - 1).unwrap();
        let mut crates_to_move: Vec<String> = vec![];

        for _ in 0..instr.num_crates {
            let item = from_stack.crates.pop().unwrap();
            crates_to_move.insert(0, item.to_owned());
        }

        let to_stack = stacks.get_mut((instr.to as usize) - 1).unwrap();
        for c in crates_to_move {
            to_stack.crates.push(c);
        }
    }

    let mut top_crates = "".to_owned();
    for stack in stacks {
        top_crates += stack.crates.last().unwrap();
    }
    top_crates
}

fn get_crate_stacks(data: String) -> Vec<CrateStack> {
    let parts: Vec<&str> = data.split("\n\n").collect();
    let crate_stacks_data: Vec<&str> = parts.first().unwrap().lines().collect();
    let stack_numbers: Vec<&str> = crate_stacks_data.last().unwrap().split("").collect();
    let mut stacks: Vec<CrateStack> = stack_numbers
        .iter()
        .filter(|s| !s.trim().is_empty())
        .map(|_| CrateStack::new())
        .collect();
    let re_letter = Regex::new(r"\w{1}").unwrap();

    let mut i = 0;
    while i < crate_stacks_data.len() - 1 {
        let line: Vec<&str> = crate_stacks_data.get(i).unwrap().split("").collect();
        let mut j = 0;

        while j < line.len() {
            let crate_letter = line.get(j).unwrap().to_owned();
            if re_letter.is_match(crate_letter) {
                let stack_number: i32 = stack_numbers.get(j).unwrap().parse().unwrap();
                stacks
                    .get_mut((stack_number as usize) - 1)
                    .unwrap()
                    .crates
                    .insert(0, crate_letter.to_owned());
            }
            j += 1;
        }
        i += 1;
    }

    stacks
}

fn get_crane_instructions(data: String) -> Vec<Instruction> {
    let parts: Vec<&str> = data.split("\n\n").collect();
    let instructions_data = parts.last().unwrap();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let instructions: Vec<Instruction> = instructions_data
        .lines()
        .map(|l| {
            let (mut num_crates, mut from, mut to): (i32, i32, i32) = (0, 0, 0);
            for cap in re.captures_iter(l) {
                num_crates = cap[1].parse().unwrap();
                from = cap[2].parse().unwrap();
                to = cap[3].parse().unwrap();
            }

            Instruction::new(num_crates, from, to)
        })
        .collect();

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: String,
    }

    #[test]
    fn it_should_return_top_crates() {
        let contents = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        let test_case = TestCase {
            input: contents,
            expected: String::from("CMZ"),
        };

        let result = get_top_crates(test_case.input);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_return_top_crates_9001() {
        let contents = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        let test_case = TestCase {
            input: contents,
            expected: String::from("MCD"),
        };

        let result = get_top_crates_9001(test_case.input);

        assert_eq!(result, test_case.expected);
    }
}
