use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Tree {
    is_visible: bool,
}

pub fn trees_visible(data: String) -> i64 {
    get_trees_map(data)
        .values()
        .filter(|v| v.is_visible)
        .collect::<Vec<&Tree>>()
        .len() as i64
}

// pub fn max_visible_trees(data: String) -> i64 {
//     get_trees_map(data)
//         .values()
//         .map(|v| v.left * v.right * v.top * v.bottom)
//         .max()
//         .unwrap() as i64
// }

fn is_visible(items: &[i64], item: &i64) -> bool {
    let mut result = true;
    for other in items.into_iter() {
        if other >= item {
            result = false;
            break;
        }
    }
    result
}

fn get_trees_map(data: String) -> HashMap<String, Tree> {
    let rows: Vec<Vec<i64>> = data
        .lines()
        .into_iter()
        .map(|s| {
            s.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let mut trees_visible: HashMap<String, Tree> = HashMap::new();

    for i in 0..rows.len() {
        let row = &rows[i];
        let mut col_idx = 0;

        while col_idx < row.len() {
            let left_slice = &row[0..col_idx];
            let right_slice = &row[col_idx + 1..];
            let mut visible_left = true;
            let mut visible_right = true;

            if col_idx > 0 {
                visible_left = is_visible(&left_slice, &row[col_idx]);
                visible_right = is_visible(&right_slice, &row[col_idx]);
            }

            trees_visible
                .entry(format!("{},{}", i, col_idx))
                .and_modify(|e| {
                    if !e.is_visible {
                        e.is_visible = visible_left || visible_right;
                    }
                })
                .or_insert(Tree {
                    is_visible: visible_left || visible_right,
                });
            col_idx += 1;
        }
    }

    for i in 0..rows[0].len() {
        let column: Vec<i64> = rows.clone().into_iter().flat_map(|r| [r[i]]).collect();
        let mut row_idx = 0;

        while row_idx < column.len() {
            let bottom_slice = &column[row_idx + 1..];
            let top_slice = &column[0..row_idx + 1];
            let mut visible_top = true;
            let mut visible_bottom = true;

            if row_idx > 0 {
                visible_top = is_visible(&top_slice, &column[row_idx]);
                visible_bottom = is_visible(&bottom_slice, &column[row_idx]);
            }

            trees_visible
                .entry(format!("{},{}", row_idx, i))
                .and_modify(|e| {
                    if !e.is_visible {
                        e.is_visible = visible_top || visible_bottom;
                    }
                })
                .or_insert(Tree {
                    is_visible: visible_top || visible_bottom,
                });
            row_idx += 1;
        }
    }
    trees_visible
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: i64,
    }

    #[test]
    fn it_should_return_visible_trees() {
        let test_case = TestCase {
            input: String::from(
                "30373
25512
65332
33549
35390",
            ),
            expected: 21,
        };

        let result = trees_visible(test_case.input);

        assert_eq!(result, test_case.expected);
    }

    //     #[test]
    //     fn it_should_return_max_visible_trees() {
    //         let test_case = TestCase {
    //             input: String::from(
    //                 "30373
    // 25512
    // 65332
    // 33549
    // 35390",
    //             ),
    //             expected: 8,
    //         };
    //
    //         let result = max_visible_trees(test_case.input);
    //
    //         assert_eq!(result, test_case.expected);
    //     }

    // #[test]
    // fn it_should_check_visible_item() {
    //     let test_case = {
    //
    //     }
    // }
}
