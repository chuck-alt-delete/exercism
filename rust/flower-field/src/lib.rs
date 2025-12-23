use std::collections::HashMap;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let mut count_map: HashMap<(usize, usize), u8> = HashMap::new();

    // First pass: find all flowers and increment their neighbors
    for (row, line) in garden.iter().enumerate() {
        for (col, &cell) in line.as_bytes().iter().enumerate() {
            if cell == b'*' {
                increment_neighbors(&mut count_map, garden, row, col);
            }
        }
    }

    // Second pass: build output directly using garden dimensions
    let rows = garden.len();
    let cols = garden.get(0).map_or(0, |r| r.len());

    (0..rows)
        .map(|row| {
            (0..cols)
                .map(|col| {
                    let cell = garden[row].as_bytes()[col];
                    match cell {
                        b'*' => '*',
                        b' ' => match count_map.get(&(row, col)) {
                            Some(&count) => char::from_digit(count as u32, 10).unwrap(),
                            None => ' ',
                        },
                        _ => char::from(cell),
                    }
                })
                .collect()
        })
        .collect()
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
