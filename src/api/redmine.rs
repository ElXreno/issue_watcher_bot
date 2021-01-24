/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::structs::redmine::{Issue, Issues};
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::Client;

pub async fn get_client(api_key: &Option<String>, base_url: &str) -> Client {
    let mut default_headers = HeaderMap::new();
    if let Some(api_key) = api_key {
        default_headers.insert(
            "X-Redmine-API-Key",
            HeaderValue::from_str(&api_key).unwrap(),
        );
    }

    let client = Client::builder()
        .default_headers(default_headers)
        .build()
        .unwrap();

    println!("Redmine: Checking API key...");
    let client = if client
        .get(&format!("{}/my/account.json", base_url))
        .send()
        .await
        .unwrap()
        .status()
        != StatusCode::OK
    {
        println!("Redmine: WARNING: API key is not valid! Working without API key.");
        Client::builder().build().unwrap()
    } else {
        println!("Redmine: API key is valid!");
        client
    };

    client
}

pub async fn get_issue(issue_id: u32, client: &Client, base_url: &str) -> reqwest::Result<Issue> {
    let response = client
        .get(&format!("{}/issue.json?issue_id={}", base_url, issue_id))
        .send()
        .await
        .unwrap()
        .json::<Issue>()
        .await;

    response
}

pub async fn get_all_issues(client: &Client, base_url: &str) -> reqwest::Result<Issues> {
    let response = client
        .get(&format!("{}/issues.json?status_id=open", base_url))
        .send()
        .await
        .unwrap()
        .json::<Issues>()
        .await;

    response
}
