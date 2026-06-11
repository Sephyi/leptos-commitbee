// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use leptos_meta::*;

pub const BASE_URL: &str = "https://commitbee.buzz";
const OG_IMAGE: &str = "https://commitbee.buzz/images/og-image.png";

/// Emits the full per-page head set: title, description, canonical,
/// Open Graph, and Twitter card tags. `path` must start with "/".
#[component]
pub fn SeoMeta(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] path: String,
    #[prop(optional)] article: bool,
) -> impl IntoView {
    let url = format!("{BASE_URL}{path}");
    let og_type = if article { "article" } else { "website" };
    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Link rel="canonical" href=url.clone()/>
        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:url" content=url/>
        <Meta property="og:site_name" content="CommitBee"/>
        <Meta property="og:type" content=og_type/>
        <Meta property="og:image" content=OG_IMAGE/>
        <Meta property="og:image:width" content="1200"/>
        <Meta property="og:image:height" content="630"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=OG_IMAGE/>
    }
}
