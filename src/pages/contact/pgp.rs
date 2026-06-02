use core::fmt;

/// A newtype for the PGP key fingerprint, which provides formatting and a URL.
pub struct PgpKey(pub &'static str);

impl PgpKey {
    /// Returns the URL to the key on keys.openpgp.org.
    pub fn url(&self) -> String {
        format!("https://keys.openpgp.org/vks/v1/by-fingerprint/{}", self.0)
    }
}

/// Formats the PGP key fingerprint with spaces every 4 characters for
/// readability.
impl fmt::Display for PgpKey {
    #[expect(
        clippy::integer_division_remainder_used,
        reason = "modulo-4 check for spacing is intentional"
    )]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let spaced = self
            .0
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 4 == 0 && i != 0 {
                    format!(" {c}")
                } else {
                    c.to_string()
                }
            })
            .collect::<String>();
        write!(f, "{spaced}")
    }
}
