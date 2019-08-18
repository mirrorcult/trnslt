#[macro_use]
extern crate clap;
extern crate curl;

#[cfg(test)]
mod tests;

use curl::easy::Easy;


fn main() {
    let matches = clap_app!(trnslt =>
        (version: "0.1.0")
        (author: "cyclowns <cyclowns@protonmail.ch>")
        (about: "CLI Google Translate app written in Rust")
        (@arg INLANG: +required +takes_value -i --inlang "Input language for your translation, i.e. en, es, cn. Full list on github.com or in help.")
        (@arg OUTLANG: +required +takes_value -o --outlang "Output language for your translation.")
        (@arg INPUT: +required "Input string to translate from INLANG to OUTLANG.")
    ).get_matches();

    println!("{:?}", matches);
}
