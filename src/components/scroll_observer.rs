// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Single island that observes all `.reveal` / `.reveal-left` / `.reveal-right` /
/// `.reveal-scale` elements and adds `.visible` when they enter the viewport.
/// Place once in the app shell — it covers the entire page.
#[island]
pub fn ScrollObserver() -> impl IntoView {
    Effect::new(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let callback = Closure::<dyn Fn(js_sys::Array, web_sys::IntersectionObserver)>::new(
            move |entries: js_sys::Array, _observer: web_sys::IntersectionObserver| {
                for entry in entries.iter() {
                    let entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
                    if entry.is_intersecting() {
                        let target = entry.target();
                        let _ = target.class_list().add_1("visible");
                    }
                }
            },
        );

        let options = web_sys::IntersectionObserverInit::new();
        options.set_threshold(&JsValue::from_f64(0.1));

        let observer = web_sys::IntersectionObserver::new_with_options(
            callback.as_ref().unchecked_ref(),
            &options,
        )
        .unwrap();

        let selectors = ".reveal, .reveal-left, .reveal-right, .reveal-scale";
        let elements = document.query_selector_all(selectors).unwrap();
        for i in 0..elements.length() {
            if let Some(el) = elements.get(i) {
                observer.observe(el.unchecked_ref());
            }
        }

        callback.forget();
    });

    view! {}
}
