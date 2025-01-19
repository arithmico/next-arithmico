/** @type {import('tailwindcss').Config} */

import plugin from 'tailwindcss/plugin';

const themes = ["light", "dark"];

export const content = {
  files: [
    "*.html",
    "./src/**/*.rs",
    "../../libraries/editor/src/**/*.rs",
    "../../libraries/translate/src/**/*.rs",
    "../../libraries/ui/src/**/*.rs"
  ],
  extract: {
    rs: (content) => {
      const result = (content.match(/"(?:[^"\\]|\\.)*"|class:(.*)=/g) ?? [])
        .flatMap(candidate => candidate
          .replaceAll("\"", "")
          .replace("class:", "")
          .split(" ")
          .map(classCandidate => classCandidate.replace(/=$/g, ""))
        )
        .filter(candidate => candidate != "");
      return result;
    }
  }
};
export const theme = {
  extend: {
    colors: {
      "neutral-850": "rgb(32, 32, 32)",
    },
  },
};
export const plugins = [
  plugin(function ({ addVariant }) {
    themes.forEach((theme) => {
      addVariant(`theme-${theme}`, `.theme-${theme} &`);
    });
  }),
];

