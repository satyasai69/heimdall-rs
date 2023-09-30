use heimdall_core::decompile::DecompilerArgsBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = heimdall_core::decompile::decompile(
        DecompilerArgsBuilder::new()
            .target("0x9f00c43700bc0000Ff91bE00841F8e04c0495000".to_string())
            .rpc_url("https://eth.llamarpc.com".to_string())
            .build()?,
    )
    .await?;

    println!("Decompiled contract: {:#?}", result);

    Ok(())
}
