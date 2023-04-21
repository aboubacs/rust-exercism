const MINE_CHAR: char = '*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, _)| get_result_at(minefield, i, j))
                .collect()
        })
        .collect()
}

fn get_result_at(minefield: &[&str], i: usize, j: usize) -> String {
    if is_mine(minefield, i, j) {
        MINE_CHAR.to_string()
    } else {
        let count = get_neighbouring_mine_count(minefield, i, j);
        get_output_for_count(count)
    }
}

fn is_mine(minefield: &[&str], i: usize, j: usize) -> bool {
    minefield[i].chars().nth(j).unwrap() == MINE_CHAR
}

fn get_neighbouring_mine_count(minefield: &[&str], i: usize, j: usize) -> i8 {
    let mut count = 0;
    for (x, y) in vec![
        (i as i32 - 1, j as i32 - 1),
        (i as i32 - 1, j as i32),
        (i as i32 - 1, j as i32 + 1),
        (i as i32, j as i32 - 1),
        (i as i32, j as i32 + 1),
        (i as i32 + 1, j as i32 - 1),
        (i as i32 + 1, j as i32),
        (i as i32 + 1, j as i32 + 1),
    ] {
        if is_in_minefield(minefield, x, y) && minefield[x as usize].chars().nth(y as usize).unwrap() == '*' {
            count += 1;
        }
    }
    count
}

fn is_in_minefield(minefield: &[&str], x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < minefield.len() as i32 && y < minefield[0].len() as i32
}

fn get_output_for_count(count: i8) -> String {
    match count {
        0 => " ".to_string(),
        _ => count.to_string(),
    }
}
