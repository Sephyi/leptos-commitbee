// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

/// Scroll-reveal wrapper. Renders children inside a div with the given CSS class.
/// The IntersectionObserver interactivity will be added as an island in Chunk 6.
#[component]
pub fn ScrollReveal(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let class = if class.is_empty() {
        "reveal".to_string()
    } else {
        class
    };

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
