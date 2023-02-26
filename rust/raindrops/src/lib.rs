pub fn raindrops(n: u32) -> String {
    let sound = |divisor, sound| (n % divisor == 0).then_some(sound);
    let sounds = [
        sound(3, "Pling"),
        sound(5, "Plang"),
        sound(7, "Plong"),
    ];

    if let [None, None, None] = sounds {
        return n.to_string();
    }

    sounds.into_iter().flatten().collect()
}
