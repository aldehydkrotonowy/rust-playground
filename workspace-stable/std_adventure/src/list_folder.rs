fn try_list_folder_content() -> std::io::Result<()> {
    let entries = std::fs::read_dir("/home/norbert")?;

    for entry in entries {
        println!("Name: {}", entry?.path().display());
    }

    Ok(())
}

pub fn run() {
  let res = try_list_folder_content();

    if let Err(e) = res {
        println!("Error: {}", e);
    }
  }