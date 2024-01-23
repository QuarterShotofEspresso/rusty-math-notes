# Rusty (Math) Notes
> Author: Ratnodeep Bandyopadhyay ([@QuarterShotofEspresso](https://github.com/QuarterShotofEspresso))

## What's this about?
A starter project for gaining expereince with Rust + Leptos. Rusty notes is a collection of notes from various math classes over the years--all in one location where I'm hoping to a have a prim & proper reference from anywhere I could possibly wish to have them. Displayed like a long latex document, but all written in Rust.

Notes are accessible [here](https://quartershotofespresso.github.io/rusty-math-notes/).

## Dependencies
Naturally, this project was written in 100% Rust <333. Listed are some of the crates used for this project:
- [Leptos](https://leptos.dev)
- [latex2mathml](https://github.com/QuarterShotofEspresso/latex2mathml) ([supported syntax](https://github.com/QuarterShotofEspresso/latex2mathml/blob/master/src/token.rs))
> Original [latex2mathml](https://crates.io/crates/latex2mathml) ([supported syntax](https://github.com/osanshouo/latex2mathml/blob/master/src/token.rs)). *There above repo-link adds some commands (most notably vskip) to the original latex2mathml crate. It is the crate currently being used for this project.*
- [Latin Modern Math](https://www.cdnfonts.com/latin-modern-math.font) (font)

## Todo
- ~~Font sucks.~~ Fixed: Latin Modern Math.
- ~~Development proc could be further improved.~~ Fixed. Boilerplate's been boiled away.
    - ~~There's still too much to continue writing this in the current syntax. These vertical splits would greatly benefit if they could be added to the latex2mathml crate itself.~~ Fixed. latex2mathml crate was forked and the `vksip` command was added to the transpiler see [here](https://github.com/QuarterShotofEspresso/latex2mathml). Should be configurable by any unit e.g. `\vskip 0.1em` etc. Although an annoying `$$...$$` is still needed.
    - Need to remove the `$$` wrapper around formatting elements e.g. \textit, \vskip, \mathbb...
- More notes need to be added.
- Host on github pages (after 171 content is up to date).
- Pages need to be added for scaling notes + classes.
    - Idea: Update the... forgot the idea.
- Project structure needs to be cleaned up a little.
- For the home page add a square animation that spans the width of the `journal` and add topic selections below that.