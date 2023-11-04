# 11-04-2023

## Install

Well, that recommended installer, `rustup` is slow, I hope it works, operation timed out. Found 2 relative issues on Github, but not seeing a valid solution. I wish it's not something wrong with my network. Don't know why, moving on, trying `brew install rust` hope it works.

Hmm, brew takes a long time...

failed to download

Got stuck, will come back later.

Okay, looks like `brew install rust` worked out, it just happened to take a while,(30 - 40 min). I am happy now.

```
rustc --version
rustc 1.72.1
```

Although `brew` installation works, but it is not recommended, so the `rustup update` won't work, don't know if this gonna be an issue later on. Maybe I need to find a way to make `rustup` work, I will see.

## Hello world

Nice, got the `Hello world!` working.

## Hello cargo

Cargo is a tool for build and dep manager, it's like Bun.

## The ignore file

Cargo won't generate a .gitignore file if you are already on a git repo, I thought that was a mistake, and I looked at the issues, couldn't find any issue about it, nice I could open an issue. But then I read it again, it said "cargo initialized a new Git repository", thus nvm.
