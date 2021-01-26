/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use std::fmt;

#[derive(Default, Debug, Deserialize)]
pub struct Base {
    pub id: u32,
    pub name: String,
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct Issue {
    pub id: u32,
    pub project: Base,
    pub tracker: Base,
    pub status: Base,
    pub priority: Base,
    pub author: Base,
    pub assigned_to: Option<Base>,
    pub subject: String,
    pub description: String,
    pub start_date: String, // TODO: Convert to date
    pub due_date: Option<String>,
    pub done_ratio: u8,
    pub is_private: bool,
    pub estimated_hours: Option<u32>,
    pub created_on: String,
    pub updated_on: String,
    pub closed_on: Option<String>,
}

impl Issue {
    pub fn as_message(&self, base_url: &str) -> String {
        let mut message: String = format!(
            "<b>Issue name:</b> <a href=\"{}/issues/{}\">{}</a><br>",
            base_url, self.id, self.subject
        );
        // message.push_str(&format!("<b>Status:</b> {}<br>", self.status));
        message.push_str(&format!("<b>Author:</b> {}<br>", self.author));
        // message.push_str(&format!("<b>Assigned:</b> {}<br>", self.assigned_to));
        message.push_str(&format!("<b>Priority:</b> {}<br>", self.priority));
        message.push_str(&format!("<b>Progress:</b> {}%", self.done_ratio));
        // message.push_str("<b>Description:</b><br>");
        // message.push_str("<pre><code class=\"language-rust\">");
        // message.push_str(&format!("{}", self.description));
        // message.push_str("</code></pre>");
        message
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct Issues {
    pub issues: Vec<Issue>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

impl Issues {
    pub fn as_message(&self, base_url: &str) -> String {
        let mut message: String = format!(
            "<b>Issues:</b> {}<br>",
            self.issues
                .iter()
                .map(|x| x.id.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        // message.push_str(&format!("<b>Count:</b> {}<br>", self.total_count));
        message.push_str("<b>Info:</b>");
        for issue in &self.issues {
            message.push_str("<br><br>");
            message.push_str(&issue.as_message(base_url));
        }

        message
    }
}
