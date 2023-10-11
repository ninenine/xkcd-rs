use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use tokio::time::Duration;

// Define a struct to represent a comic.
#[derive(Serialize, Deserialize)]
struct Comic {
    title: String,
    alt: String,
    img: String,
    num: i32,
}

#[tokio::main]
async fn main() {
    // Define the base URL for the XKCD comics website.
    let base_url = "https://xkcd.com/";

    // Define the output directory for the downloaded comics.
    let output_dir = "./xkcd_comics";

    // Define the maximum number of concurrent downloads.
    let max_concurrent_downloads = 100; // Adjust the batch size as needed.

    // Define the maximum number of retries for failed requests.
    let max_retries = 3;

    // Define the delay between retries for failed requests.
    let retry_delay = Duration::from_secs(5);

    // Create the output directory if it doesn't exist.
    create_dir_all(output_dir)
        .await
        .expect("Failed to create output directory");

    // Get the latest XKCD comic number to determine the range.
    let latest_comic = match get_latest_comic(&base_url, max_retries, retry_delay).await {
        Ok(latest_comic) => latest_comic,
        Err(e) => panic!("❌ Failed to fetch latest comic info: {}", e),
    };

    // Create a semaphore to control concurrent downloads.
    let semaphore = Arc::new(Semaphore::new(max_concurrent_downloads));

    let mut tasks = vec![];

    // Download each comic in the range.
    for comic_num in 1..=latest_comic {
        let semaphore = semaphore.clone();

        let task = tokio::spawn(async move {
            // Acquire a semaphore permit to control concurrent downloads.
            let permit = semaphore
                .acquire()
                .await
                .expect("Failed to acquire semaphore permit");

            // Download the comic data.
            let comic_data =
                match download_comic(&base_url, comic_num, max_retries, retry_delay).await {
                    Ok(comic_data) => comic_data,
                    Err(e) => {
                        println!("❌ Failed to download comic {}: {}", comic_num, e);
                        drop(permit); // Release the semaphore permit.
                        return;
                    }
                };

            // Save the downloaded comic data to a file.
            let file_path = format!("{}/{}-{}", output_dir, comic_num, comic_data.1);
            let path = std::path::Path::new(&file_path);
            if path.exists() {
                println!("👀 Comic {} already downloaded", file_path);
            } else {
                if let Err(e) = save_comic_data_to_file(&file_path, &comic_data.0).await {
                    println!("❌ Failed to save comic {}: {}", comic_num, e);
                } else {
                    println!("✅ Downloaded and saved comic #{}-{}", comic_num, file_path);
                }
            }

            // Release the semaphore permit.
            drop(permit);
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete.
    for task in tasks {
        task.await.expect("Task failed");
    }

    println!("🎉 All XKCD comics downloaded and saved successfully!");
}

// Get the latest XKCD comic number.
async fn get_latest_comic(
    base_url: &str,
    max_retries: u32,
    retry_delay: Duration,
) -> Result<i32, String> {
    let mut retries = 0;

    loop {
        // Send a GET request to the XKCD website to get the latest comic info.
        let response = match reqwest::get(format!("{}info.0.json", base_url)).await {
            Ok(response) => response,
            Err(e) => {
                retries += 1;
                if retries > max_retries {
                    return Err(format!("❌ Failed to get latest comic info: {}", e));
                } else {
                    tokio::time::sleep(retry_delay).await;
                    continue;
                }
            }
        };

        // Parse the JSON response into a Comic struct.
        let comic = match response.json::<Comic>().await {
            Ok(comic) => comic,
            Err(e) => {
                retries += 1;
                if retries > max_retries {
                    return Err(format!("❌ Failed to parse JSON: {}", e));
                } else {
                    tokio::time::sleep(retry_delay).await;
                    continue;
                }
            }
        };

        // Return the latest comic number.
        return Ok(comic.num);
    }
}

// Download a comic.
async fn download_comic(
    base_url: &str,
    comic_num: i32,
    max_retries: u32,
    retry_delay: Duration,
) -> Result<(Vec<u8>, String), String> {
    let mut retries = 0;

    loop {
        // Send a GET request to the XKCD website to get the comic info.
        let img_url = format!("{}{}/info.0.json", base_url, comic_num);
        let response = match reqwest::get(&img_url).await {
            Ok(response) => response,
            Err(e) => {
                retries += 1;
                if retries > max_retries {
                    return Err(format!("❌ Failed to get comic info: {}", e));
                } else {
                    println!("🔍 Retrying get metadata {}. #{}...", img_url, retries);
                    tokio::time::sleep(retry_delay).await;
                    continue;
                }
            }
        };

        // Parse the JSON response into a Comic struct.
        let comic = match response.json::<Comic>().await {
            Ok(comic) => comic,
            Err(e) => {
                retries += 1;
                if retries > max_retries {
                    return Err(format!("❌ Failed to parse JSON: {}", e));
                } else {
                    println!("🔍 Retrying to download comic {}. #{}...", img_url, retries);
                    tokio::time::sleep(retry_delay).await;
                    continue;
                }
            }
        };

        // Download the comic image data.
        let img_url = comic.img;
        let filename = img_url.split('/').last().unwrap().to_string();
        let image_data = match reqwest::get(&img_url).await {
            Ok(response) => match response.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(e) => {
                    retries += 1;
                    if retries > max_retries {
                        return Err(format!("❌ Failed to get image data: {}", e));
                    } else {
                        tokio::time::sleep(retry_delay).await;
                        continue;
                    }
                }
            },
            Err(e) => {
                retries += 1;
                if retries > max_retries {
                    return Err(format!("❌ Failed to download comic image: {}", e));
                } else {
                    tokio::time::sleep(retry_delay).await;
                    continue;
                }
            }
        };

        // Return the downloaded comic data.
        return Ok((image_data, filename));
    }
}

// Save comic data to a file.
async fn save_comic_data_to_file(file_path: &str, comic_data: &[u8]) -> Result<(), String> {
    let mut file = match File::create(file_path).await {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to create file {}: {}", file_path, e)),
    };

    if let Err(e) = file.write_all(comic_data).await {
        return Err(format!("Failed to write to file {}: {}", file_path, e));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_save_comic_data_to_file() {
        let test_path = "./test_comic.jpg";
        let test_data = vec![1, 2, 3, 4, 5]; // Mock data

        // Run the function
        let result = save_comic_data_to_file(test_path, &test_data).await;

        // Ensure no error is returned
        assert!(result.is_ok());

        // Check if file was written correctly
        let saved_data = fs::read(test_path).await.unwrap();
        assert_eq!(saved_data, test_data);

        // Cleanup
        let _ = fs::remove_file(test_path).await;
    }
}
