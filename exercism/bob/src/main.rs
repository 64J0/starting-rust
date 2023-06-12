fn main() {
    println!("Question: {:?}", is_question("Question?"));
    println!("No Question: {:?}", is_question("Question!"));
    println!("Yell: {:?}", is_yell("YELLING AT YOU!"));
    println!("No Yell: {:?}", is_yell("Not yelling at you!"));
    println!("Test silence: {:?}", reply("										"));
    println!("No letters: {:?}", reply("1, 2, 3"));
}

enum BobAnswer {
    Question,
    Yell,
    YellQuestion,
    Silence,
    AnythingElse,
}

fn is_question(message: &str) -> bool {
    message.trim().ends_with('?')
}

fn is_yell(message: &str) -> bool {
    let trimmed_message = message.trim();
    let uppercase_message = trimmed_message.to_uppercase();
    let filtered_message = trimmed_message
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>();

    !is_silence(&filtered_message) && uppercase_message == trimmed_message
}

fn is_silence(message: &str) -> bool {
    message.trim().is_empty()
}

fn translate_bob(message: &str) -> BobAnswer {
    let q = is_question(message);
    let y = is_yell(message);
    let s = is_silence(message);

    if q && y {
        BobAnswer::YellQuestion
    } else if q {
        BobAnswer::Question
    } else if y {
        BobAnswer::Yell
    } else if s {
        BobAnswer::Silence
    } else {
        BobAnswer::AnythingElse
    }
}

pub fn reply(message: &str) -> &str {
    match translate_bob(message) {
        BobAnswer::YellQuestion => "Calm down, I know what I'm doing!",
        BobAnswer::Yell => "Whoa, chill out!",
        BobAnswer::Question => "Sure.",
        BobAnswer::Silence => "Fine. Be that way!",
        BobAnswer::AnythingElse => "Whatever.",
    }
}
