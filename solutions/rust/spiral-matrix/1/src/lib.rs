pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir_idx = 0;

    let mut output = vec![vec![0; size as usize]; size as usize];
    
    let mut row: isize = 0;
    let mut col: isize = 0;
    for num in 1..=(size * size) {
        output[row as usize][col as usize] = num;

        row += dirs[dir_idx].0 as isize;
        col += dirs[dir_idx].1 as isize;

        if !(0..size).contains(&(row as u32))
            || !(0..size).contains(&(col as u32))
            || output[row as usize][col as usize] != 0
        {
            row -= dirs[dir_idx].0 as isize;
            col -= dirs[dir_idx].1 as isize;
            dir_idx = (dir_idx + 1) % 4;
            row += dirs[dir_idx].0 as isize;
            col += dirs[dir_idx].1 as isize;
        }
    }
    output
}
