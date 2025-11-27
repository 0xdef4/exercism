fn is_target_max_in_row(target: u64, row: &Vec<u64>) -> bool {
    row.iter().all(|e| *e <= target)
}

fn is_target_min_in_col(target: u64, col: &Vec<u64>) -> bool {
    col.iter().all(|e| *e >= target)
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut output = vec![];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if is_target_max_in_row(input[i][j], &input[i]) && is_target_min_in_col(input[i][j], 
                &input.iter().map(|e| *e.iter().nth(j).unwrap()).collect::<Vec<_>>()
            ) {
                output.push((i,j));
            }
        }
    }
    output
}
