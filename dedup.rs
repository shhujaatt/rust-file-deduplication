use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;


fn compute_hash(file_path: &Path) -> io::Result<String> {
    use sha2::{Digest, Sha256};
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}


fn find_duplicates(directory: &Path) -> io::Result<HashMap<String, Vec<String>>> {
    let mut file_map: HashMap<String, Vec<String>> = HashMap::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let hash = compute_hash(&path)?;
            file_map.entry(hash).or_default().push(path.to_string_lossy().to_string());
        } else if path.is_dir() {
            let nested_duplicates = find_duplicates(&path)?;
            for (hash, paths) in nested_duplicates {
                file_map.entry(hash).or_default().extend(paths);
            }
        }
    }

    Ok(file_map)
}

fn main() -> io::Result<()> {
    let directory = Path::new("./test_directory"); 
    if !directory.exists() || !directory.is_dir() {
        eprintln!("Please provide a valid directory path.");
        return Ok(());
    }

    let duplicates = find_duplicates(directory)?;

    println!("Duplicate Files Found:");
    for (hash, files) in duplicates {
        if files.len() > 1 {
            println!("Hash: {}", hash);
            for file in &files {
                println!("  - {}", file);
            }
            println!();
        }
    }

    Ok(())
}
