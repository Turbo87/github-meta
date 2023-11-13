include!(concat!(env!("OUT_DIR"), "/types.rs"));

/// The content of <https://api.github.com/meta> as a const [Meta] struct.
pub const META: Meta = include!(concat!(env!("OUT_DIR"), "/meta.rs"));

/// The content of <https://api.github.com/meta/public_keys/secret_scanning> as
/// a const [SecretScanning] struct.
pub const SECRET_SCANNING: SecretScanning =
    include!(concat!(env!("OUT_DIR"), "/secret_scanning.rs"));
