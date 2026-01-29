use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Bird to search: ");
    let mut bird = String::new();
    io::stdin().read_line(&mut bird)?;
    let bird = bird.trim();

    for folder in ["one", "two", "three"].iter() {
        println!("\n{}:", folder);
        let path = format!("./src/data/{}", folder);
        
        if let Ok(entries) = std::fs::read_dir(&path) {
            let mut files: Vec<_> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().map_or(false, |ext| ext == "xlsx" || ext == "xls"))
                .collect();
            
            files.sort();
            
            let mut found = false;
            for file in files {
                if let Ok(mut workbook) = open_workbook::<Xlsx<_>, _>(&file) {
                    if let Some(Ok(range)) = workbook.worksheet_range("Beobachtungen") {
                        for (i, row) in range.rows().enumerate().skip(1) {
                            if let (DataType::String(name), DataType::Float(conf)) = (&row[7], &row[8]) {
                                if conf > &0.8 && name.trim() == bird {
                                    println!("  {} (row {})", file.file_name().unwrap().to_string_lossy(), i + 1);
                                    found = true;
                                    break;
                                }
                            }
                        }
                        if found {
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}