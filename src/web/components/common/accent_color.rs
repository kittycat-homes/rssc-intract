use rand::seq::SliceRandom;

lazy_static! {
    static ref COLORS: Vec<&'static str> = vec![
        "before:text-pink-500 decoration-pink-500 border-pink-500",
        "before:text-teal-500 decoration-teal-500 border-teal-500",
        "before:text-lime-500 decoration-lime-500 border-lime-500",
    ];
}

pub fn random_color(length: usize) -> Vec<&'static str> {
    let mut rng = rand::thread_rng();
    // unwrapping here is fine since COLORS is never empty
    let mut colors: Vec<&'static str> = vec![];
    for _i in 0..length {
        colors.push(COLORS.choose(&mut rng).unwrap())
    }
    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_color() {
        assert!(random_color(256).len() == 256);
        assert!(random_color(1).len() == 1);
    }
}
