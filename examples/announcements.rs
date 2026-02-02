use librus_rs::Client;

#[tokio::main]
async fn main() -> Result<(), librus_rs::Error> {
    println!("Authenticating with Librus...");
    let client = Client::from_env().await?;

    println!("Fetching school notices...");
    let notices = client.school_notices_latest(10).await?;

    for notice in notices {
        let content = Client::notice_content_to_text(&notice.content);
        let preview: String = content.chars().take(120).collect();
        println!(
            "[{}] {} - {}",
            notice.creation_date, notice.subject, preview
        );
    }

    Ok(())
}
