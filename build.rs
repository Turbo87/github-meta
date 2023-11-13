use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use types::*;

mod types {
    include!("build_types.rs");
}

fn main() -> Result<(), Box<dyn Error>> {
    generate_types_module()?;
    generate_meta_rs()?;
    generate_secret_scanning_rs()?;
    Ok(())
}

fn generate_types_module() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build_types.rs");

    let path = Path::new(&std::env::var("OUT_DIR")?).join("types.rs");
    let mut file = BufWriter::new(File::create(path)?);

    let build_types = include_str!("build_types.rs");
    let types = build_types
        .replace("use serde::Deserialize;\n", "")
        .replace("#[derive(", "#[derive(Debug, ")
        .replace(", Deserialize", "")
        .replace("#[serde(deny_unknown_fields)]\n", "")
        .replace("#[serde(rename_all = \"SCREAMING_SNAKE_CASE\")]\n", "")
        .replace("String", "&'static str")
        .replace("Vec<", "&'static [")
        .replace(">,", "],");

    file.write_all(types.as_bytes())?;

    Ok(())
}

fn generate_meta_rs() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=data/");

    let path = Path::new(&std::env::var("OUT_DIR")?).join("meta.rs");
    let mut file = BufWriter::new(File::create(path)?);

    let meta = include_bytes!("data/meta.json");
    let meta: Meta = serde_json::from_slice(meta)?;
    writeln!(file, "{meta:?}")?;

    Ok(())
}

fn generate_secret_scanning_rs() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=data/");

    let path = Path::new(&std::env::var("OUT_DIR")?).join("secret_scanning.rs");
    let mut file = BufWriter::new(File::create(path)?);

    let secret_scanning = include_bytes!("data/secret-scanning.json");
    let secret_scanning: SecretScanning = serde_json::from_slice(secret_scanning)?;
    writeln!(file, "{secret_scanning:?}")?;

    Ok(())
}

struct Array<'a, T> {
    array: &'a Vec<T>,
    indent: usize,
}

impl<'a, T> Array<'a, T> {
    fn new(array: &'a Vec<T>, indent: usize) -> Self {
        Self { array, indent }
    }
}

impl<T: Debug> Debug for Array<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "&[")?;
        for value in self.array {
            write!(f, "{}", "    ".repeat(self.indent + 1))?;
            writeln!(f, "{value:?},")?;
        }
        write!(f, "{}]", "    ".repeat(self.indent))
    }
}

impl Debug for Meta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Meta {{")?;
        writeln!(
            f,
            "    verifiable_password_authentication: {:?},",
            self.verifiable_password_authentication
        )?;
        writeln!(
            f,
            "    ssh_key_fingerprints: {:?},",
            self.ssh_key_fingerprints
        )?;
        writeln!(f, "    ssh_keys: {:?},", Array::new(&self.ssh_keys, 1))?;
        writeln!(f, "    hooks: {:?},", Array::new(&self.hooks, 1))?;
        writeln!(f, "    web: {:?},", Array::new(&self.web, 1))?;
        writeln!(f, "    api: {:?},", Array::new(&self.api, 1))?;
        writeln!(f, "    git: {:?},", Array::new(&self.git, 1))?;
        writeln!(
            f,
            "    github_enterprise_importer: {:?},",
            Array::new(&self.github_enterprise_importer, 1)
        )?;
        writeln!(f, "    packages: {:?},", Array::new(&self.packages, 1))?;
        writeln!(f, "    pages: {:?},", Array::new(&self.pages, 1))?;
        writeln!(f, "    importer: {:?},", Array::new(&self.importer, 1))?;
        writeln!(f, "    actions: {:?},", Array::new(&self.actions, 1))?;
        writeln!(f, "    dependabot: {:?},", Array::new(&self.dependabot, 1))?;
        writeln!(f, "    domains: {:?},", self.domains)?;
        write!(f, "}}")
    }
}

impl Debug for SshKeyFingerprints {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SshKeyFingerprints {{")?;
        writeln!(f, "        sha256_ecdsa: \"{}\",", self.sha256_ecdsa)?;
        writeln!(f, "        sha256_ed25519: \"{}\",", self.sha256_ed25519)?;
        writeln!(f, "        sha256_rsa: \"{}\",", self.sha256_rsa)?;
        write!(f, "    }}")
    }
}

impl Debug for Domains {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Domains {{")?;
        writeln!(f, "        website: {:?},", Array::new(&self.website, 2))?;
        writeln!(
            f,
            "        codespaces: {:?},",
            Array::new(&self.codespaces, 2)
        )?;
        writeln!(f, "        copilot: {:?},", Array::new(&self.copilot, 2))?;
        writeln!(f, "        packages: {:?},", Array::new(&self.packages, 2))?;
        write!(f, "    }}")
    }
}

impl Debug for SecretScanning {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SecretScanning {{")?;
        writeln!(
            f,
            "    public_keys: {:?},",
            Array::new(&self.public_keys, 1)
        )?;
        write!(f, "}}")
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PublicKey {{")?;
        writeln!(f, "        key_identifier: \"{}\",", self.key_identifier)?;
        writeln!(f, "        key: \"{}\",", self.key)?;
        writeln!(f, "        is_current: {:?},", self.is_current)?;
        write!(f, "    }}")
    }
}
