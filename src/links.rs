//! Shared Aura Telegram links. Returned URLs are plain text; callers apply
//! escaping for their output format (for example, Telegram HTML).

use core::fmt::Display;

pub use crate::consts::{AURA_TG_BOT_LINK, AURA_TG_GROUP_LINK};

/// A referral link for a numeric user ID or a validated referral nickname.
pub fn referral_link(referrer: impl Display) -> String {
    format!("{AURA_TG_BOT_LINK}?start={referrer}")
}

/// A referral link that also opens a token. `token` may be an address or the
/// `TOKENADDRESS` placeholder when displaying a link template.
pub fn referral_token_link(referrer: impl Display, token: impl Display) -> String {
    referral_link(format_args!("{referrer}_{token}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_and_vanity_links_use_the_bot() {
        assert_eq!(
            referral_link(123456_u64),
            "https://t.me/trade_with_aura_bot?start=123456"
        );
        assert_eq!(
            referral_link("AuraFriend"),
            "https://t.me/trade_with_aura_bot?start=AuraFriend"
        );
        assert_eq!(AURA_TG_GROUP_LINK, "https://t.me/trade_with_aura");
    }

    #[test]
    fn token_links_preserve_the_referrer_and_token() {
        let mint = solana_address::Address::default();
        assert_eq!(
            referral_token_link("AuraFriend", mint),
            "https://t.me/trade_with_aura_bot?start=AuraFriend_11111111111111111111111111111111"
        );
        assert_eq!(
            referral_token_link(123456_u64, "TOKENADDRESS"),
            "https://t.me/trade_with_aura_bot?start=123456_TOKENADDRESS"
        );
    }
}
