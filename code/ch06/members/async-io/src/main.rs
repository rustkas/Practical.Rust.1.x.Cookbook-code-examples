use std::path::Path;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = "".to_string();
    let input_filename = "file.txt";
    let output_filename = "output.txt";

    let remove_old_file = true;

    if remove_old_file {
        // let _ = fs::remove_file(output_filename).await;
        // // Проверяем, существует ли файл
        if Path::new(input_filename).try_exists()? {
            // Если файл существует, удаляем его
            let _ = fs::remove_file(output_filename).await;
            println!("File {} successfully deleted", output_filename);
        } else {
            println!("File {} does not exist", output_filename);
        }
    }

    if Path::new(input_filename).try_exists()? {
        // Read the contents of a file.
        contents = fs::read_to_string("file.txt").await?;
        println!("Input file contents: {contents}");
    }

    // if Path::new(output_filename).try_exists()? {
    //     println!("{}{}", output_filename, " already exists");
    // } else {
    //     println!("{}{}", output_filename, " not exist");
    // }

    if Path::new(output_filename).try_exists()? {
        fs::write(output_filename, "New file\n\r").await?;
        println!("{}{}", output_filename, "File already exists");
    } else {
        fs::write(output_filename, "Create new file\n\r").await?;
        println!("{}{}", output_filename, " created");
    }


    // Открываем файл "output.txt" в режиме добавления
    let mut file = OpenOptions::new()
        .append(true)
        .create(false)
        .open(output_filename)
        .await?;

    // Записываем содержимое в файл "output.txt"
    file.write_all((&contents).as_bytes()).await?;
    Ok(())
}
