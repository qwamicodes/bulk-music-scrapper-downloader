use std::env;
use std::error::Error;
use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let c = Client::new();

    let urls: Vec<&str> = vec![

    ];

    for url in urls {
        println!("Visiting page: {}", url);  // Debug log to check page visit
        let response = c.get(url).send().await?.text().await?;
        let document = Html::parse_document(&response);
        let class_selectors: Vec<&str> = vec![".entry-content > p > strong > a", ".entry-content > strong > a", ".entry-content > p > a", ".entry-content > h3 > strong > a", ".mb_map_master"];

        for class_selector in class_selectors {
            // Check for the actual download button
            let selector = Selector::parse(class_selector).unwrap();
            let mut found = false;  // Track if the download link was found

            for element in document.select(&selector) {
                let download_link = element.value().attr("href").unwrap();
                println!("Found download link: {}", download_link);  // Debug the download link
                found = true;
                download_music(&c, download_link).await?;
            }

            if !found {
                println!("No download button found on {}", url);
            } else {
                break;
            }
        }
    }


    Ok(())
}

async fn download_music(client: &Client, url: &str) -> Result<(), Box<dyn Error>> {
    println!("Downloading this file: {}", url);  // Debug log the download start
    let mut response = client.get(url).send().await?;
    let filename = url.split("/").last().unwrap();

    let home_dir = env::var("HOME").expect("Could not get the home directory");

    let folder = format!("{}/Music/Gospel", home_dir);
    tokio::fs::create_dir_all(&folder).await?;
    let file_path = std::path::Path::new(&folder).join(filename);

    let mut file = tokio::fs::File::create(&file_path).await?;
    while let Some(chunk) = response.chunk().await? {
        tokio::io::copy(&mut &chunk[..], &mut file).await?;
    }

    println!("Downloaded {}", file_path.display());  // Debug log on download complete
    Ok(())
}

