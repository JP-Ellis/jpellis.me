//! UI component library for this site.
#![expect(
    clippy::pub_use,
    reason = "module facade: re-exporting component types for ergonomic imports"
)]

/// Full-bleed contrast band component.
pub mod band;
/// Site-wide footer component.
pub mod footer;
/// Site-wide sticky header component.
pub mod masthead;
/// Year-in-code statistics component.
pub mod year_in_code;

pub use band::Band;
pub use footer::Footer;
pub use masthead::Masthead;
pub use year_in_code::YearInCode;
