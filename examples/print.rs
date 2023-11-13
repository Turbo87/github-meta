fn main() {
    let meta = github_meta::META;
    println!("{meta:#?}");
    let secret_scanning = github_meta::SECRET_SCANNING;
    println!("{secret_scanning:#?}");
}
