use chrono::Datelike;
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    config::GitlabPat,
    data_model::{ContributionsResponse, SourceTypeTotals},
};

#[derive(Deserialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize)]
struct PushData {
    commit_count: u32,
}

#[derive(Deserialize)]
struct Event {
    action_name: String,
    target_type: Option<String>,
    created_at: String,
    push_data: Option<PushData>,
}

/// Helper function to fetch paginated events from GitLab
async fn fetch_paginated_events(client: &Client, url: String, token: &str) -> Vec<Event> {
    let mut results = Vec::new();
    let mut page = 1;

    loop {
        let paged_url = format!("{}&page={}", url, page);
        let resp = client
            .get(&paged_url)
            .header("PRIVATE-TOKEN", token)
            .send()
            .await
            .expect("Cannot send request to GitLab")
            .json::<Vec<Event>>()
            .await
            .expect("Cannot parse response from GitLab");

        if resp.is_empty() {
            break;
        }

        results.extend(resp);
        page += 1;
    }

    results
}

pub async fn fetch_gitlab_contributions(pat: &GitlabPat, response: &mut ContributionsResponse) {
    let client = Client::new();
    let year = Utc::now().year();
    let from = format!("{}-01-01T00:00:00Z", year);
    let to = format!("{}-12-31T23:59:59Z", year);

    // --- Step 1: Fetch user info ---
    let user: User = client
        .get(format!("{}/user", pat.uri))
        .header("PRIVATE-TOKEN", &pat.token)
        .send()
        .await
        .expect("Cannot fetch GitLab user")
        .json()
        .await
        .expect("Cannot parse response from GitLab");

    println!("Fetching contributions for user: {}", user.username);

    // --- Step 2: Fetch events ---
    let url = format!(
        "{}/users/{}/events?after={}&before={}&per_page=100",
        pat.uri, user.id, from, to
    );

    let events = fetch_paginated_events(&client, url, &pat.token).await;

    // Totals
    let mut total_commits = 0;
    let mut total_issues = 0;
    let mut total_mrs = 0;

    // --- Step 3: Aggregate per day ---
    for e in events {
        let date = &e.created_at[..10]; // YYYY-MM-DD
        let mut count = 0;

        match (e.action_name.as_str(), e.target_type.as_deref()) {
            ("pushed to", _) | ("pushed new", _) => {
                // Count real commits
                if let Some(push_data) = e.push_data {
                    count = push_data.commit_count;
                    total_commits += count;
                }
            }
            ("opened", Some("Issue")) | ("opened", Some("WorkItem")) => {
                count = 1;
                total_issues += 1;
            }
            ("opened", Some("MergeRequest")) => {
                count = 1;
                total_mrs += 1;
            }
            _ => {}
        }

        if count > 0 {
            response
                .by_date
                .entry(date.to_string())
                .or_default()
                .entry(pat.name.clone())
                .and_modify(|v| *v += count)
                .or_insert(count);
        }
    }

    // --- Step 4: Fill totals ---
    response.by_source_total.insert(
        pat.name.clone(),
        SourceTypeTotals {
            commits: total_commits,
            issues: total_issues,
            merge_requests: total_mrs,
        },
    );
}
