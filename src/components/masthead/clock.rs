use chrono::Datelike as _;
use chrono::Timelike as _;
use leptos::prelude::*;

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

fn format_time(now: chrono::DateTime<chrono::Local>) -> String {
    format!(
        "{} · {} · {}",
        to_roman(now.hour()),
        to_roman(now.minute()),
        to_roman(now.second()),
    )
}

#[cfg(feature = "ssr")]
fn server_date() -> String {
    let now = chrono::Local::now();
    format!(
        "{} · {} · {}",
        to_roman(now.day()),
        to_roman(now.month()),
        to_roman(now.year() as u32),
    )
}

/// Live Roman-numeral clock, progressively enhanced.
///
/// Server-renders the current date as `day · month · year`.
/// After hydration, switches to the local time as `hour · minute · second`,
/// ticking every second.
#[component]
pub fn Clock() -> impl IntoView {
    let initial = {
        #[cfg(feature = "ssr")]
        {
            server_date()
        }
        #[cfg(not(feature = "ssr"))]
        {
            String::new()
        }
    };

    let text = RwSignal::new(initial);

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            text.set(format_time(chrono::Local::now()));
            leptos::leptos_dom::helpers::set_interval(
                move || text.set(format_time(chrono::Local::now())),
                std::time::Duration::from_secs(1),
            );
        }
    });

    view! { {move || text.get()} }
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
