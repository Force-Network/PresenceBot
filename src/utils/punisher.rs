use std::sync::Arc;

use serenity::{all::{EditMember, Http, Timestamp, User}, http};
use chrono::{Utc, Duration as ChronoDuration};

use crate::scanners::punishments::Punishment;

pub async fn punish(user: User, punishment: Punishment, http: Arc<Http>, guild_id: u64) {
    match punishment {
        Punishment::Ban(ban) => {
            let _ = http.ban_user(guild_id.into(), user.id, 0, Some(ban.reason.as_str())).await;
            println!("Banned user");
        }
        Punishment::Kick(kick) => {
            let _ = http.kick_member(guild_id.into(), user.id, Some(kick.reason.as_str()));
            println!("Kicked user");
        }
        Punishment::Timeout(timeout) => {
            let time = Utc::now() + ChronoDuration::seconds(timeout.duration as i64);
            let timeoutr = http.get_guild(guild_id.into()).await.unwrap().edit_member(http, user.id, EditMember::new().disable_communication_until(time.to_rfc3339())).await;
            println!("Muted user for {} seconds", timeout.duration);
        }
        Punishment::No(_) => {
            println!("No punishment");
        }
    }

}