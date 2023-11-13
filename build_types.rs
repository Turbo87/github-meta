use serde::Deserialize;

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

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SshKeyFingerprints {
    pub sha256_ecdsa: String,
    pub sha256_ed25519: String,
    pub sha256_rsa: String,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Domains {
    pub website: Vec<String>,
    pub codespaces: Vec<String>,
    pub copilot: Vec<String>,
    pub packages: Vec<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretScanning {
    pub public_keys: Vec<PublicKey>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicKey {
    pub key_identifier: String,
    pub key: String,
    pub is_current: bool,
}
