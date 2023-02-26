# Crate-Evolution
A zero-to-hero tutorial on refactoring your Rust project structure when necessary. Manage growing complexity incrementally with Rust's powerful tools!
(Inspired by [rust-split-example](https://github.com/robertorojasr/rust-split-example).)

Each step in the tutorial is a tag in this repo: "step1"," step2", etc.  You are challenged to make the next step on your own before moving on.

This evolution should look like this:

    0. Single executable file with everything on it
    1. Single executable file with a module inside
    2. Executable file plus a module file
    3a. Executable plus a module in a folder
    3b. Executable plus a folder module with multiple files as sub-modules
    4. Library crate with included executable
    5. Move the executable and library to different crates (workspaces) with their own directory tree

When our project gets bigger and more complex this strategies will help to keep the overall structure of your code manageable, clear and decoupled.
## Step 3: Executable plus a folder module with multiple files as sub-modules
_moved abc.rs into folder abc with mod.rs and a.rs, b.rs, c.rs_
```PowerShell
git checkout step3
```

## Step 2: Executable plus a module file
_moved module code into abc.rs_
```PowerShell
git checkout step2
```

## Step 1: Single executable with a module inside
_added some structure to main.rs_
```PowerShell
git checkout step1
```

## Step 0: Single executable file with everything on it
_all your code in main.rs_
```PowerShell
git checkout step0
```
Here's our starting point: too many things (code, ideas) crammed into main without enough structure.

**Do this**:  Refactor _main.rs_ by wrapping the structures in a module called "abc" -- but keep it all in _main.rs_ for now.
----------------
## Meta-Notes (REMOVE THESE)
* Match main.rs XML comments to this files step titles, include step #
* Rename crate to "rust-complexity-ladder"
* Add Cargo.lock to .gitignore in step1
* Change to 3,5,+/-/* --> "first one: 3, 5 = 8"
* step2-N: Don't name module "abc", just "abc"
* step3
```Rust
// these 3 lines could be combined into one:
// use crate::abc::{a::A, b::B, c::*};
use crate::abc::a::A;
use crate::abc::b::B;
// You can use wildcards, but explicit imports are better
use crate::abc::c::*;
```
* This tutorial may actually go too far (into workspaces)  
* Perhaps stop this tutorial with two crates -- library and binary that uses it.  (Last step would be adding a path-based Cargo.toml dependency and/or a git repo dependency with version info.)  
* Consider creating a series of [GitHub Skills](https://skills.github.com/quickstart) interactive courses. The second points back to the first, and the third through Nth points back to all prior lessons.  (But not future lessons?)  
Or always show the full list but say "You Are Here" on the appropriate lesson.  
* See also [Rust-by-Example](https://doc.rust-lang.org/rust-by-example/mod/split.html)  