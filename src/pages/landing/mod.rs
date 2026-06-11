// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

mod comparison;
mod differentiators;
mod docs_transition;
mod hero;
mod install;
mod pipeline;
mod problem;

use leptos::prelude::*;

use comparison::ComparisonSection;
use differentiators::DifferentiatorsSection;
use docs_transition::DocsTransitionSection;
use hero::HeroSection;
use install::InstallSection;
use pipeline::PipelineSection;
use problem::ProblemSection;

const LANDING_JSON_LD: &str = r#"{"@context":"https://schema.org","@graph":[{"@type":"WebSite","name":"CommitBee","url":"https://commitbee.buzz/"},{"@type":"SoftwareApplication","name":"CommitBee","applicationCategory":"DeveloperApplication","operatingSystem":"macOS, Linux, Windows","description":"CommitBee uses tree-sitter semantic analysis and LLMs to generate high-quality conventional commit messages.","url":"https://commitbee.buzz/","downloadUrl":"https://crates.io/crates/commitbee","offers":{"@type":"Offer","price":"0","priceCurrency":"USD"},"author":{"@type":"Person","name":"Sephyi","url":"https://sephy.io"}}]}"#;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <crate::components::seo::SeoMeta
            title="CommitBee – Semantic AI Commit Message Generator"
            description="CommitBee is a Rust CLI that generates Conventional Commit messages from your staged changes using tree-sitter semantic analysis and local or cloud LLMs."
            path="/"
        />
        <script type="application/ld+json" inner_html=LANDING_JSON_LD></script>

        <main id="main-content">
            <HeroSection/>
            <ProblemSection/>
            <PipelineSection/>
            // TODO: <DifferentiatorsSection/>
            // TODO: <ComparisonSection/>
            // TODO: <InstallSection/>
            <DocsTransitionSection/>
            <crate::components::code_block::CodeBlockActivator/>
        </main>
    }
}
