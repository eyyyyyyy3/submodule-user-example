use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("./submodule-provider-example/proto/v1/example.proto")?;
    Ok(())
}
