#[macro_use]
extern crate clap;
extern crate curl;

use curl::easy::Easy;
use serde_json::Value;
use std::time::Instant;
use std::str;

#[cfg(test)]
mod tests;

const TRANSLATE_URL: &'static str = "http://translate.googleapis.com/translate_a/single?client=gtx&sl=";
const VALID_LANGS: &'static [&'static str] = &["auto", "af", "sq", "am", "ar", "az", "eu", "bn", "be", "bg", "ca", "zh-CN", "zh-TW", "hr", "cs", "da", "nl", "en", "eo", "et", "tl",
                                           "fi", "fr", "gl", "ka", "de", "el", "gu", "ht", "iw", "hi", "hu", "is", "id", "ga", "it", "ja", "kn", "ko", "la", "lv", "lt", "mk", "ms",
                                           "mt", "no", "fa", "pl", "pt", "ro", "ru", "sr", "sk", "sl", "es", "sw", "sv", "ta", "te", "th", "tr", "uk", "ur", "vi", "cy", "yi"];

fn main() {
    // timestamp stuff
    let start = Instant::now();

    let matches = clap_app!(trnslt =>
        (version: "0.1.0")
        (author: "cyclowns <cyclowns@protonmail.ch>")
        (about: "CLI Google Translate app written in Rust")
        (@arg INLANG: +takes_value -i --inlang "Input language for your translation, i.e. en, es, cn. Leave blank for auto-translation. Full list on github.com/cyclowns/trnslt or in README.md.")
        (@arg OUTLANG: +required +takes_value -o --outlang "Output language for your translation.")
        (@arg INPUT: +required "Input string to translate from INLANG to OUTLANG.")
    ).get_matches();

    let mut inlang = "auto";
    if matches.is_present("INLANG") {
        inlang = &matches.value_of("INLANG").unwrap();
    }

    let outlang = &matches.value_of("OUTLANG").unwrap();
    let input = matches.value_of("INPUT").unwrap();

    if !VALID_LANGS.contains(&outlang) || !VALID_LANGS.contains(&inlang) {
        panic!("Invalid language keys!\n Please use the valid languages listed in the README.md, or at github.com/cyclowns/trnslt");
    }

    println!("Output: {}\nTime elapsed: {:?}", translate(input, inlang, outlang), start.elapsed());
}

/// Translates the input string `input` from `inlang` to `outlang`.
fn translate(input: &str, inlang: &str, outlang: &str) -> String {
    // replace spaces with %20
    let input_url = input.replace(" ", "%20");
    let curl_str = &format!("{}{}&tl={}&dt=t&q={}", TRANSLATE_URL, inlang, outlang, input_url);

    let mut easy = Easy::new();
    easy.url(curl_str).unwrap();

    let mut data = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.push_str(str::from_utf8(&new_data).unwrap());
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    parse(data)
}

/// Parse json output given by Translate API query
fn parse(input: String) -> String {
    let json: Value = serde_json::from_str(&input).unwrap();

    let err = "Invalid translator response! Either you used an invalid language key, or something went wrong at Google.";
    let translation = json.get(0).expect(err).get(0).expect(err).get(0).expect(err); // yeah the json response we get is a mess
    translation.as_str().unwrap().to_string()
}
