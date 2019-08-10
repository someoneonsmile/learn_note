module.exports = {
  extends: [
    "react-app", // for editor
    "eslint:recommended",
    "plugin:prettier/recommended",
    "prettier/react"
  ],
  parser: "babel-eslint",
  plugins: ["babel"],
  rules: {
    quotes: ["error", "single"],
    "no-debugger": process.env.NODE_ENV === "production" ? 2 : 0,
    "no-console": process.env.NODE_ENV === "production" ? 2 : 0,
    "babel/no-unused-expressions": "error",
    "no-unused-expressions": ["error", { allowShortCircuit: true }],
    "no-unused-vars": ["error"],
    "prettier/prettier": [
      "error",
      {
        singleQuote: true,
        semi: true
      }
    ]
  }
};
