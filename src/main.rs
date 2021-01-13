/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use assign::assign;
use ruma::{
    api::client::r0::{filter::FilterDefinition, message::send_message_event, sync::sync_events},
    events::{
        room::message::{MessageEventContent, TextMessageEventContent},
        AnyMessageEventContent, AnySyncMessageEvent, AnySyncRoomEvent, SyncMessageEvent,
    },
    presence::PresenceState,
};
use ruma_client::Client;
use std::process::exit;
use std::time::Duration;
use tokio_stream::StreamExt;

mod api;
mod config;
mod structs;

#[tokio::main]
async fn main() {
    let mut config = config::Config::load();

    let home_server_url = config.matrix_server.parse().unwrap();
    let client = Client::new(home_server_url, config.matrix_session);

    let session = match client
        .log_in(
            &config.matrix_login,
            &config.matrix_password,
            None,
            Some("Issue watcher client"),
        )
        .await
    {
        Ok(s) => s,
        Err(e) => {
            println!("Error on authorization phase! {}", e);
            exit(1);
        }
    };

    config.matrix_session = Some(session);
    config.save_to_file_warn();

    println!("Authorization complete!");

    let filter = FilterDefinition::ignore_all().into();
    let initial_sync_response = client
        .request(assign!(sync_events::Request::new(), {
            filter: Some(&filter),
        }))
        .await
        .unwrap();

    let mut sync_stream = Box::pin(client.sync(
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
                                client
                                    .request(send_message_event::Request::new(
                                        &room_id,
                                        &rand::random::<i32>().to_string(),
                                        &AnyMessageEventContent::RoomMessage(
                                            MessageEventContent::text_plain("Pong!"),
                                        ),
                                    ))
                                    .await
                                    .unwrap();
                            } else if msg_body.starts_with("!issues") {
                                let redmine_client = api::redmine::get_client(&config.redmine_token);
                                let issues = api::redmine::get_all_issues(
                                    &redmine_client,
                                    &config.redmine_server,
                                )
                                .await;
                                if let Ok(issues) = issues {
                                    if issues.issues.len() == 0 {
                                        client
                                            .request(send_message_event::Request::new(
                                                &room_id,
                                                &rand::random::<i32>().to_string(),
                                                &AnyMessageEventContent::RoomMessage(
                                                    MessageEventContent::text_plain(
                                                        "Issues not found!",
                                                    ),
                                                ),
                                            ))
                                            .await
                                            .unwrap();
                                    } else {
                                        let msg = issues.as_message(&config.redmine_server);
                                        client
                                            .request(send_message_event::Request::new(
                                                &room_id,
                                                &rand::random::<i32>().to_string(),
                                                &AnyMessageEventContent::RoomMessage(
                                                    MessageEventContent::text_html(&msg, &msg),
                                                ),
                                            ))
                                            .await
                                            .unwrap();
                                    }
                                } else {
                                    let msg = format!(
                                        "Failed to fetch issues from redmine!<br>Error:<br><pre><code class=\"language-rust\">{:#?}</code></pre>", &issues.err()
                                    );
                                    client
                                        .request(send_message_event::Request::new(
                                            &room_id,
                                            &rand::random::<i32>().to_string(),
                                            &AnyMessageEventContent::RoomMessage(
                                                MessageEventContent::text_html(&msg, &msg),
                                            ),
                                        ))
                                        .await
                                        .unwrap();
                                }
                            } else {
                                client
                                    .request(send_message_event::Request::new(
                                        &room_id,
                                        &rand::random::<i32>().to_string(),
                                        &AnyMessageEventContent::RoomMessage(
                                            MessageEventContent::text_plain(
                                                "I don't know this command!",
                                            ),
                                        ),
                                    ))
                                    .await
                                    .unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
