#[derive(Clone, Debug)]
pub struct Rucksack {
    pub items: Vec<Item>,
}

impl Rucksack {
    pub fn new(items: String) -> Rucksack {
        let item_names: Vec<&str> = items.split("").filter(|i| !i.is_empty()).collect();
        let all_items: Vec<Item> = item_names.iter().map(|i| Item::new(i)).collect();

        Rucksack { items: all_items }
    }

    pub fn get_misplaced_items(&self) -> Vec<Item> {
        let mut misplaced: Vec<Item> = vec![];

        let left_pocket = self.get_left_pocket();
        let right_pocket = self.get_right_pocket();

        for i in &left_pocket {
            for j in &right_pocket {
                if i.name == j.name && !misplaced.contains(i) {
                    misplaced.push(i.clone());
                }
            }
        }

        misplaced
    }

    pub fn get_left_pocket(&self) -> Vec<Item> {
        let pockets: Vec<Vec<Item>> = self
            .items
            .chunks(self.items.len() / 2)
            .map(|s| s.into())
            .collect();

        pockets.get(0).unwrap().to_vec()
    }

    pub fn get_right_pocket(&self) -> Vec<Item> {
        let pockets: Vec<Vec<Item>> = self
            .items
            .chunks(self.items.len() / 2)
            .map(|s| s.into())
            .collect();

        pockets.get(1).unwrap().to_vec()
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

#[derive(Debug)]
pub struct ElfGroup {
    pub rucksacks: [Rucksack; 3],
}

impl ElfGroup {
    pub fn new(elf1: Rucksack, elf2: Rucksack, elf3: Rucksack) -> ElfGroup {
        ElfGroup {
            rucksacks: [elf1, elf2, elf3],
        }
    }

    pub fn get_item_badge(&self) -> Option<Item> {
        let elf1 = &self.rucksacks[0];
        let elf2 = &self.rucksacks[1];
        let elf3 = &self.rucksacks[2];

        for item in &elf1.items {
            if elf2.items.contains(&item) && elf3.items.contains(&item) {
                return Some(item.to_owned());
            }
        }

        None
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

pub fn sum_elf_groups_prio(data: String) -> i32 {
    let rucksacks: Vec<Rucksack> = data
        .lines()
        .map(|line| Rucksack::new(line.to_owned()))
        .collect();
    let rucksacks_groups: Vec<Vec<Rucksack>> = rucksacks.chunks(3).map(|r| r.into()).collect();

    match rucksacks_groups
        .iter()
        .map(|g| {
            ElfGroup::new(
                g.get(0).unwrap().to_owned(),
                g.get(1).unwrap().to_owned(),
                g.get(2).unwrap().to_owned(),
            )
            .get_item_badge()
            .unwrap()
            .prio
        })
        .reduce(|a, b| a + b)
    {
        Some(s) => s,
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

    #[test]
    fn it_should_return_total_elf_group_prio() {
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
            expected: 70,
        };

        let result = sum_elf_groups_prio(test_case.input);

        assert_eq!(result, test_case.expected);
    }
}
