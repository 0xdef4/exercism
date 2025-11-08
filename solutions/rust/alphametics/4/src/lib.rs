use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let parts: Vec<&str> = input.split(" == ").collect();
    let left = parts[0];
    let right = parts[1];

    let words: Vec<&str> = left.split(" + ").collect();
    let mut all_words = words.clone();
    all_words.push(right);

    // 모든 문자 추출
    let mut letters: Vec<char> = all_words.iter().flat_map(|w| w.chars()).collect();
    letters.sort_unstable();
    letters.dedup();

    // 선두 문자는 0이 되면 안 됨
    let mut leading = HashSet::new();
    for w in &all_words {
        leading.insert(w.chars().next().unwrap());
    }

    let mut map = HashMap::new();
    let mut used = [false; 10];

    fn word_value(word: &str, map: &HashMap<char, u8>) -> Option<i64> {
        let mut value = 0;
        for c in word.chars() {
            if let Some(&d) = map.get(&c) {
                value = value * 10 + d as i64;
            } else {
                return None;
            }
        }
        Some(value)
    }

    fn backtrack(
        idx: usize,
        letters: &[char],
        leading: &HashSet<char>,
        map: &mut HashMap<char, u8>,
        used: &mut [bool; 10],
        words: &[&str],
        right: &str,
    ) -> Option<HashMap<char, u8>> {
        if idx == letters.len() {
            // 모든 문자에 숫자 할당됨 → 계산 검증
            let sum: i64 = words.iter().map(|w| word_value(w, map).unwrap()).sum();
            if sum == word_value(right, map).unwrap() {
                return Some(map.clone());
            }
            return None;
        }

        let ch = letters[idx];
        for d in 0..=9 {
            // 이미 사용된 숫자면 패스
            if used[d as usize] {
                continue;
            }
            // 선두 문자가 0이면 안 됨
            if d == 0 && leading.contains(&ch) {
                continue;
            }

            used[d as usize] = true;
            map.insert(ch, d);
            if let Some(result) = backtrack(idx + 1, letters, leading, map, used, words, right) {
                return Some(result);
            }
            // 백트래킹
            used[d as usize] = false;
            map.remove(&ch);
        }

        None
    }

    backtrack(0, &letters, &leading, &mut map, &mut used, &words, right)
}
