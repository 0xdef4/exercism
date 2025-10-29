pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut bottle_song = String::new();
    for i in (3..11).rev() {
        bottle_song += format!(
            "{} green bottles hanging on the wall,
{} green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {} green bottles hanging on the wall.\n\n",
            get_in_string(i),
            get_in_string(i),
            get_in_string(i - 1).to_lowercase()
        ).as_str();
    }
    bottle_song += concat!(
        "Two green bottles hanging on the wall,
Two green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be one green bottle hanging on the wall.\n\n"
,
"One green bottle hanging on the wall,
One green bottle hanging on the wall,
And if one green bottle should accidentally fall,
There'll be no green bottles hanging on the wall.");

    let bottle_song_collection = bottle_song.split("\n\n").collect::<Vec<_>>();

    bottle_song_collection[(bottle_song_collection.len() - start_bottles as usize)
        ..(bottle_song_collection.len() - start_bottles as usize + (take_down as usize))]
        .join("\n\n")
}

fn get_in_string(num: u32) -> String {
    match num {
        10 => String::from("Ten"),
        9 => String::from("Nine"),
        8 => String::from("Eight"),
        7 => String::from("Seven"),
        6 => String::from("Six"),
        5 => String::from("Five"),
        4 => String::from("Four"),
        3 => String::from("Three"),
        2 => String::from("Two"),
        1 => String::from("One"),
        0 => String::from("no"),
        _ => String::new(),
    }
}
