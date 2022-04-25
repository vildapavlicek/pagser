fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .compile(&["protos/customer_service.proto"], &["protos/"])?;

    // export git hash as cargo env, requires git to be installed
    let output = std::process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()?;
    let git_hash = String::from_utf8(output.stdout)?;
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    Ok(())
}