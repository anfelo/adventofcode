#[derive(Debug)]
struct Instruction {
    amount: i64,
    cycles: i64,
}

pub fn calculate_signal_strengths(data: String) -> i64 {
    let instructions: Vec<Instruction> = get_instructions(data);

    let mut instr_idx = 0;
    let mut cycle = 1;
    let mut cycles_processed = 1;
    let mut strength = 0;
    let mut register = 1;
    let signals = [20, 60, 100, 140, 180, 220];

    loop {
        match instructions.get(instr_idx) {
            Some(i) => {
                if signals.contains(&cycle) {
                    strength += register * cycle;
                }

                if i.cycles == cycles_processed {
                    instr_idx += 1;
                    cycles_processed = 0;
                    register += i.amount;
                }
            }
            None => break,
        }

        cycle += 1;
        cycles_processed += 1;
    }

    strength
}

pub fn get_crt_pixels(data: String) -> String {
    let instructions: Vec<Instruction> = get_instructions(data);
    let crt_max_cols = 40;
    let crt_max_rows = 6;
    let mut instr_idx = 0;
    let mut cycle = 1;
    let mut cycles_processed = 1;
    let mut register = 1;
    let mut crt_screen: Vec<Vec<&str>> = vec![vec!["#"]];
    let mut curr_row = 0;

    loop {
        match instructions.get(instr_idx) {
            Some(i) => {
                if i.cycles == cycles_processed {
                    instr_idx += 1;
                    cycles_processed = 0;
                    register += i.amount;
                }
            }
            None => break,
        }
        let offset = curr_row * crt_max_cols;
        let cursor = [
            &register - 1 + offset,
            register + offset,
            &register + 1 + offset,
        ];

        if cursor.contains(&cycle) {
            crt_screen[curr_row as usize].push("#");
        } else {
            crt_screen[curr_row as usize].push(".");
        }

        cycle += 1;
        cycles_processed += 1;

        if cycle >= (crt_max_cols * (curr_row + 1)) as i64 {
            crt_screen[curr_row as usize].push("\n");
            curr_row += 1;
            crt_screen.push(vec![]);
        }

        if curr_row >= crt_max_rows {
            break;
        }
    }

    crt_screen.iter().map(|r| r.join("")).collect()
}

fn get_instructions(data: String) -> Vec<Instruction> {
    data.lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            match parts[0] {
                "addx" => Instruction {
                    amount: parts[1].parse::<i64>().unwrap(),
                    cycles: 2,
                },
                "noop" => Instruction {
                    amount: 0,
                    cycles: 1,
                },
                _ => panic!("instruction not supported"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: i64,
    }

    fn get_data() -> String {
        String::from(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        )
    }

    #[test]
    fn it_should_calculate_signal_strength() {
        let test_case = TestCase {
            input: get_data(),
            expected: 13140,
        };

        let result = calculate_signal_strengths(test_case.input);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_get_crt_pixels() {
        let input = get_data();
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        let result = get_crt_pixels(input);

        assert_eq!(result, expected);
    }
}
