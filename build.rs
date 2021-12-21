fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("data/protobuf-schemas/schema.proto")?;
    Ok(())
}