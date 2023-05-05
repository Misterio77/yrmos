macro_rules! include_asset_file {
    ($path:expr) => {
        include_str!(concat!("../../assets/icons/", $path))
    }
}

macro_rules! include_out_file {
    ($path:expr) => {
        include_str!(concat!(env!("OUT_DIR"), $path))
    }
}

pub(crate) use include_asset_file;
pub(crate) use include_out_file;
