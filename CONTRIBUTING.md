# Contributing to PBP

The following are a few ways you can contribute to the project, along with a guideline to follow when writing code or making a commit.

## **With code**

---

Contributing to this project is easy and appreciated.

You will need [git](https://git-scm.com) for contributing.

1. [Fork the repo](https://github.com/ProjectBlackPearl/PBPL/fork)
2. Create a branch `git checkout -b branch-name`
3. Commit them `git commit -m "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce rhoncus."`
4. Push them to Github `git push -u origin branchname`
5. Open a PR

## **Style guidelines**

---

Most of these here are taken from the Atom guidelines <sup>(they're really good)</sup>

### **Git commit messages**

-   Use the present tense ("Add feature" not "Added feature")
-   Use the imperative mood ("Move cursor to..."not "Moves cursor to")
-   Limit the first line to 72 characters or less
-   Describe the additions on the next line
-   When changing documentation, prefix `[ci skip]` on the commit message

### **Typescript guideline**

All of our code is styled with [Prettier](https://prettier.io).

-   Prefer using the spread syntax `{...someObj}` instead `Object.assign()`
-   Use different cases:
    -   camelCase for constants, variables and functions
    -   PascalCase for classes
-   Inline exports when possible

```ts
// Use this:
export const functionName = (): void => {
    // ...
}

// Not this:
const functionName = (): void => {
    // ...
}
export functionName;
```

-   Use arrow functions when possible

```ts
// Use this:
const functionName = (): void => {
	// ...
};

// Not this:
function functionName(): void {
	// ...
}
```

### **Rust Guidelines**

-   Use different cases:
    -   `snake_case` for functions and variables
    -   `PascalCase` for structs
    -   `SCREAMING_SNAKE_CASE` for constants

### **Documentation guideline**

-   Use [JSDoc](https://jsdoc.app)
-   Use [Markdown](https://www.markdownguide.org/)
-   Reference types in documentation using `{}`
-   When making a function use this
    -   If it invokes a Rust function use `Typescript Function -> Rust Function`
    -   If it's only a Typescript function use `Typescript Function`

Example:

```ts
/*
 * Typescript Function
 * - Adds 2 + 2
 *
 * @returns {number} the number added
 */
const addNum = (): number => 2 + 2;
```
