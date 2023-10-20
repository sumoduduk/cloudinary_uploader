mod convert_to_csv;
mod file_operation;
mod upload;
mod write_to_json;

use std::path::Path;

use anyhow::Ok;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{
    convert_to_csv::convert, file_operation::read_folder, upload::begin_upload,
    write_to_json::begin_write,
};

#[derive(Debug, Serialize)]
struct Record {
    id: u16,
    width: u32,
    height: u32,
    title: String,
    thumb_image: String,
    image: String,
    category: String,
    photo_type: String,
    key_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseJson {
    asset_id: String,
    width: u32,
    height: u32,
    url: String,
    original_filename: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about= None)]
struct Args {
    /// path where the folder located
    #[arg(short, long)]
    path: String,
    /// folder name in cloudinaty cloud
    #[arg(short, long)]
    folder_out: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let folder_path = args.path;
    println!(" folder path : {}", &folder_path);
    let folder_out = args.folder_out;
    println!(" folder out : {}", &folder_out);
    let folder_path = Path::new(&folder_path);

    let images = read_folder(folder_path).expect("no images on folder");

    let result = begin_upload(images, &folder_out)?;
    begin_write(&result, &folder_out)?;
    convert(&result, &folder_out)?;

    Ok(())
}
