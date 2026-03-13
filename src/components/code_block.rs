// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Island that activates copy-to-clipboard buttons on code blocks.
/// Rendered code blocks (from build.rs) contain `<button class="copy-btn" data-code="...">`.
/// This island finds them and attaches click handlers.
#[island]
pub fn CodeBlockActivator() -> impl IntoView {
    Effect::new(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let buttons = document.query_selector_all(".copy-btn").unwrap();

        for i in 0..buttons.length() {
            if let Some(btn) = buttons.item(i) {
                let btn_el: web_sys::HtmlElement = btn.unchecked_into();
                let btn_clone = btn_el.clone();

                let closure = Closure::<dyn Fn()>::new(move || {
                    if let Some(code) = btn_clone.dataset().get("code") {
                        let window = web_sys::window().unwrap();
                        let navigator = window.navigator();
                        let clipboard = navigator.clipboard();
                        let _ = clipboard.write_text(&code);

                        btn_clone.set_inner_html("Copied!");
                        let btn_reset = btn_clone.clone();
                        let timeout_closure = Closure::<dyn Fn()>::new(move || {
                            btn_reset.set_inner_html("Copy");
                        });
                        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout_closure.as_ref().unchecked_ref(),
                            2000,
                        );
                        timeout_closure.forget();
                    }
                });

                let _ = btn_el
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                closure.forget();
            }
        }
    });

    view! { <div class="hidden" data-code-block-activator="true"/> }
}
