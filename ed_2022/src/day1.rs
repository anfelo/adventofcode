/*
The Elves take turns writing down the number of Calories contained by the various meals, snacks,
rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory
from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
This list represents the Calories of the food carried by five Elves:

The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
The second Elf is carrying one food item with 4000 Calories.
The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
The fifth Elf is carrying one food item with 10000 Calories.
In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like
to know how many Calories are being carried by the Elf carrying the most Calories. In the example above,
this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
*/
pub fn max_calories(data: String) -> i32 {
    let elfs_calories: Vec<&str> = data.split("\n\n").collect();
    let mut max = 0;

    for per_elf in &elfs_calories {
        let calories: Vec<i32> = per_elf
            .split('\n')
            .filter(|cal| !cal.is_empty())
            .map(|cal| cal.trim().parse::<i32>().unwrap())
            .collect();

        let sum: i32 = calories.iter().sum();

        if sum > max {
            max = sum;
        }
    }

    max
}

/*
By the time you calculate the answer to the Elves' question, they've already realized that the Elf
carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried
by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks,
they still have two backups.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/
pub fn top_three_calories(data: String) -> i32 {
    let elfs_calories: Vec<&str> = data.split("\n\n").collect();
    let mut elfs_cal_sum: Vec<i32> = vec![];

    for per_elf in &elfs_calories {
        let calories: Vec<i32> = per_elf
            .split('\n')
            .filter(|cal| !cal.is_empty())
            .map(|cal| cal.trim().parse::<i32>().unwrap())
            .collect();

        let sum: i32 = calories.iter().sum();

        elfs_cal_sum.push(sum)
    }

    elfs_cal_sum.sort_by(|a, b| b.cmp(a));

    elfs_cal_sum[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: i32,
    }

    #[test]
    fn it_should_return_max_calories() {
        let contents = String::from(
            "1000
2000
3000

4000 

5000
6000

7000
8000
9000

10000",
        );
        let test_case = TestCase {
            input: contents,
            expected: 24000,
        };

        let result = max_calories(test_case.input);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_return_top_three_calories_count() {
        let contents = String::from(
            "1000
2000
3000

4000 

5000
6000

7000
8000
9000

10000",
        );
        let test_case = TestCase {
            input: contents,
            expected: 45000,
        };

        let result = top_three_calories(test_case.input);

        assert_eq!(result, test_case.expected);
    }
}
