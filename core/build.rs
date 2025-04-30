use std::fs::{self, File, DirEntry};
use std::io;
use std::path::Path;


// Function to recursively copy a directory and its contents
fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    // Create the destination directory if it doesn't exist
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    // Iterate over the source directory's entries
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        // If it's a directory, recurse
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            // If it's a file, copy it
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let source = Path::new("../resources"); // Source directory
    let destination = Path::new("../web/assets/"); // Destination directory

    copy_dir_recursive(source, destination)?;
    println!("Directory copied successfully!");

    // let path = "../resources"; // Specify your folder path
    // let mut output_file = File::create("filenames.txt")?; // Output file
    //
    // for entry in fs::read_dir(path)? {
    //     let entry = entry?;
    //     let file_name = entry.file_name().into_string().unwrap_or_default();
    //     writeln!(output_file, "{}", file_name)?; // Write each filename to the file
    // }

    Ok(())
}
