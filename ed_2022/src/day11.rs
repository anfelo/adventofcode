#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum OpType {
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspected_count: u64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Operation {
    op_type: OpType,
    left: Option<u64>,
    right: Option<u64>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Test {
    divisible_by: u64,
    throw_true: usize,
    throw_false: usize,
}

pub fn monkey_business_level(monkeys: &mut [Monkey], rounds: u64, with_relief: bool) -> u64 {
    for _ in 0..rounds {
        let monkeys_clone = monkeys.to_owned();
        for (i, monkey) in monkeys_clone.iter().enumerate() {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.remove(0);

                let left_op = match monkey.operation.left {
                    Some(v) => v,
                    None => item,
                };
                let right_op = match monkey.operation.right {
                    Some(v) => v,
                    None => item,
                };

                let new_item = match monkey.operation.op_type {
                    OpType::Add => left_op + right_op,
                    OpType::Substract => left_op - right_op,
                    OpType::Multiply => left_op * right_op,
                    OpType::Divide => left_op / right_op,
                };

                let new_worry_level = if with_relief {
                    ((new_item as f64) / 3.0).floor() as u64
                } else {
                     new_item % monkeys.iter().map(|m| m.test.divisible_by).product::<u64>()
                };

                if new_worry_level % monkey.test.divisible_by == 0 {
                    monkeys[monkey.test.throw_true].items.push(new_worry_level);
                } else {
                    monkeys[monkey.test.throw_false].items.push(new_worry_level);
                }

                monkeys[i].inspected_count += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));
    monkeys.iter().take(2).map(|m| m.inspected_count).product()
}

pub fn parse_input(data: String) -> Vec<Monkey> {
    data.split("\n\n")
        .into_iter()
        .map(|b| {
            let lines = b.lines().collect::<Vec<&str>>();

            let items = lines[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .into_iter()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();

            let op_parts: Vec<&str> = lines[2]
                .strip_prefix("  Operation: new = ")
                .unwrap()
                .split(' ')
                .collect();
            let operation = Operation {
                op_type: match op_parts[1] {
                    "+" => OpType::Add,
                    "-" => OpType::Substract,
                    "*" => OpType::Multiply,
                    "/" => OpType::Divide,
                    _ => panic!("Operation not allowed"),
                },
                left: match op_parts[0].parse::<u64>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                },
                right: match op_parts[2].parse::<u64>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                },
            };

            let divisible_by = lines[3]
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let throw_true = lines[4]
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let throw_false = lines[5]
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Monkey {
                items,
                operation,
                test: Test {
                    divisible_by,
                    throw_true,
                    throw_false,
                },
                inspected_count: 0,
            }
        })
        .collect::<Vec<Monkey>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<Monkey>,
        expected: u64,
    }

    fn get_data() -> String {
        return String::from(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );
    }

    #[test]
    fn it_should_parse_day11_input_data() {
        let expected = vec![
            Monkey {
                items: vec![79, 98],
                operation: Operation {
                    op_type: OpType::Multiply,
                    left: None,
                    right: Some(19),
                },
                test: Test {
                    divisible_by: 23,
                    throw_true: 2,
                    throw_false: 3,
                },
                inspected_count: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Operation {
                    op_type: OpType::Add,
                    left: None,
                    right: Some(6),
                },
                test: Test {
                    divisible_by: 19,
                    throw_true: 2,
                    throw_false: 0,
                },
                inspected_count: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Operation {
                    op_type: OpType::Multiply,
                    left: None,
                    right: None,
                },
                test: Test {
                    divisible_by: 13,
                    throw_true: 1,
                    throw_false: 3,
                },
                inspected_count: 0,
            },
            Monkey {
                items: vec![74],
                operation: Operation {
                    op_type: OpType::Add,
                    left: None,
                    right: Some(3),
                },
                test: Test {
                    divisible_by: 17,
                    throw_true: 0,
                    throw_false: 1,
                },
                inspected_count: 0,
            },
        ];

        let result = parse_input(get_data());

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_calculate_monkey_business_level_20_rounds() {
        let mut test_case = TestCase {
            input: parse_input(get_data()),
            expected: 10605,
        };

        let result = monkey_business_level(&mut test_case.input, 20, true);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_calculate_monkey_business_level_10000_rounds_no_relief() {
        let mut test_case = TestCase {
            input: parse_input(get_data()),
            expected: 2713310158,
        };

        let result = monkey_business_level(&mut test_case.input, 10000, false);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_calculate_monkey_business_level_20_rounds_no_relief() {
        let mut test_case = TestCase {
            input: parse_input(get_data()),
            expected: 10197,
        };

        let result = monkey_business_level(&mut test_case.input, 20, false);

        assert_eq!(result, test_case.expected);
    }
}
