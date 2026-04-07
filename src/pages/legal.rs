// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

//! Legal pages required by German/EU law:
//! - Impressum (TMG §5)
//! - Datenschutzerklärung (DSGVO / GDPR)
//!
//! NOTE: The placeholder fields marked `TODO` must be filled in with the
//! operator's real name, postal address, and contact details before deployment.
//! German law (TMG §5) requires a reachable postal address — a P.O. box or
//! "contact via email only" is NOT sufficient for natural persons operating
//! a publicly accessible website.

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Imprint() -> impl IntoView {
    view! {
        <Title text="Imprint - CommitBee"/>
        <Meta name="description" content="Legal notice and imprint for commitbee.buzz"/>

        <main class="max-w-3xl px-4 py-16 mx-auto sm:px-6 lg:px-8">
            <article class="prose max-w-none">
                <h1>"Imprint / Impressum"</h1>

                <p class="text-sm text-comb">
                    "Legal notice according to § 5 TMG (Telemediengesetz, German Telemedia Act)."
                </p>

                <h2>"Operator / Betreiber"</h2>
                <p>
                    "Sephyi"<br/>
                    // TODO: Add street and house number
                    "[Street and number]"<br/>
                    // TODO: Add postal code and city
                    "[Postal code] [City]"<br/>
                    "Germany"
                </p>

                <h2>"Contact / Kontakt"</h2>
                <p>
                    "Email: "<a href="mailto:me@sephy.io">"me@sephy.io"</a><br/>
                    "Web: "<a href="https://sephy.io" target="_blank" rel="noopener noreferrer">"sephy.io"</a>
                </p>

                <h2>"Responsible for content / Verantwortlich für den Inhalt"</h2>
                <p>
                    "Responsible for content according to § 18 Abs. 2 MStV:"<br/>
                    "Sephyi, address as above."
                </p>

                <h2>"Disclaimer / Haftungsausschluss"</h2>

                <h3>"Liability for content"</h3>
                <p>
                    "The content of this website has been created with the utmost care. However, "
                    "we cannot guarantee the content's accuracy, completeness, or topicality. "
                    "According to § 7(1) TMG, we are responsible for our own content on these pages "
                    "under general law. However, according to §§ 8 to 10 TMG, we are not obliged to "
                    "monitor transmitted or stored third-party information or to investigate "
                    "circumstances indicating illegal activity."
                </p>

                <h3>"Liability for links"</h3>
                <p>
                    "This website contains links to external third-party websites. We have no "
                    "influence on the content of those websites, and cannot accept any liability "
                    "for their content. The respective provider or operator of the linked pages "
                    "is always responsible for their content. Illegal content was not recognizable "
                    "at the time of linking. Should we become aware of any legal violations, we "
                    "will remove such links immediately."
                </p>

                <h3>"Copyright"</h3>
                <p>
                    "The content and works on these pages created by the site operator are subject "
                    "to the "<a href="https://polyformproject.org/licenses/noncommercial/1.0.0/" target="_blank" rel="noopener noreferrer">"PolyForm Noncommercial License 1.0.0"</a>". "
                    "Third-party content is marked as such."
                </p>

                <p class="mt-12 text-sm text-comb">
                    <a href="/" class="transition-colors hover:text-honey">"← Back to home"</a>
                </p>
            </article>
        </main>
    }
}

#[component]
pub fn Privacy() -> impl IntoView {
    view! {
        <Title text="Privacy Policy - CommitBee"/>
        <Meta name="description" content="Privacy policy and data protection notice for commitbee.buzz"/>

        <main class="max-w-3xl px-4 py-16 mx-auto sm:px-6 lg:px-8">
            <article class="prose max-w-none">
                <h1>"Privacy Policy / Datenschutzerklärung"</h1>

                <p class="text-sm text-comb">
                    "Information according to Art. 13 GDPR (DSGVO)."
                </p>

                <h2>"1. Controller / Verantwortlicher"</h2>
                <p>
                    "Controller within the meaning of the General Data Protection Regulation (GDPR) "
                    "and other national data protection laws of the EU member states is the operator "
                    "listed in the "<a href="/imprint">"Imprint"</a>"."
                </p>

                <h2>"2. Scope of data processing"</h2>
                <p>
                    "This website is a purely static documentation site. It does NOT:"
                </p>
                <ul>
                    <li>"set any cookies;"</li>
                    <li>"use any web analytics (no Google Analytics, Plausible, Fathom, Matomo, etc.);"</li>
                    <li>"use any tracking pixels, fingerprinting, or session recording;"</li>
                    <li>"embed any third-party scripts, iframes, or content delivery networks;"</li>
                    <li>"load fonts from third-party CDNs — all fonts are self-hosted;"</li>
                    <li>"process any form input (there are no forms);"</li>
                    <li>"maintain user accounts or profiles."</li>
                </ul>

                <h2>"3. Server log files (hosting provider)"</h2>
                <p>
                    "This website is hosted on "
                    <strong>"GitHub Pages"</strong>
                    " (GitHub, Inc., 88 Colin P Kelly Jr Street, San Francisco, CA 94107, USA), "
                    "operated in the EU by GitHub B.V., Vijzelstraat 68-72, 1017 HL Amsterdam, Netherlands."
                </p>
                <p>
                    "GitHub automatically collects and stores information in server log files that "
                    "your browser transmits when visiting the site. This includes:"
                </p>
                <ul>
                    <li>"browser type and version;"</li>
                    <li>"operating system used;"</li>
                    <li>"referrer URL;"</li>
                    <li>"hostname of the accessing computer;"</li>
                    <li>"time of the server request;"</li>
                    <li>"IP address."</li>
                </ul>
                <p>
                    "This data is not merged with other data sources by us. Collection is based on "
                    "Art. 6(1)(f) GDPR (legitimate interest in a technically error-free presentation "
                    "and optimization of the website). The operator has no access to these logs and "
                    "does not process them. See "
                    <a href="https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement" target="_blank" rel="noopener noreferrer">
                        "GitHub's Privacy Statement"
                    </a>
                    " for details."
                </p>

                <h2>"4. Local storage"</h2>
                <p>
                    "The site uses the browser's "<code>"localStorage"</code>" to remember your "
                    "theme preference (dark / light mode). This data never leaves your browser and "
                    "is not transmitted to any server. You can clear it at any time via your "
                    "browser settings."
                </p>

                <h2>"5. Your rights under GDPR"</h2>
                <p>
                    "Under the GDPR, you have the following rights regarding any personal data "
                    "concerning you that is processed:"
                </p>
                <ul>
                    <li>"right of access (Art. 15 GDPR);"</li>
                    <li>"right to rectification (Art. 16 GDPR);"</li>
                    <li>"right to erasure (Art. 17 GDPR);"</li>
                    <li>"right to restriction of processing (Art. 18 GDPR);"</li>
                    <li>"right to data portability (Art. 20 GDPR);"</li>
                    <li>"right to object (Art. 21 GDPR);"</li>
                    <li>"right to lodge a complaint with a supervisory authority (Art. 77 GDPR)."</li>
                </ul>
                <p>
                    "To exercise these rights, contact the operator listed in the "
                    <a href="/imprint">"Imprint"</a>
                    "."
                </p>

                <h2>"6. Changes to this policy"</h2>
                <p>
                    "We reserve the right to update this privacy policy to reflect changes in "
                    "legal requirements or the site itself. The current version is always "
                    "available on this page."
                </p>

                <p class="mt-12 text-sm text-comb">
                    "Last updated: 2026-04-07"
                </p>

                <p class="mt-4 text-sm text-comb">
                    <a href="/" class="transition-colors hover:text-honey">"← Back to home"</a>
                </p>
            </article>
        </main>
    }
}
