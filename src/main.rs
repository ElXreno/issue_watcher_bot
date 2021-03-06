/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use assign::assign;
use ruma::{
    api::client::r0::{filter::FilterDefinition, sync::sync_events},
    events::{
        room::message::{MessageEventContent, TextMessageEventContent},
        AnySyncMessageEvent, AnySyncRoomEvent, SyncMessageEvent,
    },
    presence::PresenceState,
};
use std::time::Duration;
use tokio_stream::StreamExt;

mod api;
mod config;
mod structs;

#[tokio::main]
async fn main() {
    let mut config = config::Config::load();

    let (matrix_client, session) = api::matrix::get_client_with_session(
        &config.matrix_server,
        config.matrix_session,
        &config.matrix_login,
        &config.matrix_password,
    )
    .await;

    config.matrix_session = Some(session);
    config.save_to_file_warn();

    let redmine_client =
        api::redmine::get_client(&config.redmine_token, &config.redmine_server).await;

    let filter = FilterDefinition::ignore_all().into();
    let initial_sync_response = matrix_client
        .request(assign!(sync_events::Request::new(), {
            filter: Some(&filter),
        }))
        .await
        .unwrap();

    let mut sync_stream = Box::pin(matrix_client.sync(
        None,
        initial_sync_response.next_batch,
        &PresenceState::Online,
        Some(Duration::from_secs(30)),
    ));

    while let Some(response) = sync_stream.try_next().await.unwrap() {
        for (room_id, room) in response.rooms.join {
            for event in room
                .timeline
                .events
                .into_iter()
                .flat_map(|r| r.deserialize())
            {
                if let AnySyncRoomEvent::Message(AnySyncMessageEvent::RoomMessage(
                    SyncMessageEvent {
                        content:
                            MessageEventContent::Text(TextMessageEventContent {
                                body: msg_body, ..
                            }),
                        sender,
                        ..
                    },
                )) = event
                {
                    println!("{} in {}: {}", sender, room_id, msg_body);
                    match msg_body.chars().next() {
                        Some('!') => {
                            if msg_body.starts_with("!ping") {
                                api::matrix::send_plain_message(&matrix_client, &room_id, "Pong!")
                                    .await;
                            } else if msg_body.starts_with("!issues") {
                                let issues = api::redmine::get_all_issues(
                                    &redmine_client,
                                    &config.redmine_server,
                                )
                                .await;
                                if let Ok(issues) = issues {
                                    if issues.issues.len() == 0 {
                                        api::matrix::send_plain_message(
                                            &matrix_client,
                                            &room_id,
                                            "Issues not found!",
                                        )
                                        .await;
                                    } else {
                                        let msg = issues.as_message(&config.redmine_server);
                                        api::matrix::send_html_message(
                                            &matrix_client,
                                            &room_id,
                                            &msg,
                                            &msg,
                                        )
                                        .await;
                                    }
                                } else {
                                    let msg = format!(
                                        "Failed to fetch issues from redmine!<br>Error:<br><pre><code class=\"language-rust\">{:#?}</code></pre>", &issues.err()
                                    );
                                    api::matrix::send_html_message(
                                        &matrix_client,
                                        &room_id,
                                        &msg,
                                        &msg,
                                    )
                                    .await;
                                }
                            } else {
                                api::matrix::send_plain_message(
                                    &matrix_client,
                                    &room_id,
                                    "I don't know this command!",
                                )
                                .await;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
