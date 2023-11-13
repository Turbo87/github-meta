fn main() {
    // Find version field in Cargo.toml
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

    let re = regex::Regex::new(r#"version = "(.*)""#).expect("Failed to compile regex");
    let captures = re
        .captures(&cargo_toml)
        .expect("Failed to find version in Cargo.toml");

    let version = captures.get(1).unwrap().as_str();
    let version = semver::Version::parse(version).expect("Failed to parse version");

    // Increase minor version
    let new_version = semver::Version {
        minor: version.minor + 1,
        ..version
    };

    // Replace version in Cargo.toml
    let new_cargo_toml = re.replace(
        &cargo_toml,
        format!(r#"version = "{}""#, new_version).as_str(),
    );
    std::fs::write("Cargo.toml", new_cargo_toml.as_bytes()).expect("Failed to write Cargo.toml");
}
