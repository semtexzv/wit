fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("OUT_DIR", "./src/proto");
    quix_build::Config::default()
        .service_generator(quix_build::generator(true))
        .type_attribute("wit.control.Rule", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .compile_protos(
            &["./proto/control.proto"],
            &["./proto"],
        )?;
    Ok(())
}