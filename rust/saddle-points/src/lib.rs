pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_max = Vec::with_capacity(input.len());
    let mut col_min = Vec::new();
    for (row, vec) in input.into_iter().enumerate() {
        for (col, &value) in vec.into_iter().enumerate() {
            if col_min.len() <= col {
                col_min.push((value, vec![row]))
            } else if col_min[col].0 == value {
                col_min[col].1.push(row)
            } else if col_min[col].0 > value {
                col_min[col] = (value, vec![row])
            }

            if row_max.len() <= row {
                row_max.push((value, vec![col]))
            } else if row_max[row].0 == value {
                row_max[row].1.push(col)
            } else if row_max[row].0 < value {
                row_max[row] = (value, vec![col])
            }
        }
    }

    let mut saddle_points = Vec::new();

    for (row, values) in row_max.into_iter().enumerate() {
        for col in values.1 {
            if col_min[col].1.contains(&row) {
                saddle_points.push((row, col))
            }
        }
    }

    saddle_points
}
