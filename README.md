# insult - The Oh...Sir!-like insult generator

`insult` is the command line "utility" to insult you with a "Oh... Sir! The Insult Simulator"-like fashion!

Note: Windows is no longer supported.

## Example!

Did you know that `the Royal Family can't exercise because of your sister` or that `your wife probably murdered your husband`?  
Well, me neither.

## Configs!

You can edit this project's config files.  
They follow the XDG standard, so by default they're in `~/.config/insult`.

## Installing!

The easiest way to install is with cargo:  
```
cargo install insult
```

# Library!

Thanks to [#1](https://github.com/jD91mZM2/insult/issues/1), the core has now been split into a separate crate!  
This means you can generate your own insults from your own applications!

Example:

```Rust
let words = WordsFile {
    nouns: include_str!("nouns"),
    endings: include_str!("endings"),
    verbs: include_str!("verbs")
};
println!("{}", words.generate());
```
