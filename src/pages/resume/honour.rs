use serde::Deserialize;

/// An honour or award entry on the CV.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Honour {
    /// Year the award was received.
    pub year: u32,
    /// Name of the award.
    pub award: String,
    /// Organisation that granted the award.
    pub org: String,
}
