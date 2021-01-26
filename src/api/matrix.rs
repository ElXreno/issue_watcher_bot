/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use ruma::{
    api::client::r0::message::send_message_event::{Request, Response},
    events::{room::message::MessageEventContent, AnyMessageEventContent},
    RoomId,
};
use ruma_client::{Client, Session};
use std::process::exit;

pub async fn get_client_with_session(
    matrix_server: &str,
    matrix_session: Option<Session>,
    matrix_login: &str,
    matrix_password: &str,
) -> (Client, Session) {
    println!("Matrix: logging in...");

    let matrix_client = {
        let home_server_url = matrix_server.parse().unwrap();
        Client::new(home_server_url, matrix_session)
    };

    let session = match matrix_client
        .log_in(
            matrix_login,
            matrix_password,
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

    println!("Matrix: authorization complete!");

    (matrix_client, session)
}

pub async fn send_plain_message(client: &Client, room_id: &RoomId, message: &str) -> Response {
    client
        .request(Request::new(
            room_id,
            &rand::random::<i32>().to_string(),
            &AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(message)),
        ))
        .await
        .unwrap()
}

pub async fn send_html_message(
    client: &Client,
    room_id: &RoomId,
    plain_message: &str,
    html_message: &str,
) -> Response {
    client
        .request(Request::new(
            &room_id,
            &rand::random::<i32>().to_string(),
            &AnyMessageEventContent::RoomMessage(MessageEventContent::text_html(
                plain_message,
                html_message,
            )),
        ))
        .await
        .unwrap()
}
