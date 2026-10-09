//! Probe module: bounces a counter back to [`crate::gtprobe_ping`].

use crate::gtprobe_ping::ping;

/// Decrements `remaining` and hands the turn back to [`ping`] until it reaches zero.
pub fn pong(remaining: u32) -> u32 {
    if remaining == 0 {
        return 0;
    }
    ping(remaining - 1) + 1
}
