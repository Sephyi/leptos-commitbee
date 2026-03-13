use leptos::prelude::*;

const STEPS: &[PipelineStep] = &[
    PipelineStep {
        name: "Git Service",
        icon: "📂",
        description: "Reading staged changes...",
        detail: "3 files staged: auth.rs, config.rs, tests/auth_test.rs",
    },
    PipelineStep {
        name: "Tree-sitter",
        icon: "🌳",
        description: "Parsing symbols...",
        detail: "Found: fn validate() [modified-signature], fn refresh_token() [added], struct AuthConfig [modified]",
    },
    PipelineStep {
        name: "Splitter",
        icon: "🔀",
        description: "Analyzing change groups...",
        detail: "1 logical group detected — all files share auth symbol dependencies",
    },
    PipelineStep {
        name: "Context Builder",
        icon: "📋",
        description: "Building evidence...",
        detail: "EVIDENCE: bug_fix=true, breaking_change=true (return type changed)\nBUDGET: 2,847 / 6,000 tokens",
    },
    PipelineStep {
        name: "LLM",
        icon: "🧠",
        description: "Generating commit message...",
        detail: "fix(auth)!: return Result from validate with expiry check",
    },
    PipelineStep {
        name: "Validator",
        icon: "✅",
        description: "Checking 7 rules...",
        detail: "✓ fix-evidence  ✓ breaking-change  ✓ anti-hallucination\n✓ mechanical  ✓ dependency  ✓ specificity  ✓ length (52/72)",
    },
    PipelineStep {
        name: "Sanitizer",
        icon: "🧹",
        description: "Cleaning output...",
        detail: "fix(auth)!: return Result from validate with expiry check\n\nBREAKING CHANGE: validate() now returns Result<(), AuthError>\ninstead of bool. Callers must handle the error case.\n\nAdd token expiry checking to the validation pipeline. The\nprevious implementation silently ignored expired tokens.",
    },
];

struct PipelineStep {
    name: &'static str,
    icon: &'static str,
    description: &'static str,
    detail: &'static str,
}

#[island]
pub fn PipelineDemo() -> impl IntoView {
    let (current_step, set_current_step) = signal(0usize);
    let (is_playing, set_is_playing) = signal(false);

    // Auto-advance when playing
    Effect::new(move || {
        if is_playing.get() {
            let step = current_step.get();
            if step < STEPS.len() - 1 {
                let handle = set_timeout_with_handle(
                    move || {
                        set_current_step.update(|s| *s += 1);
                    },
                    std::time::Duration::from_millis(2000),
                );
                on_cleanup(move || {
                    if let Ok(h) = handle {
                        h.clear();
                    }
                });
            } else {
                set_is_playing.set(false);
            }
        }
    });

    view! {
        <div class="rounded-xl border border-honey/20 bg-surface-raised overflow-hidden">
            // Controls
            <div class="flex items-center gap-3 px-6 py-4 border-b border-honey/10">
                <button
                    on:click=move |_| {
                        if is_playing.get() {
                            set_is_playing.set(false);
                        } else {
                            if current_step.get() >= STEPS.len() - 1 {
                                set_current_step.set(0);
                            }
                            set_is_playing.set(true);
                        }
                    }
                    class="rounded-lg bg-honey px-4 py-2 text-sm font-medium text-white hover:bg-honey-dark transition-colors"
                >
                    {move || if is_playing.get() { "Pause" } else { "Play" }}
                </button>
                <button
                    on:click=move |_| {
                        set_is_playing.set(false);
                        set_current_step.update(|s| if *s > 0 { *s -= 1 });
                    }
                    class="rounded-lg border border-honey/20 px-3 py-2 text-sm text-comb hover:text-bark transition-colors"
                    disabled=move || current_step.get() == 0
                >
                    "← Prev"
                </button>
                <button
                    on:click=move |_| {
                        set_is_playing.set(false);
                        set_current_step.update(|s| if *s < STEPS.len() - 1 { *s += 1 });
                    }
                    class="rounded-lg border border-honey/20 px-3 py-2 text-sm text-comb hover:text-bark transition-colors"
                    disabled=move || current_step.get() >= STEPS.len() - 1
                >
                    "Next →"
                </button>
                <span class="ml-auto text-sm text-comb">
                    {move || format!("Step {} of {}", current_step.get() + 1, STEPS.len())}
                </span>
            </div>

            // Step progress bar
            <div class="flex gap-1 px-6 py-3 bg-pollen/30">
                {STEPS.iter().enumerate().map(|(i, step)| {
                    view! {
                        <div class="flex-1">
                            <div class=move || {
                                let active = i <= current_step.get();
                                let current = i == current_step.get();
                                format!(
                                    "h-1 rounded-full transition-all duration-500 {}",
                                    if current { "bg-honey" }
                                    else if active { "bg-honey/40" }
                                    else { "bg-honey/10" }
                                )
                            }/>
                            <div class="mt-1 text-center">
                                <span class=move || {
                                    let active = i <= current_step.get();
                                    format!(
                                        "text-[10px] font-medium {}",
                                        if active { "text-honey" } else { "text-comb/50" }
                                    )
                                }>
                                    {step.name}
                                </span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Current step detail
            <div class="p-6">
                {move || {
                    let step = &STEPS[current_step.get()];
                    view! {
                        <div class="pipeline-step active">
                            <div class="flex items-center gap-3 mb-4">
                                <span class="text-3xl">{step.icon}</span>
                                <div>
                                    <h3 class="text-lg font-semibold text-bark">{step.name}</h3>
                                    <p class="text-sm text-comb">{step.description}</p>
                                </div>
                            </div>
                            <pre class="rounded-lg bg-pollen p-4 font-mono text-sm text-bark whitespace-pre-wrap">
                                {step.detail}
                            </pre>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
