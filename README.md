<img src="https://i.imgur.com/YoSd5SR.png" height=200>

![Build Status](https://travis-ci.org/cyclowns/trnslt.svg?branch=master) [![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

`trnslt` is a Rust CLI app that translates strings from one language to another using Google Translate.

## Installation / Usage

To install `trnslt` on Linux, you need `rustc` and `cargo` installed. Clone this repository, then run `cargo build --release`. An executable will appear at `(cloned_repo)/target/release/trnslt`. You can move this to `/usr/bin` or wherever else is covered by your PATH.

On Windows, do the same steps but move the executable to somewhere covered by your PATH, or move it to some location and add that location to your PATH.

Actual binary distributions coming soon(?)

```
trnslt 1.0.2 
cyclowns <cyclowns@protonmail.ch>
CLI Google Translate app written in Rust

USAGE:
    trnslt [OPTIONS] <INPUT> --outlang <OUTLANG>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --inlang <INLANG>      Input language for your translation, i.e. en, es, cn. Leave blank for auto-translation.
                               Full list on github.com/cyclowns/trnslt or in README.md.
    -o, --outlang <OUTLANG>    Output language for your translation.

ARGS:
    <INPUT>    Input string to translate from INLANG to OUTLANG.
```

## Example

```bash
trnslt -i tr -o is "Olmak ya da olmamak" # To be or not to be--from turkish to icelandic
    > "Að vera eða ekki vera"
```

## All Language Keys

Here is a list of all valid language keys. If the input language is unknown, simply leave it blank and `trnslt` will auto-detect what language it is. I haven't guaranteed that all of these work, but if one doesn't, either make a PR to remove it or msg me on Discord at `cyclowns#1440` and I'll remove it and add a disclaimer.

Disclaimer: Some non-ascii languages are kinda iffy. Arabic in particular, since it's got some weird writing shenanigans going on that I don't understand.

```
af  Afrikaans
sq  Albanian
am  Amharic
ar  Arabic
az  Azerbaijani
eu  Basque
bn  Bengali
be  Belarusian
bg  Bulgarian
ca  Catalan
zh-CN Chinese Simplified
zh-TW Chinese Traditional
hr  Croatian
cs  Czech
da  Danish
nl  Dutch
en  English
eo  Esperanto
et  Estonian
tl  Filipino
fi  Finnish
fr  French
gl  Galician
ka  Georgian
de  German
el  Greek
gu  Gujarati
ht  Haitian Creole
iw  Hebrew
hi  Hindi
hu  Hungarian
is  Icelandic
id  Indonesian
ga  Irish
it  Italian
ja  Japanese
kn  Kannada
ko  Korean
la  Latin
lv  Latvian
lt  Lithuanian
mk  Macedonian
ms  Malay
mt  Maltese
no  Norwegian
fa  Persian
pl  Polish
pt  Portuguese
ro  Romanian
ru  Russian
sr  Serbian
sk  Slovak
sl  Slovenian
es  Spanish
sw  Swahili
sv  Swedish
ta  Tamil
te  Telugu
th  Thai
tr  Turkish
uk  Ukrainian
ur  Urdu
vi  Vietnamese
cy  Welsh
yi  Yiddish
```
