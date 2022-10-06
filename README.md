<h1 align="center">wordsplit</h1>

**wordsplit** is a library that takes two arguments `haystack` & `delimiter` and splits the `haystack` with `delimiter`.

```rust
use wordsplit::WordSplit;

let haystack = "wordsplit is awesome";
let delimiter = " ";
let letters = WordSplit::new(haystack, delimiter);

assert!(letters.eq(vec!["wordsplit", "is", "awesome"].into_iter()));
```

## How to use

```toml
[dependencies]
wordsplit = { version = "0.1.0", url = git@github.com/royrustdev/wordsplit.git}
```
