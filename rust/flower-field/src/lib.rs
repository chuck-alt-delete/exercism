use std::collections::HashMap;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        // Garden is empty, so return empty vector
        return Vec::new();
    }
    let cols = garden[0].len();

    let mut count_map: HashMap<(usize, usize), u8> = HashMap::new();

    // First pass: find all flowers and increment their neighbors
    for (row, line) in garden.iter().enumerate() {
        for (col, &cell) in line.as_bytes().iter().enumerate() {
            if cell == b'*' {
                increment_neighbors(&mut count_map, garden, row, col);
            }
        }
    }

    // build and return result
    let mut result = Vec::with_capacity(rows);
    for r in 0..rows {
        let mut row_str = String::with_capacity(cols);
        for c in 0..cols {
            let cell = garden[r].as_bytes()[c];
            match cell {
                b'*' => row_str.push('*'),
                b' ' => match count_map.get(&(r, c)) {
                    Some(&count) => row_str.push((b'0' + count) as char),
                    None => row_str.push(' '),
                },
                _ => row_str.push(char::from(cell)),
            }
        }
        result.push(row_str);
    }

    result
}

fn increment_neighbors(
    count_map: &mut HashMap<(usize, usize), u8>,
    garden: &[&str],
    row: usize,
    col: usize,
) {
    let rows = garden.len();
    let cols = garden.get(0).map_or(0, |r| r.len());

    let row_start = row.saturating_sub(1);
    let row_end = (row + 2).min(rows);
    let col_start = col.saturating_sub(1);
    let col_end = (col + 2).min(cols);

    for r in row_start..row_end {
        for c in col_start..col_end {
            // Skip the flower itself
            if (r, c) == (row, col) {
                continue;
            }

            // Only increment if it's a space (not another flower)
            if garden[r].as_bytes()[c] == b' ' {
                *count_map.entry((r, c)).or_insert(0) += 1;
            }
        }
    }
}
