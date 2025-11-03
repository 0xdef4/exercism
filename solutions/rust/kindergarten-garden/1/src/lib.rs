pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut output = Vec::new();
    let diagram_split = diagram.split("\n").collect::<Vec<&str>>();

    for row in diagram_split {
        output.push(row.chars().nth(find_student_index(student)).expect("index is always in range"));
        output.push(row.chars().nth(find_student_index(student) +1).expect("index is always in range"));
    }

    output.iter().map(|e| decode_plant(e)).collect::<Vec<_>>()
}

fn decode_plant(plant_encoding: &char) -> &'static str {
    match plant_encoding {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => unreachable!()
    }
}

fn find_student_index(student: &str) -> usize {
    let index = match student {
        "Alice" => 0,
        "Bob" => 2,
        "Charlie" => 4,
        "David" => 6,
        "Eve" => 8,
        "Fred" => 10,
        "Ginny" => 12,
        "Harriet" => 14,
        "Ileana" => 16,
        "Joseph" => 18,
        "Kincaid" => 20,
        "Larry" => 22,
        _ => unreachable!()
    };
    index
}