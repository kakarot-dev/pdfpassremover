use lopdf::{Document, Error};

fn main() -> Result<(), Error> {
    // take password from user in terminal
    println!("Enter the password to decrypt the PDF's: ");
    println!("Note: The password should be the same for all the PDF's");
    let password = take_input();

    // filter the current directory for all pdf files
    let paths = std::fs::read_dir(".")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "pdf"))
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    println!("Found {} PDF's in the current directory", paths.len());
    println!("Decrypting the PDF's...");

    // decrypt all the pdf files
    paths.iter().for_each(|path| {
        if let Err(e) = decrypt_pdf(path.to_str().unwrap(), &password) {
            println!("Failed to decrypt the document named {}", path.to_str().unwrap());
            println!("Error: {}", e);
        } else {
            println!("Decrypted the document named {}", path.to_str().unwrap());
        }
    });
    Ok(())
}

fn decrypt_pdf(path: &str, password: &str) -> Result<(), Error> {
    let mut pdf = Document::load(path)?;
    if pdf.decrypt(password.as_bytes()).is_err() {
        println!("Failed to decrypt the document named {}", path);
        return Ok(());
    }

    pdf.compress();
    std::fs::remove_file(path)?;
    pdf.save(path)?;
    Ok(())
}

fn take_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}