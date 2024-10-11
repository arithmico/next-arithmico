/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    extract: {
      rs: (content) => {
        const result = (content.match(/\"(.*)\"|class:(.*)=/g) ?? [])
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
  },
  theme: {
    extend: {},
  },
  plugins: [],
}

