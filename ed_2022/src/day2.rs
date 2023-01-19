use std::collections::HashMap;

pub fn total_score_own_strat(data: String) -> i32 {
    let plays: Vec<&str> = data.split('\n').filter(|p| !p.is_empty()).collect();
    let mut score: i32 = 0;

    for play in &plays {
        let players: Vec<&str> = play.split(' ').collect();
        let own_hand = players[1];

        score += get_hand_value(own_hand);
        score += get_own_strategy_result(play);
    }

    score
}

pub fn total_score_elf_strat(data: String) -> i32 {
    let plays: Vec<&str> = data.split('\n').filter(|p| !p.is_empty()).collect();
    let mut score: i32 = 0;

    for play in &plays {
        let players: Vec<&str> = play.split(' ').collect();
        let opponent_hand = players[0];
        let strat = players[1];
        let own_hand = &get_own_hand(opponent_hand, strat);

        score += get_hand_value(own_hand);
        score += get_own_strategy_result(&format!("{} {}", opponent_hand, own_hand));
    }

    score
}

fn get_own_hand(opponent_hand: &str, strat: &str) -> String {
    match strat {
        "X" => {
            if opponent_hand == "A" {
                "Z".to_owned()
            } else if opponent_hand == "B" {
                "X".to_owned()
            } else {
                "Y".to_owned()
            }
        }
        "Y" => {
            if opponent_hand == "A" {
                "X".to_owned()
            } else if opponent_hand == "B" {
                "Y".to_owned()
            } else {
                "Z".to_owned()
            }
        }
        "Z" => {
            if opponent_hand == "A" {
                "Y".to_owned()
            } else if opponent_hand == "B" {
                "Z".to_owned()
            } else {
                "X".to_owned()
            }
        }
        _ => "".to_owned(),
    }
}

fn get_hand_value(hand: &str) -> i32 {
    let hands = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    match hands.get(hand) {
        Some(score) => score.to_owned(),
        None => 0,
    }
}

fn get_own_strategy_result(play: &str) -> i32 {
    let play_results = HashMap::from([
        ("A X", 3),
        ("A Y", 6),
        ("A Z", 0),
        ("B X", 0),
        ("B Y", 3),
        ("B Z", 6),
        ("C X", 6),
        ("C Y", 0),
        ("C Z", 3),
    ]);

    match play_results.get(play) {
        Some(score) => score.to_owned(),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: i32,
    }

    #[test]
    fn it_should_return_total_score_own_strat() {
        let contents = String::from(
            "A Y
B X
C Z",
        );
        let test_case = TestCase {
            input: contents,
            expected: 15,
        };

        let result = total_score_own_strat(test_case.input);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_return_total_score_elf_strat() {
        let contents = String::from(
            "A Y
B X
C Z
A X",
        );
        let test_case = TestCase {
            input: contents,
            expected: 15,
        };

        let result = total_score_elf_strat(test_case.input);

        assert_eq!(result, test_case.expected);
    }
}
