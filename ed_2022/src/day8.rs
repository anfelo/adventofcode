use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Tree {
    is_visible: bool,
    top: i64,
    bottom: i64,
    left: i64,
    right: i64,
}

pub fn trees_visible(data: String) -> i64 {
    get_trees_map(data)
        .values()
        .filter(|v| v.is_visible)
        .collect::<Vec<&Tree>>()
        .len() as i64
}

pub fn max_visible_trees(data: String) -> i64 {
    get_trees_map(data)
        .values()
        .map(|v| v.left * v.right * v.top * v.bottom)
        .max()
        .unwrap() as i64
}

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

fn num_visible(items: &[i64], item: &i64) -> i64 {
    let mut count = 0;
    for other in items.into_iter() {
        count += 1;
        if other >= item {
            break;
        }
    }
    count
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
            let mut row_clone = row.clone();
            let left_slice = &mut row_clone[0..col_idx];
            left_slice.reverse();
            let right_slice = &row[col_idx + 1..];

            let mut visible_left = true;
            let mut visible_right = true;

            let num_visible_left = num_visible(left_slice, &row[col_idx]);
            let num_visible_right = num_visible(right_slice, &row[col_idx]);

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
                    e.right = num_visible_right;
                    e.left = num_visible_left;
                })
                .or_insert(Tree {
                    is_visible: visible_left || visible_right,
                    right: num_visible_right,
                    left: num_visible_left,
                    top: 0,
                    bottom: 0,
                });
            col_idx += 1;
        }
    }

    for i in 0..rows[0].len() {
        let column: Vec<i64> = rows.clone().into_iter().flat_map(|r| [r[i]]).collect();
        let mut row_idx = 0;

        while row_idx < column.len() {
            let mut column_clone = column.clone();
            let top_slice = &mut column_clone[0..row_idx];
            top_slice.reverse();
            let bottom_slice = &column[row_idx + 1..];

            let mut visible_top = true;
            let mut visible_bottom = true;

            let num_visible_top = num_visible(top_slice, &column[row_idx]);
            let num_visible_bottom = num_visible(bottom_slice, &column[row_idx]);

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
                    e.top = num_visible_top;
                    e.bottom = num_visible_bottom;
                })
                .or_insert(Tree {
                    is_visible: visible_top || visible_bottom,
                    top: num_visible_top,
                    bottom: num_visible_bottom,
                    right: 0,
                    left: 0,
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

    #[test]
    fn it_should_return_max_visible_trees() {
        let test_case = TestCase {
            input: String::from(
                "30373
25512
65332
33549
35390",
            ),
            expected: 8,
        };

        let result = max_visible_trees(test_case.input);

        assert_eq!(result, test_case.expected);
    }

    #[test]
    fn it_should_return_num_visible_trees() {
        let result1 = num_visible(&[3, 3], &5);
        let result2 = num_visible(&[3, 5, 3], &5);
        let result3 = num_visible(&[5], &5);
        let result4 = num_visible(&[], &5);
        let result5 = num_visible(&[3, 3, 5, 6], &2);

        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
        assert_eq!(result3, 1);
        assert_eq!(result4, 0);
        assert_eq!(result5, 1);
    }
}
