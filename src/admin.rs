use dialoguer::Select;

pub async fn open() -> std::io::Result<()> {
    let selection = Select::new().items(&["hello", "owo", "uwu"]).interact()?;
    println!("{}", selection);
    Ok(())
}
