#[derive(Clone)]
pub struct Section {
    pub start: i32,
    pub end: i32,
}

impl Section {
    pub fn new(start: i32, end: i32) -> Section {
        Self { start, end }
    }

    pub fn contains(&self, section: &Section) -> bool {
        self.start <= section.start && self.end >= section.end
    }

    pub fn overlaps(&self, section: &Section) -> bool {
        self.start <= section.start && self.end >= section.start
            || self.start <= section.end && self.end >= section.end
    }
}

pub struct Assignment {
    pub pair: [Section; 2],
}

impl Assignment {
    pub fn new(line: String) -> Assignment {
        let sections: Vec<Section> = line
            .split(',')
            .map(|s| {
                let section: Vec<&str> = s.split('-').collect();

                Section::new(
                    section.first().unwrap().parse().unwrap(),
                    section.get(1).unwrap().parse().unwrap(),
                )
            })
            .collect();

        Assignment {
            pair: [
                sections.first().unwrap().to_owned(),
                sections.get(1).unwrap().to_owned(),
            ],
        }
    }
}

pub fn fully_covered_count(data: String) -> i32 {
    let assignments: Vec<Assignment> = data
        .lines()
        .map(|line| Assignment::new(line.to_owned()))
        .collect();
    let mut sum: i32 = 0;

    for a in assignments {
        if a.pair[0].contains(&a.pair[1]) || a.pair[1].contains(&a.pair[0]) {
            sum += 1;
        }
    }

    sum
}

pub fn overlaping_count(data: String) -> i32 {
    let assignments: Vec<Assignment> = data
        .lines()
        .map(|line| Assignment::new(line.to_owned()))
        .collect();
    let mut sum: i32 = 0;

    for a in assignments {
        if a.pair[0].overlaps(&a.pair[1]) || a.pair[1].overlaps(&a.pair[0]) {
            sum += 1;
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
    fn it_should_return_fully_covered_count() {
        let contents = String::from(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );
        let test_case = TestCase {
            input: contents,
            expected: 2,
        };

        let result = fully_covered_count(test_case.input);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_return_overlaping_assignments() {
        let contents = String::from(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );
        let test_case = TestCase {
            input: contents,
            expected: 4,
        };

        let result = overlaping_count(test_case.input);

        assert_eq!(result, test_case.expected);
    }
}
