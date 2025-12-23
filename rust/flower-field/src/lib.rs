use std::collections::HashMap;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    // Initialize a HashMap to store the count of flowers around each square.
    // This count will be updated as we come across new flowers.
    // Flower locations will have a count of -1.
    let mut count_map: HashMap<(i32, i32), i32> = HashMap::new();

    // Iterate through the garden
    garden.iter().enumerate().for_each(|(row_index, &row)| {
        row.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(col_index, &byte)| {
                if byte == b'*' {
                    update_flower_neighbors(
                        garden,
                        &mut count_map,
                        (row_index as i32, col_index as i32),
                    );
                } else if byte == b' ' {
                    count_map
                        .entry((row_index as i32, col_index as i32))
                        .or_insert(0);
                }
            });
    });

    let mut result = Vec::new();
    garden.iter().enumerate().for_each(|(row_index, &row)| {
        let mut row_string = String::new();
        row.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(col_index, &byte)| match byte {
                b'*' => row_string.push('*'),
                b' ' => {
                    let count = count_map
                        .get(&(row_index as i32, col_index as i32))
                        .expect("Count map should contain all positions");
                    if *count == 0 {
                        row_string.push(' ');
                    } else {
                        row_string.push_str(&count.to_string());
                    }
                }
                _ => {}
            });
        result.push(row_string);
    });

    result
}

fn update_flower_neighbors(
    garden: &[&str],
    count_map: &mut HashMap<(i32, i32), i32>,
    (row, column): (i32, i32),
) {
    // Increase count for all non-flower neighbors.

    // We have visited a flower. Mark flower location with -1
    count_map.entry((row, column)).or_insert(-1);

    let neighbors = vec![
        (row - 1, column - 1),
        (row - 1, column),
        (row - 1, column + 1),
        (row, column - 1),
        (row, column + 1),
        (row + 1, column - 1),
        (row + 1, column),
        (row + 1, column + 1),
    ];

    for (r, c) in neighbors {
        if r < 0 || c < 0 || r >= garden.len() as i32 || c >= garden[0].len() as i32 {
            continue;
        }
        println!("{:?}", count_map);

        let cell = garden
            .get(r as usize)
            .and_then(|row| row.as_bytes().get(c as usize));

        match cell {
            Some(&b' ') => {
                count_map
                    .entry((r, c))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            Some(&b'*') if count_map.get(&(r, c)).is_none() => {
                // Skip flower neighbors.
                continue;
            }
            None => {
                // Out of bounds, skip
                continue;
            }
            _ => {}
        }
    }
}
