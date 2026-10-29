# Frontline Arcana fork (0.4.0-fa.1)

Based on ceifa/steamworks.js master after 0.4.0. Adds:

- `client.friends.getFriends()` → `{ steamId64, name, online, playingThisGame }[]` (immediate friends)
- `client.friends.inviteUserToGame(steamId64, connect)` → `boolean` (ISteamFriends::InviteUserToGame)
- `SteamCallback.GameRichPresenceJoinRequested` → `{ friend_steam_id, connect }`

steamworks-rs gets the `raw-bindings` feature for the persona state, the invite's result and the
callback.

## Building

`npm run types` reads `client.d.ts`, which the Rust build writes, so build first:

    PATH="$PWD/node_modules/.bin:$PATH" node build --release --target aarch64-apple-darwin
    PATH="$PWD/node_modules/.bin:$PATH" node build --release --target x86_64-apple-darwin
    PATH="$PWD/node_modules/.bin:$PATH" npm run types
    npm pack

0.4.0-fa.1 has the new API on macOS only: `dist/win64` and `dist/linux64` are stock 0.4.0
binaries until they are built on Windows and Linux (`--target x86_64-pc-windows-msvc`,
`--target x86_64-unknown-linux-gnu`) or by this repo's GitHub Actions. The launcher
feature-detects the API, so those platforms keep Steam's invite dialog until then.
