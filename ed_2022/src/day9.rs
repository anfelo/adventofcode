use std::collections::HashMap;

#[derive(Debug)]
enum MotionDirection {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Motion {
    direction: MotionDirection,
    steps: i64,
}

#[derive(Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Knot {
    coord: Coord,
    char: String,
}

impl Knot {
    fn follow(&mut self, prev: &Knot) {
        let x_diff = prev.coord.x - self.coord.x;
        if x_diff.abs() > 1 {
            self.coord.x += if x_diff > 1 { 1 } else { -1 };

            if prev.coord.y - self.coord.y < 0 {
                self.coord.y -= 1;
            } else if prev.coord.y - self.coord.y > 0 {
                self.coord.y += 1;
            }
        }

        let y_diff = prev.coord.y - self.coord.y;
        if y_diff.abs() > 1 {
            self.coord.y += if y_diff > 1 { 1 } else { -1 };

            if prev.coord.x - self.coord.x < 0 {
                self.coord.x -= 1;
            } else if prev.coord.x - self.coord.x > 0 {
                self.coord.x += 1;
            }
        }
    }
}

#[derive(Debug)]
struct Board {
    max: Coord,
    min: Coord,
}

pub fn tail_unique_visits(data: String, knots_num: i64) -> i64 {
    let motions: Vec<Motion> = get_motions(data);

    let mut knots = get_initial_knots(knots_num);
    let mut board = Board {
        min: Coord { x: 0, y: 0 },
        max: Coord { x: 0, y: 0 },
    };
    let mut tail_visited = HashMap::from([("0,0".to_owned(), true)]);

    for motion in motions {
        for _ in 0..motion.steps {
            let knots_clone = knots.clone();
            let mut head = knots.first_mut().unwrap();
            let mut prev_knot = knots_clone.first().unwrap();

            match motion.direction {
                MotionDirection::Up => {
                    if head.coord.y == board.min.y {
                        board.min.y -= 1;
                    }
                    head.coord.y -= 1;
                }
                MotionDirection::Down => {
                    if head.coord.y == board.max.y {
                        board.max.y += 1;
                    }
                    head.coord.y += 1;
                }
                MotionDirection::Right => {
                    if head.coord.x == board.max.x {
                        board.max.x += 1;
                    }
                    head.coord.x += 1;
                }
                MotionDirection::Left => {
                    if head.coord.x == board.min.x {
                        board.min.x -= 1;
                    }
                    head.coord.x -= 1;
                }
            }

            for knot in knots.iter_mut() {
                if knot.char != "H".to_string() {
                    knot.follow(prev_knot);
                }
                prev_knot = knot;
            }

            tail_visited.insert(
                format!(
                    "{},{}",
                    knots.last().unwrap().coord.x,
                    knots.last().unwrap().coord.y
                ),
                true,
            );
        }
        // println!("-- final --");
        // print_rope_map(&board, &knots, &tail_visited);
    }
    tail_visited.keys().len() as i64
}

fn print_rope_map(board: &Board, knots: &Vec<Knot>, visited: &HashMap<String, bool>) {
    for y in board.min.y..board.max.y + 1 {
        for x in board.min.x..board.max.x + 1 {
            let mut item = ".";

            if visited.contains_key(&format!("{},{}", x, y)) {
                item = "#";
            }

            for knot in knots.iter().rev() {
                if knot.coord.y == y && knot.coord.x == x {
                    item = &knot.char;
                }
            }

            print!("{}", item);
        }
        println!("")
    }
}

fn get_initial_knots(num: i64) -> Vec<Knot> {
    let mut knots: Vec<Knot> = vec![];
    for i in 0..num {
        if i == 0 {
            knots.push(Knot {
                coord: Coord { x: 0, y: 0 },
                char: "H".to_string(),
            });
        } else if i == num - 1 {
            knots.push(Knot {
                coord: Coord { x: 0, y: 0 },
                char: "T".to_string(),
            });
        } else {
            knots.push(Knot {
                coord: Coord { x: 0, y: 0 },
                char: format!("{}", i),
            });
        }
    }
    knots
}

fn get_motions(data: String) -> Vec<Motion> {
    data.lines()
        .map(|l| {
            let line_parts: Vec<&str> = l.split(' ').collect();
            Motion {
                steps: line_parts.get(1).unwrap().parse::<i64>().unwrap(),
                direction: match line_parts.get(0).unwrap() {
                    &"R" => MotionDirection::Right,
                    &"L" => MotionDirection::Left,
                    &"U" => MotionDirection::Up,
                    &"D" => MotionDirection::Down,
                    _ => panic!("not valid motion direction"),
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: (String, i64),
        expected: i64,
    }

    #[test]
    fn it_should_return_tail_unique_visits_two_knots() {
        let test_case = TestCase {
            input: (
                String::from(
                    "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
                ),
                2,
            ),
            expected: 13,
        };

        let result = tail_unique_visits(test_case.input.0, test_case.input.1);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_return_tail_unique_visits_nine_knots() {
        let test_case = TestCase {
            input: (
                String::from(
                    "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
                ),
                10,
            ),
            expected: 1,
        };

        let result = tail_unique_visits(test_case.input.0, test_case.input.1);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_return_tail_unique_visits_nine_knots_large() {
        let test_case = TestCase {
            input: (
                String::from(
                    "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
                ),
                10,
            ),
            expected: 36,
        };

        let result = tail_unique_visits(test_case.input.0, test_case.input.1);

        assert_eq!(result, test_case.expected);
    }
}
