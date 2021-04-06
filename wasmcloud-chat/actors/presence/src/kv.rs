use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter::FromIterator;
use wapc_guest::prelude::*;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloudchat_presence_interface::*;

const USER_PRESENCE_EXPIRATION_SECONDS: i32 = 10 * 60; // 10 minutes
const PRESENCE_USERS: &str = "presence:users";

pub(crate) fn store_tracking_report(report: PresenceTrackReport) -> HandlerResult<()> {
    let detail_key = format!("presence:{}", report.user_id);
    let mut detail = get_user_detail(&report.user_id)?.unwrap_or(UserPresence {
        user_id: report.user_id.to_string(),
        channels: vec![],
    });
    let mut hs: HashSet<String> = HashSet::from_iter(detail.channels.iter().cloned());
    hs.insert(report.channel_id);

    detail.channels = hs.into_iter().collect();

    let _ = keyvalue::default().set(
        detail_key,
        serde_json::to_string(&detail)?,
        USER_PRESENCE_EXPIRATION_SECONDS,
    )?;

    keyvalue::default().set_add(PRESENCE_USERS.to_string(), report.user_id.to_string());

    Ok(())
}

pub(crate) fn get_online_users() -> HandlerResult<OnlineUserList> {
    let mut expired: Vec<String> = vec![];
    let mut presences: Vec<PresenceUser> = vec![];
    let online = keyvalue::default()
        .set_query(PRESENCE_USERS.to_string())?
        .values;

    for user in online {
        let detail = get_user_detail(&user)?;
        if let Some(up) = detail {
            presences.push(PresenceUser {
                user_id: up.user_id,
                channels: up.channels,
                last_seen_mins: 0, // TODO: do actual calculation of time when the `wasmcloud:time` provider exists
            })
        } else {
            expired.push(user);
        }
    }

    // Remove users from the online list that have expired detail keys
    for user in expired {
        let _ = keyvalue::default().set_remove(PRESENCE_USERS.to_string(), user.to_string())?;
    }

    Ok(OnlineUserList { users: presences })
}

fn get_user_detail(user_id: &str) -> HandlerResult<Option<UserPresence>> {
    let detail_key = format!("presence:{}", user_id);
    let gr = keyvalue::default().get(detail_key.clone())?;
    Ok(if gr.exists {
        Some(serde_json::from_str(&gr.value)?)
    } else {
        None
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
struct UserPresence {
    pub user_id: String,
    pub channels: Vec<String>,
}
