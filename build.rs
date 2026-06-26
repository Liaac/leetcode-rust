use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let src = Path::new("src");

    for entry in fs::read_dir(src).expect("read src dir") {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let mut mod_content = String::new();
        let mut files: Vec<String> = fs::read_dir(&path)
            .unwrap()
            .filter_map(|e| {
                let name = e.unwrap().file_name().to_string_lossy().into_owned();
                if name.ends_with(".rs") && name != "mod.rs" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect();
        files.sort();

        for file in &files {
            let mod_name = filename_to_ident(file);
            mod_content.push_str(&format!(
                "#[path = \"{}\"]\npub mod {};\n",
                file, mod_name
            ));
        }

        let mod_path = path.join("mod.rs");
        let existing = fs::read_to_string(&mod_path).unwrap_or_default();
        if existing != mod_content {
            let mut f = fs::File::create(&mod_path).unwrap();
            f.write_all(mod_content.as_bytes()).unwrap();
            println!("cargo:warning=updated {}", mod_path.display());
        }
    }
}

fn filename_to_ident(filename: &str) -> String {
    let stem = filename.strip_suffix(".rs").unwrap_or(filename);
    let ident: String = stem
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect();
    let ident = ident.trim_matches('_');
    if ident.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", ident)
    } else if ident.is_empty() {
        "unknown".to_string()
    } else {
        ident.to_string()
    }
}
