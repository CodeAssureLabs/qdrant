//! Probe module: bounces a counter to [`crate::gtprobe_pong`].

use crate::gtprobe_pong::pong;

/// Decrements `remaining` and hands the turn over to [`pong`] until it reaches zero.
pub fn ping(remaining: u32) -> u32 {
    if remaining == 0 {
        return 0;
    }
    pong(remaining - 1) + 1
}
