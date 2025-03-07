mod github;
mod summarizer;
mod joke;
mod utils;

use anyhow::{Context, Result};
use std::env;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    info!("Starting PR Summarizer");

    // Load the GitHub event payload
    let event_path = env::var("GITHUB_EVENT_PATH")
        .context("GITHUB_EVENT_PATH environment variable not set")?;
    
    let event_payload = utils::read_github_event(&event_path)
        .context("Failed to read GitHub event payload")?;
    
    // Extract PR number and repository from the event
    let (repo_owner, repo_name) = utils::get_repository_info()
        .context("Failed to get repository information")?;
    
    let pr_number = utils::extract_pr_number(&event_payload)
        .context("Failed to extract PR number from event payload")?;
    
    // Create GitHub client
    let token = env::var("GITHUB_TOKEN")
        .context("GITHUB_TOKEN environment variable not set")?;
    
    let github_client = github::create_github_client(&token)
        .context("Failed to create GitHub client")?;
    
    // Get PR information
    let pr_info = github::get_pr_info(&github_client, &repo_owner, &repo_name, pr_number)
        .await
        .context("Failed to get PR information")?;
    
    // Generate summary
    let summary = summarizer::generate_summary(&pr_info)
        .context("Failed to generate summary")?;
    
    // Fetch a random joke
    let joke = joke::fetch_random_joke()
        .await
        .context("Failed to fetch random joke")?;
    
    // Combine summary and joke into a comment with emojis
    let comment = format!(
        "# 🚀 PR Summary\n\n## 📝 Changes Overview\n{}\n\n## 📂 Affected Files\n{}\n\n## 😄 Code Humor\n{}\n\n---\n*This summary was automatically generated by [PR Summarizer](https://github.com/{}/{})* ⚡",
        summary.description,
        summary.affected_files,
        joke,
        repo_owner,
        repo_name
    );
    
    // Post comment to PR
    github::post_pr_comment(&github_client, &repo_owner, &repo_name, pr_number, &comment)
        .await
        .context("Failed to post PR comment")?;
    
    info!("Successfully posted PR summary with joke");
    Ok(())
}