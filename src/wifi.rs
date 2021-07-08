use crate::docker;
use crate::file;
use std::io::{Error, ErrorKind};

pub fn config(config_file: std::path::PathBuf, image_file: std::path::PathBuf ) -> Result<(),Error> {
    file::error_on_file_not_exists(&config_file)?;
    file::error_on_file_not_exists(&image_file)?;

    /*
        todo some content verification of config_file and image_file?
        e.g. image_file currently should be an uncompressed wic file
    */

    docker::set_wifi_config(config_file.to_str().unwrap(), image_file.to_str().unwrap())
}

pub fn info(image_file: std::path::PathBuf) -> Result<(),Error> {
    file::error_on_file_not_exists(&image_file)?;

    Err(Error::new(ErrorKind::Other, "Not implemented"))
}
