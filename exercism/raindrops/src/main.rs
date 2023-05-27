struct RaindropInput {
    input: u32,
    factor: u32,
    sound: String,
}

fn get_sound(r: RaindropInput) -> String {
    match r.input % r.factor == 0 {
        true => r.sound,
        false => String::from(""),
    }
}

pub fn raindrops(input: u32) -> String {
    let raindrop3 = RaindropInput {
        input,
        factor: 3,
        sound: "Pling".to_string(),
    };

    let raindrop5 = RaindropInput {
        factor: 5,
        sound: "Plang".to_string(),
        ..raindrop3
    };

    let raindrop7 = RaindropInput {
        factor: 7,
        sound: "Plong".to_string(),
        ..raindrop3
    };

    let final_sound = [raindrop3, raindrop5, raindrop7].map(get_sound).join("");

    match String::is_empty(&final_sound) {
        true => input.to_string(),
        false => final_sound,
    }
}

fn main() {
    println!("28: {:?}", raindrops(28));
    println!("30: {:?}", raindrops(30));
    println!("34: {:?}", raindrops(34));
}
