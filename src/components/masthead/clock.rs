#[cfg(feature = "ssr")]
use chrono::Datelike as _;
#[cfg(feature = "hydrate")]
use chrono::Timelike as _;
use leptos::prelude::*;

/// Convert an integer to a lowercase Roman numeral string.
///
/// # Arguments
///
/// * `n` - A value in the range `0..=3999`.
///
/// # Returns
///
/// * `"○"` (U+25CB WHITE CIRCLE) when `n` is `0`.
/// * A lowercase Roman numeral string for `1..=3999`.
///
/// # Panics
///
/// Panics if `n >= 4000`.
///
/// # Example
///
/// ```rust
/// assert_eq!(to_roman(0), "○");
/// assert_eq!(to_roman(4), "iv");
/// assert_eq!(to_roman(2026), "mmxxvi");
/// ```
#[cfg(any(feature = "ssr", feature = "hydrate", test))]
#[expect(
    clippy::arithmetic_side_effects,
    reason = "loop subtracts table values; value is always <= remaining by loop guard"
)]
fn to_roman(n: u32) -> String {
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
    if n == 0 {
        return "○".to_owned();
    }
    assert!(n < 4000, "to_roman: {n} is out of range (0–3999)");
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

#[cfg(feature = "hydrate")]
/// Formats the given local time as a Roman numeral string `hour · minute · second`.
///
/// Pads the seconds field to 7 characters with non-breaking spaces to prevent
/// layout shifts as the value changes every tick.
fn format_time(now: chrono::DateTime<chrono::Local>) -> String {
    let seconds = to_roman(now.second());
    // Pad seconds to max width (38 = "xxxviii" = 7 chars) with non-breaking spaces
    // to prevent layout shifts as the value changes every tick.
    let padding = 7_usize.saturating_sub(seconds.chars().count());
    format!(
        "{} · {} · {}{}",
        to_roman(now.hour()),
        to_roman(now.minute()),
        seconds,
        "\u{00A0}".repeat(padding),
    )
}

#[cfg(feature = "ssr")]
/// Formats today's date as a Roman numeral string for server-side rendering.
fn server_date() -> String {
    let now = chrono::Local::now();
    format!(
        "{} · {} · {}",
        to_roman(now.day()),
        to_roman(now.month()),
        to_roman(u32::try_from(now.year()).unwrap_or(0)),
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

    #[cfg(feature = "hydrate")]
    #[expect(
        clippy::expect_used,
        reason = "set_interval_with_handle always succeeds in a browser context"
    )]
    Effect::new(move |_| {
        text.set(format_time(chrono::Local::now()));
        let handle = leptos::leptos_dom::helpers::set_interval_with_handle(
            move || text.set(format_time(chrono::Local::now())),
            std::time::Duration::from_secs(1),
        )
        .expect("set_interval_with_handle should always succeed in a browser context");
        on_cleanup(move || handle.clear());
    });

    view! { {move || text.get()} }.into_any()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn zero_returns_circle() {
        assert_eq!(to_roman(0), "○");
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn additive_sequences() {
        assert_eq!(to_roman(1), "i");
        assert_eq!(to_roman(2), "ii");
        assert_eq!(to_roman(3), "iii");
        assert_eq!(to_roman(8), "viii");
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn subtractive_pairs() {
        assert_eq!(to_roman(4), "iv");
        assert_eq!(to_roman(9), "ix");
        assert_eq!(to_roman(40), "xl");
        assert_eq!(to_roman(90), "xc");
        assert_eq!(to_roman(400), "cd");
        assert_eq!(to_roman(900), "cm");
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn clock_boundary_values() {
        assert_eq!(to_roman(12), "xii");
        assert_eq!(to_roman(23), "xxiii"); // max 24h hour
        assert_eq!(to_roman(59), "lix"); // max minute / second
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn date_values() {
        assert_eq!(to_roman(31), "xxxi"); // max day
        assert_eq!(to_roman(2026), "mmxxvi"); // current year
        assert_eq!(to_roman(3999), "mmmcmxcix"); // upper bound
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[should_panic(expected = "out of range")]
    fn four_thousand_panics() {
        to_roman(4000);
    }
}
