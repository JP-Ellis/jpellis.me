use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::Footer;
use crate::components::Masthead;

/// Interactive colour space visualiser page.
#[component]
pub fn ColorspaceVisualizerPage() -> impl IntoView {
    view! {
        <Title text="Colour Space Visualiser — Joshua Ellis" />
        <Masthead />
        <main>
            <div class="container">
                <h1>"Colour Space Visualiser"</h1>
                <p>
                    "An interactive exploration of eight colour spaces. "
                    "Rotate the 3D solid, drag the slice sliders, or pick two colours to see "
                    "how interpolation differs across spaces."
                </p>
            </div>
            <div id="colorspace-viz"></div>
        </main>
        <Footer />
        <script
            type="module"
            inner_html="import initWasm, { init } from '/tools/colorspace-viz/colorspace_viz.js'; await initWasm(); init();"
        />
    }
}
