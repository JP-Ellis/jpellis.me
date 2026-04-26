mod css_foundation;

pub use css_foundation::CssFoundationPage;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn TestLayout() -> impl IntoView {
    view! {
        <div>"TestLayout stub"</div>
        <Outlet />
    }
}

#[component]
pub fn TestIndex() -> impl IntoView {
    view! { <div>"TestIndex stub"</div> }
}
