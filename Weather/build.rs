use std::{path::PathBuf, env};

fn main() {
    let _res = tonic_build::configure()
    .build_server(true)
    .file_descriptor_set_path(
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"))
            .join("weather_descriptor.bin"),
    )
    .compile(
        &["proto/weather/weather.proto"], 
        &["proto/weather"],);
}
