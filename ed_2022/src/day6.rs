struct Packet {
    seq: String,
}

impl Packet {
    fn new(seq: &str) -> Packet {
        Packet {
            seq: String::from(seq),
        }
    }

    fn is_start(&self) -> bool {
        self.has_n_unique(4)
    }

    fn is_message(&self) -> bool {
        self.has_n_unique(14)
    }

    fn has_n_unique(&self, n: usize) -> bool {
        let mut packet_chars: Vec<char> = vec![];
        let mut result = false;

        for char in self.seq.chars() {
            if packet_chars.contains(&char) {
                break;
            }
            packet_chars.push(char)
        }

        if packet_chars.len() == n {
            result = true;
        }

        result
    }
}

pub fn get_start_packet_marker(data: String) -> i32 {
    let mut marker: usize = 0;

    loop {
        if marker == (data.len() - 3) {
            break;
        }

        if Packet::new(&data[marker..marker + 4]).is_start() {
            break;
        }

        marker += 1;
    }

    (marker + 4) as i32
}

pub fn get_start_message_marker(data: String) -> i32 {
    let mut marker: usize = 0;

    loop {
        if marker == (data.len() - 13) {
            break;
        }

        if Packet::new(&data[marker..marker + 14]).is_message() {
            break;
        }

        marker += 1;
    }

    (marker + 14) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: i32,
    }

    #[test]
    fn it_should_get_start_packet_marker() {
        let test_cases: [TestCase; 4] = [
            TestCase {
                input: String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
                expected: 7,
            },
            TestCase {
                input: String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
                expected: 5,
            },
            TestCase {
                input: String::from("nppdvjthqldpwncqszvftbrmjlhg"),
                expected: 6,
            },
            TestCase {
                input: String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
                expected: 10,
            },
        ];

        for test_case in test_cases {
            let result = get_start_packet_marker(test_case.input);

            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn it_should_get_start_message_marker() {
        let test_cases: [TestCase; 4] = [
            TestCase {
                input: String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
                expected: 19,
            },
            TestCase {
                input: String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
                expected: 23,
            },
            TestCase {
                input: String::from("nppdvjthqldpwncqszvftbrmjlhg"),
                expected: 23,
            },
            TestCase {
                input: String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
                expected: 29,
            },
        ];

        for test_case in test_cases {
            let result = get_start_message_marker(test_case.input);

            assert_eq!(result, test_case.expected);
        }
    }
}
