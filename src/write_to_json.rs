use std::{fs::File, io::Write, time::SystemTime};

use serde::Serialize;

pub fn begin_write<T: Serialize>(data: &T, out: &str) -> anyhow::Result<()> {
    let now = SystemTime::now();
    let time_now = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let jsonfile_name = format!("{}-{}.json", out, time_now);

    let json_data = serde_json::to_string_pretty(data)?;
    let mut json_file = File::create(jsonfile_name)?;
    json_file.write_all(json_data.as_bytes())?;

    println!("save to json complete");
    Ok(())
}
