use crate::docker;
use crate::file;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub fn set_iotedge_gateway_config(config_file: PathBuf, image_file: PathBuf, root_ca_file: PathBuf, device_identity_file: PathBuf, device_identity_key_file: PathBuf ) -> Result<(),Error> {
    file::error_on_file_not_exists(&config_file)?;
    file::error_on_file_not_exists(&image_file)?;
    file::error_on_file_not_exists(&root_ca_file)?;
    file::error_on_file_not_exists(&device_identity_file)?;
    file::error_on_file_not_exists(&device_identity_key_file)?;

    /*
        todo some content verification of config_file and image_file?
        e.g. image_file currently should be an uncompressed wic file
    */

    docker::set_iotedge_gateway_config(config_file.to_str().unwrap(),
                               image_file.to_str().unwrap(),
                               root_ca_file.to_str().unwrap(),
                               device_identity_file.to_str().unwrap(),
                               device_identity_key_file.to_str().unwrap())?;

    Ok (())
}

pub fn set_iotedge_sas_leaf_config(config_file: PathBuf, image_file: PathBuf, root_ca_file: PathBuf) -> Result<(),Error> {
    file::error_on_file_not_exists(&config_file)?;
    file::error_on_file_not_exists(&image_file)?;
    file::error_on_file_not_exists(&root_ca_file)?;

    docker::set_iotedge_sas_leaf_config(config_file.to_str().unwrap(), image_file.to_str().unwrap(), root_ca_file.to_str().unwrap())
}

pub fn info(image_file: std::path::PathBuf) -> Result<(),Error> {
    file::error_on_file_not_exists(&image_file)?;

    Err(Error::new(ErrorKind::Other, "Not implemented"))
}
