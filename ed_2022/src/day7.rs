use std::collections::HashMap;

pub fn total_dirs_size(data: String, limit: i64) -> i64 {
    let dirs_sizes = get_dir_sizes(data);

    Vec::from_iter(dirs_sizes.values().map(|s| s.to_owned()))
        .into_iter()
        .filter(|s| s <= &limit)
        .sum()
}

pub fn dir_size_to_free(data: String, disk_space: i64, unused: i64) -> i64 {
    let dirs_sizes = get_dir_sizes(data);
    let available_space = disk_space - dirs_sizes.get("/").unwrap();

    Vec::from_iter(dirs_sizes.values().map(|s| s.to_owned()))
        .into_iter()
        .filter(|s| s + available_space >= unused)
        .min()
        .unwrap()
}

fn get_dir_sizes(data: String) -> HashMap<String, i64> {
    let lines: Vec<&str> = data.lines().collect();
    let mut dirs_sizes: HashMap<String, i64> = HashMap::new();
    let mut curr_dir: Vec<String> = vec![];

    for line in lines {
        let line_parts: Vec<&str> = line.split(' ').collect();

        if line.starts_with('$') {
            if line_parts.get(1).unwrap() == &"cd" {
                if line_parts.get(2).unwrap() == &".." {
                    curr_dir.pop();
                } else if line_parts.get(2).unwrap() != &"/" {
                    curr_dir.push(format!("{}/", line_parts.get(2).unwrap()));
                } else {
                    curr_dir.push(line_parts.get(2).unwrap().to_string());
                    dirs_sizes.insert(curr_dir.join(""), 0);
                }
            }
        } else if line.starts_with("dir") {
            dirs_sizes.insert(
                format!("{}{}/", curr_dir.join(""), line_parts.get(1).unwrap()),
                0,
            );
        } else {
            let mut index = 0;
            let dir_slice = curr_dir.as_slice();

            while index < curr_dir.len() {
                if let Some(sum) = dirs_sizes.get_mut(&dir_slice[0..index + 1].join("")) {
                    *sum += line_parts.first().unwrap().parse::<i64>().unwrap();
                }

                index += 1;
            }
        }
    }

    dirs_sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: i64,
    }

    #[test]
    fn it_should_get_total_dirs_size_upto_100000() {
        let test_case = TestCase {
            input: String::from(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
            ),
            expected: 95437,
        };

        let result = total_dirs_size(test_case.input, 100000);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_get_dir_space_to_free() {
        let test_case = TestCase {
            input: String::from(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
            ),
            expected: 24933642,
        };

        let result = dir_size_to_free(test_case.input, 70000000, 30000000);

        assert_eq!(result, test_case.expected);
    }
}
