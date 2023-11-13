use serde::Deserialize;

/// The root object of <https://api.github.com/meta>.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub verifiable_password_authentication: bool,
    pub ssh_key_fingerprints: SshKeyFingerprints,
    pub ssh_keys: Vec<String>,
    pub hooks: Vec<String>,
    pub web: Vec<String>,
    pub api: Vec<String>,
    pub git: Vec<String>,
    pub github_enterprise_importer: Vec<String>,
    pub packages: Vec<String>,
    pub pages: Vec<String>,
    pub importer: Vec<String>,
    pub actions: Vec<String>,
    pub dependabot: Vec<String>,
    pub domains: Domains,
}

/// The `ssh_key_fingerprints` object of <https://api.github.com/meta>.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SshKeyFingerprints {
    pub sha256_ecdsa: String,
    pub sha256_ed25519: String,
    pub sha256_rsa: String,
}

/// The `domains` object of <https://api.github.com/meta>.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Domains {
    pub website: Vec<String>,
    pub codespaces: Vec<String>,
    pub copilot: Vec<String>,
    pub packages: Vec<String>,
}

/// The root object of <https://api.github.com/meta/public_keys/secret_scanning>.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretScanning {
    pub public_keys: Vec<PublicKey>,
}

/// The objects inside the `public_keys` list of
/// <https://api.github.com/meta/public_keys/secret_scanning>.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicKey {
    pub key_identifier: String,
    pub key: String,
    pub is_current: bool,
}
