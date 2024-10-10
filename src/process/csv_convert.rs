use std::fs;
use csv::Reader;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use serde_json::Value;
use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
struct Film {
    #[serde(rename(deserialize = "0", serialize = "排名"))]
    rank: u16,
    #[serde(rename = "名称")]
    name: String,
    #[serde(rename = "年份")]
    year: String,
    #[serde(rename = "国家")]
    nation: String,
    #[serde(rename = "类型")]
    genre: String,
    #[serde(rename = "导演")]
    director: String,
    #[serde(rename = "评价人数")]
    rating_count: u32,
    #[serde(rename = "评分")]
    rating: f32,
}

pub fn csv_process(input: &str, output: String,format: OutputFormat) -> Result<()>{
    let mut reader = Reader::from_path(input)?;
    let mut rs: Vec<Value> = Vec::with_capacity(250);
    // for result in reader.deserialize() {
    //     let record: Film = result?;
    //     rs.push(record);
    //     // println!("{:?}", record);
    // }
    let headers = reader.headers()?.clone();
    for result in reader.records(){
        let record = result?;
        let json_value:Value = headers.iter().zip(record.iter()).collect::<Value>();
        rs.push(json_value);
    }

    let content =  match format{
        OutputFormat::Json => serde_json::to_string_pretty(&rs)?,
        OutputFormat::Yaml => serde_yaml::to_string(&rs)?,
    };

    fs::write(output, content)?;

    Ok(())
}