use leptos::prelude::*;

use crate::content::loader;

#[component]
pub fn DocSidebar(#[prop(into)] current_slug: String) -> impl IntoView {
    let tree = loader::doc_tree();

    view! {
        <aside class="w-64 shrink-0 border-r border-honey/10 bg-surface-raised p-4 overflow-y-auto">
            <nav aria-label="Documentation">
                {tree
                    .into_iter()
                    .map(|(section, pages)| {
                        view! {
                            <div class="mb-6">
                                <h3 class="mb-2 text-xs font-semibold uppercase tracking-wider text-comb">
                                    {section}
                                </h3>
                                <ul class="space-y-1">
                                    {pages
                                        .into_iter()
                                        .map(|page| {
                                            let is_active = page.slug == current_slug;
                                            let link_class = if is_active {
                                                "block rounded-md px-3 py-1.5 text-sm font-medium bg-honey/10 text-honey border-l-2 border-honey"
                                            } else {
                                                "block rounded-md px-3 py-1.5 text-sm text-comb hover:text-bark hover:bg-surface transition-colors"
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
