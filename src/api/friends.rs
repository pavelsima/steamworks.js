use napi_derive::napi;

/// A friend accepted an invite or pressed Join while this app runs (Steam's
/// GameRichPresenceJoinRequested_t). With the app closed, Steam starts it with `connect`
/// on the command line instead.
#[derive(Clone, Debug, serde::Serialize)]
pub struct GameRichPresenceJoinRequested {
    pub friend_steam_id: steamworks::SteamId,
    pub connect: String,
}

unsafe impl steamworks::Callback for GameRichPresenceJoinRequested {
    const ID: i32 =
        steamworks::sys::GameRichPresenceJoinRequested_t__bindgen_ty_1::k_iCallback as i32;
    const SIZE: i32 =
        std::mem::size_of::<steamworks::sys::GameRichPresenceJoinRequested_t>() as i32;

    unsafe fn from_raw(raw: *mut std::ffi::c_void) -> Self {
        let val = &*(raw as *const steamworks::sys::GameRichPresenceJoinRequested_t);
        let connect = std::ffi::CStr::from_ptr(val.m_rgchConnect.as_ptr())
            .to_string_lossy()
            .into_owned();
        GameRichPresenceJoinRequested {
            friend_steam_id: steamworks::SteamId::from_raw(val.m_steamIDFriend.m_steamid.m_unAll64Bits),
            connect,
        }
    }
}

#[napi]
pub mod friends {
    use napi::bindgen_prelude::BigInt;
    use steamworks::sys;
    use steamworks::FriendFlags;

    #[napi(object)]
    pub struct FriendInfo {
        pub steam_id64: BigInt,
        pub name: String,
        /// Any persona state but offline (online, busy, away, …).
        pub online: bool,
        /// Playing this app right now.
        pub playing_this_game: bool,
    }

    /// The player's immediate friends (not requests, not blocked).
    #[napi]
    pub fn get_friends() -> Vec<FriendInfo> {
        let client = crate::client::get_client();
        let app_id = client.utils().app_id();
        client
            .friends()
            .get_friends(FriendFlags::IMMEDIATE)
            .into_iter()
            .map(|friend| {
                let id = friend.id().raw();
                // Read through sys: Friend::state() panics on a persona state it does not know.
                let state = unsafe {
                    sys::SteamAPI_ISteamFriends_GetFriendPersonaState(
                        sys::SteamAPI_SteamFriends_v017(),
                        id,
                    )
                };
                FriendInfo {
                    steam_id64: BigInt::from(id),
                    name: friend.name(),
                    online: state != sys::EPersonaState::k_EPersonaStateOffline,
                    playing_this_game: friend
                        .game_played()
                        .map(|game| game.game.app_id() == app_id)
                        .unwrap_or(false),
                }
            })
            .collect()
    }

    /// Invites a friend to this app with a connect string (ISteamFriends::InviteUserToGame).
    /// Returns false when Steam refuses it (an invalid id).
    #[napi]
    pub fn invite_user_to_game(steam_id64: BigInt, connect: String) -> bool {
        let _client = crate::client::get_client();
        let Ok(connect) = std::ffi::CString::new(connect) else {
            return false;
        };
        let (negative, id, lossless) = steam_id64.get_u64();
        if negative || !lossless {
            return false;
        }
        unsafe {
            sys::SteamAPI_ISteamFriends_InviteUserToGame(
                sys::SteamAPI_SteamFriends_v017(),
                id,
                connect.as_ptr(),
            )
        }
    }
}
