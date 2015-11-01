# rust-pg
Playing with rust and postgres. This is my first rust codebase and
everything I know starts with reading https://doc.rust-lang.org/stable/book. if you know rust or what a real application don't look here.

# Quick Start
```bash
brew install rust; #install rust stable
git clone git@github.com:amichal/rust-pg.git;
cd rust-pg;
cargo run # build any missing our out of date binaries to `target/debug/...` and then run the main package (listed in Cargo.toml)
# or
cargo run --release
```

# Notes for rails devs
* `src/main.rs` is the actuall rust code and has inline comments. After skimming it you should probaby go read the rust docs if you are still interested.
* `Cargo.toml` is like a `Gemfile` or `.gemspec`. See also `Cargo.lock`. https://crates.io/ ~= https://rubygems.org/
* `RustConfig` and `Procfile` support  `heroku create --buildpack https://github.com/emk/heroku-buildpack-rust.git` which is used to deploy to heroku. This is currently inefficent as hell as it uploads all the source to heroku and rebuilds all the dependancies from sctrach on each `git push heroku master`. THe buildpack could be enhanced to cache the build directory and then only copy the release binaries etc to the dynos... bit it works for now. Basically I followed https://github.com/emk/heroku-rust-cargo-hello

* This currently uses https://github.com/iron/iron as a web framework and https://github.com/sfackler/rust-postgres for talking to postgres. I want to try out https://github.com/nickel-org/nickel.rs as well and some of the ORMs and db abstraction layers
* Next up for exploration are HTML templating. https://github.com/jeremyletang/rust-haml exists but since my goal is to explore more efficent pre-compiled web stack runtimes I really want to try https://github.com/lfairy/maud which does the template compilation at runtime and doesn't need to have everything turned into a heap allocated HashMap
