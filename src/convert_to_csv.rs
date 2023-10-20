use std::{fs::File, time::SystemTime};

use crate::{Record, ResponseJson};

pub fn convert(datas: &[ResponseJson], out: &str) -> anyhow::Result<()> {
    let now = SystemTime::now();
    let time_now = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let jsonfile_name = format!("{}-{}.csv", out, time_now);

    let csv_file = File::create(jsonfile_name).expect("cant create file name");
    let mut writer = csv::Writer::from_writer(csv_file);

    let mut i: u16 = 1;

    for data in datas {
        let (title, category, type_photo) = get_category_and_type(&data.original_filename);

        let record = Record {
            id: i,
            width: data.width,
            height: data.height,
            key_id: data.asset_id.clone(),
            title: title.to_string(),
            thumb_image: data.url.clone(),
            image: data.url.clone(),
            category: category.to_string(),
            photo_type: type_photo.to_string(),
        };

        println!("processing {}", title);
        writer.serialize(record)?;

        i += 1;
    }

    writer.flush()?;

    println!("convert to csv success");
    Ok(())
}

fn get_category_and_type(name: &str) -> (&str, &str, &str) {
    let name_split: Vec<&str> = name.split('-').collect();

    (name, name_split[0], name_split[1])
}
