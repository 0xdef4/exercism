pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }
    let rows = garden.len();
    let cols = garden[0].len();
    let grid: Vec<Vec<char>> = garden.iter().map(|str| str.chars().collect()).collect();
    let direction: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),            (0, 1),
        (1, -1),  (1, 0), (1, 1),
    ];

    let mut output = vec![];
    for i in 0..rows {
        let mut row_str = vec![];
        for j in 0..cols {
            if grid[i][j] == '*' {
                row_str.push('*');
            } else {
                let mut count = 0;
                for (di, dj) in direction {
                    if (i as isize) + di >= 0
                        && (j as isize) + dj >= 0
                        && (i as isize) + di < rows as isize
                        && (j as isize) + dj < cols as isize
                        && grid[((i as isize) + di) as usize][((j as isize) + dj) as usize] == '*'
                    {
                        count += 1;
                    }
                }
                if count == 0 {
                    row_str.push(' ')
                } else {
                    row_str.push((b'0' + count) as char)
                }
            }
        }
        output.push(row_str.iter().collect());
    }
    output
}
