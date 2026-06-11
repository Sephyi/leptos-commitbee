// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

use crate::content::loader;

#[component]
pub fn DocSidebar(#[prop(into)] current_slug: String) -> impl IntoView {
    let tree = loader::doc_tree();

    view! {
        <aside class="w-60 shrink-0 sticky top-16 h-[calc(100vh-4rem)] border-r border-honey/10 bg-surface/60 backdrop-blur-md overflow-y-auto sidebar-scroll">
            <nav aria-label="Documentation" class="px-4 py-6">
                {tree
                    .into_iter()
                    .map(|(section, pages)| {
                        view! {
                            <div class="mb-5">
                                <div class="mb-1.5 px-3 text-[0.6875rem] font-semibold uppercase tracking-widest text-comb">
                                    {section}
                                </div>
                                <ul class="space-y-0.5">
                                    {pages
                                        .into_iter()
                                        .map(|page| {
                                            let is_active = page.slug == current_slug;
                                            let link_class = if is_active {
                                                "block rounded-lg px-3 py-1.5 text-sm font-medium text-honey-text bg-honey/10"
                                            } else {
                                                "block rounded-lg px-3 py-1.5 text-sm text-comb hover:text-bark hover:bg-pollen/50 transition-colors"
                                            };
                                            view! {
                                                <li>
                                                    <a href=format!("/docs/{}", page.slug) class=link_class>
                                                        {page.title}
                                                    </a>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </div>
                        }
                    })
                    .collect_view()}
            </nav>
        </aside>
    }
}
