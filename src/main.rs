#[tokio::main]
async fn main() {
    // Future implementation will go here
    let urls = vec!["https://example.com", "https://example.org", "https://example.net"];
    fetch_and_process(urls).await;
}

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn count_words(html: &str) -> usize {
    html.split_whitespace().count()
}

async fn fetch_and_process(urls: Vec<&str>) {
    let fetches = urls.into_iter().map(|url| {
        async move {
            let html = fetch_url(url).await.unwrap();
            let word_count = count_words(&html);
            println!("{} has {} words.", url, word_count);
        }
    });
    futures::future::join_all(fetches).await;
}