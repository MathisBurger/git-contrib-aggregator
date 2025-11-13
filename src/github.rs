use chrono::{Datelike, Utc};
use reqwest::Client;
use serde_json::json;

use crate::data_model::{ContributionsResponse, SourceTypeTotals};

pub async fn fetch_github_contributions(token: &str, result: &mut ContributionsResponse) {
    let client = Client::new();

    let year = Utc::now().year();
    let from = format!("{year}-01-01T00:00:00Z");
    let to = format!("{year}-12-31T23:59:59Z");

    let query = r#"
        query($from: DateTime!, $to: DateTime!) {
            viewer {
                contributionsCollection(from: $from, to: $to) {
                    contributionCalendar {
                        totalContributions
                        weeks {
                            contributionDays {
                                date
                                contributionCount
                            }
                        }
                    }
                    totalCommitContributions
                    totalIssueContributions
                    totalPullRequestContributions
                }
            }
        }
    "#;

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .header("User-Agent", "rust-contribution-fetcher")
        .json(&json!({
            "query": query,
            "variables": { "from": from, "to": to }
        }))
        .send()
        .await
        .expect("Cannot send request");

    let data: serde_json::Value = res.json().await.expect("Cannot fetch github contributions");

    let viewer = &data["data"]["viewer"];
    let collection = &viewer["contributionsCollection"];

    if let Some(weeks) = collection["contributionCalendar"]["weeks"].as_array() {
        for week in weeks {
            if let Some(days) = week["contributionDays"].as_array() {
                for day in days {
                    if let (Some(date), Some(count)) =
                        (day["date"].as_str(), day["contributionCount"].as_u64())
                    {
                        if count > 0 {
                            let entry = result.by_date.entry(date.to_string()).or_default();
                            *entry.entry("github".to_string()).or_default() += count as u32;
                        }
                    }
                }
            }
        }
    }

    let totals = SourceTypeTotals {
        commits: collection["totalCommitContributions"].as_u64().unwrap_or(0) as u32,
        issues: collection["totalIssueContributions"].as_u64().unwrap_or(0) as u32,
        merge_requests: collection["totalPullRequestContributions"]
            .as_u64()
            .unwrap_or(0) as u32,
    };

    result.by_source_total.insert("github".into(), totals);
}
