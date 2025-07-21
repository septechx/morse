use anyhow::{Result, bail};

use crate::lookup::{LATIN_LU, MORSE_LU};

pub fn morse(latin: &str) -> Result<String> {
    let mut vec: Vec<String> = Vec::new();

    for word in latin.split(" ") {
        let mut buf: Vec<&'static str> = Vec::new();

        for c in word.chars() {
            let c_str = c.to_lowercase().to_string();

            let index = LATIN_LU.iter().position(|&s| s == c_str);
            let index = match index {
                Some(idx) => idx,
                None => bail!("Character '{}' not found in lookup table", c),
            };

            let morse = MORSE_LU[index];
            buf.push(morse);
        }

        let s = buf.join(" ");
        vec.push(s);
    }

    let s = vec.join("/");

    Ok(s)
}

pub fn parse(morse: &str) -> Result<String> {
    let mut vec: Vec<String> = Vec::new();

    for word in morse.split("/") {
        let mut buf: Vec<&'static str> = Vec::new();

        for morse in word.split(" ") {
            let index = MORSE_LU.iter().position(|&s| s == morse);
            let index = match index {
                Some(idx) => idx,
                None => bail!("Morse '{}' not found in lookup table", morse),
            };

            let latin = LATIN_LU[index];
            buf.push(latin);
        }

        let s = buf.join("");
        vec.push(s);
    }

    let s = vec.join(" ");

    Ok(s)
}
