use core::fmt;

use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "contact_email.module.scss");

/// A new type for an email address.
pub struct EmailAddress(pub &'static str);

impl EmailAddress {
    pub fn display_view(&self) -> impl IntoView {
        if let Some((local, domain)) = self.0.split_once('@') {
            let (domain_name, tld) = domain.split_once('.').unwrap_or((domain, ""));
            view! {
                <span>
                    <span class=style::local>{local}</span>
                    <span class=style::at>"@"</span>
                    <span class=style::domain>{domain_name}</span>
                    <span class=style::tld>{format!(".{}", tld)}</span>
                </span>
            }
        } else {
            panic!("Invalid email address: {}", self.0);
        }
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        self.0
    }
}
