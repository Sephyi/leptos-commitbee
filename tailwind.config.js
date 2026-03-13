// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        honey: {
          DEFAULT: "var(--honey)",
          light: "var(--honey-light)",
          dark: "var(--honey-dark)",
        },
        nectar: "var(--nectar)",
        comb: "var(--comb)",
        bark: "var(--bark)",
        pollen: "var(--pollen)",
        surface: {
          DEFAULT: "var(--surface)",
          raised: "var(--surface-raised)",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "ui-monospace", "monospace"],
      },
      maxWidth: {
        prose: "75ch",
      },
    },
  },
  plugins: [],
};
