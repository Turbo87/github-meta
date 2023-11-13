include!(concat!(env!("OUT_DIR"), "/types.rs"));

pub const META: Meta = include!(concat!(env!("OUT_DIR"), "/meta.rs"));

pub const SECRET_SCANNING: SecretScanning =
    include!(concat!(env!("OUT_DIR"), "/secret_scanning.rs"));
