# Yes/No Stable

A web app that answers your yes/no questions with a gif.

Inspired by to the excellent [Yes/No](https://yesno.wtf/) web app, but with one key difference. This app remembers previously generated answers, so you can share the URL with friends and everyone will see the same answer. New answers are still randomly generated of course.

Written in [Rust](https://www.rust-lang.org/) using the [Rocket](https://rocket.rs/) web framework.

## Building

* Get yourself a [Giphy](https://developers.giphy.com/) API key and put it in `src/giphy-api-key.txt`
* Run `cargo build` / `cargo run` / etc like a normal Rust project

## TODO

* [ ] Replace global dictionary with something that:
    * [ ] Doesn't loose all memory when the app is restarted
    * [ ] Has a limit on the number of entries / purges old entries
* [ ] Index page
* [ ] HTML niceties:
    * [ ] Add a favicon
    * [ ] Animate the answer
