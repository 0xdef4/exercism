pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        // make outer
        let mut outer = vec![];
        if self.row_count == 0 {
            return outer;
        }
        outer.push(vec![1]);
        for i in 2..=self.row_count {
            // make inner
            let mut inner = vec![];
            for j in 1..=i {
                if j == 1 || j == i {
                    inner.push(1 as u32);
                }
                if inner.len() == (i - 1) as usize {
                    continue;
                }
                if j > 1 && j < i {
                    for e in  outer[(i - 2) as usize].windows(2).map(|w| w[0]+ w[1]) {
                        inner.push(e);
                    }
                }
            }
            outer.push(inner);
        }
        outer
    }
}
