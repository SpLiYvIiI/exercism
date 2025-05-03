const OFFSET_Y: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const OFFSET_X: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if height > 0 { minefield[0].len() } else { 0 };

    let mut result = Vec::with_capacity(height);

    for i in 0..height {
        let mut row: Vec<char> = minefield[i].chars().collect();
        for j in 0..width {
            if row[j] == '*' {
                continue;
            }
            let mut count = 0;
            for k in 0..8 {
                let ny = i as i32 + OFFSET_Y[k];
                let nx = j as i32 + OFFSET_X[k];

                if ny < 0 || ny >= height as i32 || nx < 0 || nx >= width as i32 {
                    continue;
                }

                if minefield[ny as usize].chars().nth(nx as usize).unwrap() == '*' {
                    count += 1;
                }
            }
            if count > 0 {
                row[j] = char::from_digit(count, 10).unwrap();
            }
        }
        result.push(row.iter().collect());
    }

    result
}
