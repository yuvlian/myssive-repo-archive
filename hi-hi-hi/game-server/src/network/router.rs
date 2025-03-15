use crate::handlers::*;
use database::Arclite;
use hi3_proto::cmd::*;
use paste::paste;

macro_rules! handle {
    ($($handler:ident);* $(;)?) => {
        paste! {
            pub async fn ping_pong((cmd_id, user_id, body): (u32, u32, &[u8]), db: &Arclite) -> Vec<u8> {
                match cmd_id {
                    $(
                        [<$handler:upper _REQ>] => $handler::handle(user_id, body, db).await,
                    )*
                    _ => Vec::with_capacity(0),
                }
            }
        }
    };
}
handle![
    client_report;
    get_bulletin;
    get_card_product_info;
    get_client_data;
    get_login_activity;
    get_player_token;
    player_login;
    set_client_data;
];
