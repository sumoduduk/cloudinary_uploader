use anyhow::Ok;
use dotenvy::dotenv;
use rand::Rng;
use reqwest::blocking::{multipart, Client};
use std::{
    env,
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

use crate::ResponseJson;

pub fn upload_image(img_path: &Path, folder_out: String) -> anyhow::Result<ResponseJson> {
    dotenv().ok();

    let uri_endpoint = env::var("URI_ENDPOINT").expect("uri endpoint not found");
    let api_key = env::var("API_KEY").expect("api key not found");
    // let pub_id = env::var("PUB_ID").expect("api key not found");

    let files = multipart::Form::new()
        .file("file", img_path)?
        .text("api_key", api_key)
        .text("folder", folder_out)
        .text("upload_preset", "ml_default");

    // dbg!(&files);
    let client = Client::new();

    let response: ResponseJson = client.post(uri_endpoint).multipart(files).send()?.json()?;
    println!("upload success : {:?}", img_path);

    Ok(response)
}

pub fn begin_upload(images: Vec<PathBuf>, folder_out: &str) -> anyhow::Result<Vec<ResponseJson>> {
    let mut rng = rand::thread_rng();

    let mut arr_resp = Vec::new();

    for image in images {
        let folder_out = folder_out.to_string().clone();
        let wait_time = rng.gen_range(0.5..1.2);
        sleep(Duration::from_secs_f64(wait_time));

        let response = upload_image(&image, folder_out)?;

        arr_resp.push(response);
    }

    Ok(arr_resp)
}
