pub fn reply(message: &str) -> &str {
    if message.is_empty()
        || message.chars().all(|char| char == ' ')
        || message.contains("\t")
    {
        return "Fine. Be that way!";
    }
    if Some('?') == message.trim().chars().last()
        && message
            .chars()
            .filter(|char| char.is_alphabetic())
            .all(|char| char.is_uppercase())
        && message.chars().any(|char| char.is_alphabetic())
    {
        return "Calm down, I know what I'm doing!";
    }
    if Some('?') == message.trim().chars().last() {
        return "Sure.";
    }
    if message
        .chars()
        .filter(|char| char.is_alphabetic())
        .all(|char| char.is_uppercase())
        && message.chars().any(|char| char.is_alphabetic())
    {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}
