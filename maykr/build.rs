use winres::WindowsResource;

fn main() -> std::io::Result<()> {
    WindowsResource::new().compile()?;

    Ok(())
}
