fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/inbox_v1.proto")?;

    Ok(())
}
