pub fn translate(input: &str) -> String {
    let mut output = vec![];
    let words = input.split(" ").collect::<Vec<&str>>();

    for word in words {
        if let Some(index_of_vowel) =
            word.find(|c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        {
            if let Some(0) = word.find("xr") {
                return format!("{}ay", word);
            }
            if let Some(0) = word.find("yt") {
                return format!("{}ay", word);
            }

            if index_of_vowel == 0 {
                output.push(format!("{}ay", word));
            } else {
                // check index_of_vowel vs index_of_y vs index_of_qu
                if let Some(index_of_y) = word.find("y") {
                    if index_of_y != 0 {
                        if index_of_y > index_of_vowel {
                            output.push(format!(
                                "{}{}ay",
                                word.chars().skip(index_of_vowel).collect::<String>(),
                                word.chars().take(index_of_vowel).collect::<String>()
                            ));
                        } else {
                            if let Some(index_of_qu) = word.find("qu") {
                                if index_of_qu > index_of_y {
                                    output.push(format!(
                                        "{}{}ay",
                                        word.chars().skip(index_of_y).collect::<String>(),
                                        word.chars().take(index_of_y).collect::<String>()
                                    ));
                                } else {
                                    output.push(format!(
                                        "{}{}ay",
                                        word.chars().skip(index_of_qu + 2).collect::<String>(),
                                        word.chars().take(index_of_qu + 2).collect::<String>()
                                    ));
                                }
                            } else {
                                output.push(format!(
                                    "{}{}ay",
                                    word.chars().skip(index_of_y).collect::<String>(),
                                    word.chars().take(index_of_y).collect::<String>()
                                ));
                            }
                        }
                    } else {
                        if let Some(index_of_qu) = word.find("qu") {
                            if index_of_qu > index_of_y {
                                output.push(format!(
                                    "{}{}ay",
                                    word.chars().skip(index_of_y).collect::<String>(),
                                    word.chars().take(index_of_y).collect::<String>()
                                ));
                            } else {
                                output.push(format!(
                                    "{}{}ay",
                                    word.chars().skip(index_of_qu + 2).collect::<String>(),
                                    word.chars().take(index_of_qu + 2).collect::<String>()
                                ));
                            }
                        } else {
                            output.push(format!(
                                "{}{}ay",
                                word.chars().skip(index_of_vowel).collect::<String>(),
                                word.chars().take(index_of_vowel).collect::<String>()
                            ));
                        }
                    }
                } else {
                    if let Some(index_of_qu) = word.find("qu") {
                        if index_of_qu > index_of_vowel {
                            output.push(format!(
                                "{}{}ay",
                                word.chars().skip(index_of_vowel).collect::<String>(),
                                word.chars().take(index_of_vowel).collect::<String>()
                            ));
                        } else {
                            output.push(format!(
                                "{}{}ay",
                                word.chars().skip(index_of_qu + 2).collect::<String>(),
                                word.chars().take(index_of_qu + 2).collect::<String>()
                            ));
                        }
                    } else {
                        output.push(format!(
                            "{}{}ay",
                            word.chars().skip(index_of_vowel).collect::<String>(),
                            word.chars().take(index_of_vowel).collect::<String>()
                        ));
                    }
                }
            }
        } else {
            if let Some(index_of_y) = input.find("y") {
                if let Some(index_of_qu) = input.find("qu") {
                    todo!()
                } else {
                    output.push(format!(
                        "{}{}ay",
                        input.chars().skip(index_of_y).collect::<String>(),
                        input.chars().take(index_of_y).collect::<String>()
                    ));
                }
            } else {
                todo!()
            }
        }
    }

    output.join(" ")
}
