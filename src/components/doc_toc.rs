// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn DocToc(headings: &'static [(u8, &'static str, &'static str)]) -> impl IntoView {
    let toc_headings: Vec<_> = headings
        .iter()
        .filter(|(level, _, _)| *level == 2 || *level == 3)
        .collect();

    if toc_headings.is_empty() {
        return view! { <div/> }.into_any();
    }

    view! {
        <aside class="hidden w-64 pl-8 pr-4 xl:block shrink-0">
            <div class="sticky top-20">
                <div class="mb-3 text-xs font-semibold tracking-wider uppercase text-comb">"On this page"</div>
                <ul class="space-y-1 border-l border-honey/10">
                    {toc_headings
                        .into_iter()
                        .map(|(level, text, id)| {
                            let indent = if *level == 3 { "pl-6" } else { "pl-3" };
                            view! {
                                <li>
                                    <a
                                        href=format!("#{id}")
                                        class=format!("{indent} block py-1 text-xs text-comb hover:text-honey transition-colors")
                                    >
                                        {*text}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
        </aside>
    }
    .into_any()
}
