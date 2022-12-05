#[derive(Debug)]
pub struct Rucksack {
    pub left_pocket: Vec<Item>,
    pub right_pocket: Vec<Item>,
}

impl Rucksack {
    pub fn new(items: String) -> Rucksack {
        let item_names: Vec<&str> = items.split("").filter(|i| !i.is_empty()).collect();
        let pockets: Vec<Vec<Item>> = item_names
            .chunks(item_names.len() / 2)
            .map(|s| s.iter().map(|i| Item::new(i)).collect())
            .collect();

        let left_pocket = pockets.get(0).unwrap().to_vec();
        let right_pocket = pockets.get(1).unwrap().to_vec();

        Rucksack {
            left_pocket,
            right_pocket,
        }
    }

    pub fn get_misplaced_items(&self) -> Vec<Item> {
        let mut misplaced: Vec<Item> = vec![];

        for i in &self.left_pocket {
            for j in &self.right_pocket {
                if i.name == j.name && !misplaced.contains(i) {
                    misplaced.push(i.clone());
                }
            }
        }

        misplaced
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Item {
    pub prio: i32,
    pub name: String,
}

impl Item {
    pub fn new(item_name: &str) -> Item {
        Item {
            prio: get_item_prio(item_name),
            name: item_name.to_owned(),
        }
    }
}

fn get_item_prio(name: &str) -> i32 {
    let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    match items.chars().position(|i| i.to_string() == name) {
        Some(i) => (i as i32) + 1,
        None => 0,
    }
}

pub fn sum_misplaced_prio(data: String) -> i32 {
    // split lines and map to rucksacks
    let rucksacks: Vec<Rucksack> = data
        .lines()
        .map(|line| Rucksack::new(line.to_owned()))
        .collect();
    let mut sum: i32 = 0;

    // loop over the rucksacks
    for rucksack in &rucksacks {
        // get misplaced items for rucksack
        let misplaced_items = rucksack.get_misplaced_items();
        // sum the prio for each item
        if misplaced_items.len() > 0 {
            sum += misplaced_items
                .into_iter()
                .map(|a| a.prio)
                .reduce(|a, b| a + b)
                .unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: i32,
    }

    #[test]
    fn it_should_return_total_misplaced_prio() {
        let contents = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        let test_case = TestCase {
            input: contents,
            expected: 157,
        };

        let result = sum_misplaced_prio(test_case.input);

        assert_eq!(result, test_case.expected);
    }
}
