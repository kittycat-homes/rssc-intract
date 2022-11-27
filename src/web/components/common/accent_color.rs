use rand::seq::SliceRandom;

lazy_static! {
    static ref COLORS: Vec<&'static str> = vec![
        "before:text-pink-600 decoration-pink-600 dark:before:text-pink-50 dark:decoration-pink-50",
        "decoration-teal-600 dark:decoration-teal-50",
        "decoration-teal-600 dark:decoration-teal-50",
        "decoration-lime-600 dark:decoration-lime-50",
    ];
}

pub fn random_color(length: usize) -> Vec<&'static str> {
    let mut rng = rand::thread_rng();
    // unwrapping here is fine since COLORS is never empty
    vec![COLORS.choose(&mut rng).unwrap(); length]
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
