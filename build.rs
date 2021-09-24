fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = rustc_version::version().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);

    tonic_build::compile_protos("proto/userservice.proto")?;
    tonic_build::compile_protos("proto/youtubeservice.proto")?;

    Ok(())
}