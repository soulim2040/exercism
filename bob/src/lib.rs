pub fn reply(message: &str) -> &str {
    let after_trim = message.trim();
    if after_trim.is_empty() {
        return "Fine. Be that way!";
    }

    let has_letter = after_trim.chars().any(|c| c.is_alphabetic());
    if after_trim.chars().last().unwrap() == '?' {
        if after_trim.to_uppercase() == after_trim &&  has_letter {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }else if after_trim.to_uppercase() == after_trim  &&  has_letter {
        return "Whoa, chill out!";
    }else{
        return "Whatever.";
    }
}
