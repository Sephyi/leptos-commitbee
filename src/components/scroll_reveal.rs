use leptos::prelude::*;

#[island]
pub fn ScrollReveal(
    #[prop(default = "reveal".to_string())] class: String,
    children: Children,
) -> impl IntoView {
    let el_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move || {
        if let Some(el) = el_ref.get() {
            use wasm_bindgen::prelude::*;
            use web_sys::IntersectionObserverInit;

            let callback = Closure::<dyn Fn(js_sys::Array, web_sys::IntersectionObserver)>::new(
                move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
                    for entry in entries.iter() {
                        let entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
                        if entry.is_intersecting() {
                            let target = entry.target();
                            let _ = target.class_list().add_1("visible");
                            observer.unobserve(&target);
                        }
                    }
                },
            );

            let mut options = IntersectionObserverInit::new();
            options.threshold(&JsValue::from_f64(0.1));

            if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &options,
            ) {
                observer.observe(&el);
            }

            callback.forget();
        }
    });

    view! {
        <div node_ref=el_ref class=class>
            {children()}
        </div>
    }
}
