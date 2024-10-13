use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use base64::prelude::*;
use crate::Base64Format;
use std::fs::File;
use std::io::Read;
use anyhow::Result;

pub fn process_encode(input:&str, format:Base64Format) -> Result<()>{

    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}",encoded);
    Ok(())
}

pub fn process_decode(input:&str, format:Base64Format)-> anyhow::Result<()>{

    let mut reader: Box<dyn Read> = if input == "-"{
        Box::new(std::io::stdin())
    }else {
        Box::new(File::open(input)?)
    };

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{}",decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>>{
    let reader:Box<dyn Read> = if input == "-"{
        Box::new(std::io::stdin())
    }else{
        Box::new(File::open(input)?)
    };
    Ok(reader)
}