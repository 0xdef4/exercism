use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut map = HashMap::new();
    let mut output = Vec::new();
    output.push(String::from(
        "Team                           | MP |  W |  D |  L |  P",
    ));

    if match_results.len() == 0 {
        return output.join("\n");
    }

    for e in match_results.split("\n") {
        let mut line_vec = e.split(";").collect::<Vec<_>>();

        let line_vec_split_off = line_vec.split_off(2);
        for (i, ele) in line_vec.iter().enumerate() {
            let team_stats = TeamStats::new();
            map.entry(*ele).or_insert(team_stats);
            map.entry(*ele)
                .and_modify(|e: &mut TeamStats| match line_vec_split_off[0] {
                    "win" => {
                        if i == 0 {
                            e.add_win();
                        }
                        if i == 1 {
                            e.add_loss();
                        }
                    }
                    "draw" => {
                        e.add_draw();
                    }
                    "loss" => {
                        if i == 0 {
                            e.add_loss();
                        }
                        if i == 1 {
                            e.add_win();
                        }
                    }
                    _ => {}
                });
        }
    }

    let mut result_of_game = Vec::new();
    for e in map.iter() {
        let length_of_name = e.0.len();
        let length_of_point = e.1.p.to_string().len();
        let white_space_for_name = " ".repeat(31 - length_of_name);
        let white_space_for_point = " ".repeat(3 - length_of_point);
        result_of_game.push(format!(
            "{}{}|  {:?} |  {:?} |  {:?} |  {:?} |{}{:?}",
            e.0, white_space_for_name, e.1.mp, e.1.w, e.1.d, e.1.l, white_space_for_point, e.1.p
        ));
    }

    result_of_game.sort_by(|a, b| {
        let pa = a.split("|").last().unwrap().trim().parse::<u32>().unwrap();
        let pb = b.split("|").last().unwrap().trim().parse::<u32>().unwrap();

        pb.cmp(&pa) // 점수 내림차순
            .then_with(|| a[..31].trim().cmp(b[..31].trim())) // 이름 오름차순
    });

    for e in result_of_game {
        output.push(e);
    }
    output.join("\n")
}

#[derive(Debug)]
struct TeamStats {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

const POINTS_BY_RESULT: [u32; 3] = [3, 1, 0];

fn calculate_score(team_stats: &TeamStats) -> u32 {
    POINTS_BY_RESULT[0] * team_stats.w
        + POINTS_BY_RESULT[1] * team_stats.d
        + POINTS_BY_RESULT[2] * team_stats.l
}

impl TeamStats {
    pub fn new() -> Self {
        Self {
            mp: 0,
            w: 0,
            d: 0,
            l: 0,
            p: 0,
        }
    }
    pub fn add_win(&mut self) -> &mut Self {
        self.mp += 1;
        self.w += 1;
        self.p = calculate_score(self);
        self
    }
    pub fn add_draw(&mut self) -> &mut Self {
        self.mp += 1;
        self.d += 1;
        self.p = calculate_score(self);
        self
    }
    pub fn add_loss(&mut self) -> &mut Self {
        self.mp += 1;
        self.l += 1;
        self.p = calculate_score(self);
        self
    }
}
