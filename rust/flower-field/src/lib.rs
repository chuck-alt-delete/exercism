use std::collections::HashMap;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let mut count_map: HashMap<(usize, usize), u8> = HashMap::new();

    // First pass: find all flowers and increment their neighbors
    garden.iter().enumerate().for_each(|(row, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter(|&(_, &cell)| cell == b'*')
            .for_each(|(col, _)| {
                increment_neighbors(&mut count_map, garden, row, col);
            });
    });

    // build and return result
    garden
        .iter()
        .enumerate()
        .map(|(r, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(c, &cell)| match cell {
                    b'*' => '*',
                    b' ' => count_map
                        .get(&(r, c))
                        .map(|&count| (b'0' + count) as char)
                        .unwrap_or(' '),
                    _ => char::from(cell),
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
    // This function increments the neighbor count for a given flower.
    // It is written in a way that avoids bound checks by
    // 1. Using saturating_sub to handle negative indices gracefully.
    // 2. Using get and is_some_and to avoid invalid indices.

    let row_start = row.saturating_sub(1);
    let row_end = row + 2;
    let col_start = col.saturating_sub(1);
    let col_end = col + 2;

    for r in row_start..row_end {
        for c in col_start..col_end {
            // Skip the flower itself
            if (r, c) == (row, col) {
                continue;
            }

            // Only increment if it's a space (not another flower)
            if garden
                .get(r)
                .is_some_and(|line| line.as_bytes().get(c).is_some_and(|&cell| cell == b' '))
            {
                *count_map.entry((r, c)).or_insert(0) += 1;
            }
        }
    }
}
