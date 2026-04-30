fn to_roman(n: u32) -> String {
    if n == 0 {
        return "○".to_string();
    }
    assert!(n < 4000, "to_roman: {n} is out of range (0–3999)");
    const TABLE: &[(u32, &str)] = &[
        (1000, "m"),
        (900, "cm"),
        (500, "d"),
        (400, "cd"),
        (100, "c"),
        (90, "xc"),
        (50, "l"),
        (40, "xl"),
        (10, "x"),
        (9, "ix"),
        (5, "v"),
        (4, "iv"),
        (1, "i"),
    ];
    let mut result = String::new();
    let mut remaining = n;
    for &(value, symbol) in TABLE {
        while remaining >= value {
            result.push_str(symbol);
            remaining -= value;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn zero_returns_circle() {
        assert_eq!(to_roman(0), "○");
    }

    #[test]
    fn additive_sequences() {
        assert_eq!(to_roman(1), "i");
        assert_eq!(to_roman(2), "ii");
        assert_eq!(to_roman(3), "iii");
        assert_eq!(to_roman(8), "viii");
    }

    #[test]
    fn subtractive_pairs() {
        assert_eq!(to_roman(4), "iv");
        assert_eq!(to_roman(9), "ix");
        assert_eq!(to_roman(40), "xl");
        assert_eq!(to_roman(90), "xc");
        assert_eq!(to_roman(400), "cd");
        assert_eq!(to_roman(900), "cm");
    }

    #[test]
    fn clock_boundary_values() {
        assert_eq!(to_roman(12), "xii");
        assert_eq!(to_roman(23), "xxiii"); // max 24h hour
        assert_eq!(to_roman(59), "lix"); // max minute / second
    }

    #[test]
    fn date_values() {
        assert_eq!(to_roman(31), "xxxi"); // max day
        assert_eq!(to_roman(2026), "mmxxvi"); // current year
        assert_eq!(to_roman(3999), "mmmcmxcix"); // upper bound
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn four_thousand_panics() {
        to_roman(4000);
    }
}
