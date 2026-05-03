use core::fmt::Display;

use decisol::D128;

use decisol::UD128;

use decisol::udec128;
use log::error;

use chrono::TimeDelta;

use decisol::dec128;
use solana_address::Address;

use solana_signature::Signature;

pub fn bps_dec_unlim(bps: u64) -> UD128 {
    let bps = UD128::from(bps);
    bps / udec128!(10_000)
}

pub fn bps_dec(bps: u32) -> UD128 {
    if bps > 10_000 {
        error!("bps_dec overflowed with: {bps}");
    };
    let bps = bps.min(10_000);
    let bps = UD128::from(bps);
    bps / udec128!(10_000)
}
pub fn to_bps_dec(dec: UD128) -> u32 {
    match (dec * udec128!(10_000)).floor().to_u32() {
        Ok(v) => v,
        Err(_) => {
            error!("to_bps_dec overflowed with: {dec}");
            10_000
        }
    }
}

pub fn to_bps_dec_unlim(dec: UD128) -> u64 {
    match (dec * udec128!(10_000)).floor().to_u64() {
        Ok(v) => v,
        Err(_) => {
            error!("to_bps_dec overflowed with: {dec}");
            10_000
        }
    }
}
pub fn to_bps_signed_dec_unlim(dec: D128) -> u64 {
    let mut dec = dec + D128::ONE;
    if !dec.is_positive() {
        dec = D128::ZERO;
    }
    match (dec.try_to_unsigned().unwrap() * udec128!(10_000))
        .floor()
        .to_u64()
    {
        Ok(v) => v,
        Err(_) => {
            error!("to_bps_dec overflowed with: {dec}");
            10_000
        }
    }
}

pub fn price_formatter(num: UD128, scale: i16, round: bool) -> String {
    let num = if round { num.round(8) } else { num };
    //println!("fn price_formatter num: {}", num);
    let decimal_str = num.to_string();
    //println!("fn price_formatter str: {}", decimal_str);
    let parts: Vec<&str> = decimal_str.split('.').collect();
    if parts.len() != 2 {
        return decimal_str;
    }

    let integer_part: u64 = parts[0].parse().unwrap();
    if integer_part != 0 {
        return num.rescale(scale).to_string();
    };
    let decimal_part_og = parts[1];
    let mut i = 0;
    let mut i_rev = decimal_part_og.len();
    let mut iter = decimal_part_og.chars();

    while let Some('0') = iter.next() {
        i += 1;
    }
    if i_rev == i {
        return num.rescale(scale).to_string();
    }
    let mut iter = decimal_part_og.chars();
    while let Some('0') = iter.next_back() {
        i_rev -= 1;
    }
    let scale = scale as usize;
    //info!("Integer part og: {decimal_part_og}, i: {i}, i_rev: {i_rev}");
    let decimal_part = &decimal_part_og[i..i_rev];
    let i_str = match i {
        0 => "₀",
        1 => "₁",
        2 => "₂",
        3 => "₃",
        4 => "₄",
        5 => "₅",
        6 => "₆",
        7 => "₇",
        8 => "₈",
        9 => "₉",
        10 => "₁₀",
        11 => "₁",
        12 => "₁₂",
        _ => "₍₎",
    };
    if i > 3 {
        format!("{integer_part}.0{i_str}{decimal_part:.3}")
    } else {
        format!("{integer_part}.{decimal_part_og:.scale$}")
    }
}

pub fn format_pubkey(pubkey: &Address, with_link: bool) -> String {
    let pubkey = pubkey.to_string();
    let start = &pubkey[0..3];
    let end = &pubkey[pubkey.len() - 4..];
    if with_link {
        if cfg!(feature = "dev") {
            format!(
                "<a href=\"https://solscan.io/account/{pubkey}?cluster=custom&customUrl=http://127.0.0.1:8899\">{start}...{end}</a>"
            )
        } else {
            format!("<a href=\"https://solscan.io/account/{pubkey}\">{start}...{end}</a>")
        }
    } else {
        format!("{start}...{end}")
    }
}
pub fn format_wallet(pubkey: &Address) -> String {
    let pubkey = pubkey.to_string();
    pubkey[0..6].to_string()
}
pub fn format_pubkey_str(pubkey: &str, with_link: bool) -> String {
    let start = &pubkey[0..3];
    let end = &pubkey[pubkey.len() - 4..];
    if with_link {
        if cfg!(feature = "dev") {
            format!(
                "<a href=\"https://solscan.io/account/{pubkey}?cluster=custom&customUrl=http://127.0.0.1:8899\">{start}...{end}</a>"
            )
        } else {
            format!("<a href=\"https://solscan.io/account/{pubkey}\">{start}...{end}</a>")
        }
    } else {
        format!("{start}...{end}")
    }
}
pub fn format_signature(sig: &Signature, with_link: bool) -> String {
    let sig = sig.to_string();
    let start = &sig[0..3];
    let end = &sig[sig.len() - 4..];
    if with_link {
        if cfg!(feature = "dev") {
            format!(
                "<a href=\"https://solscan.io/tx/{sig}?cluster=custom&customUrl=http://127.0.0.1:8899\">{start}...{end}</a>"
            )
        } else {
            format!("<a href=\"https://solscan.io/tx/{sig}\">{start}...{end}</a>")
        }
    } else {
        format!("{start}...{end}")
    }
}
pub fn format_signature_str(sig: &str, with_link: bool) -> String {
    let start = &sig[0..3];
    let end = &sig[sig.len() - 4..];
    if with_link {
        if cfg!(feature = "dev") {
            format!(
                "<a href=\"https://solscan.io/tx/{sig}?cluster=custom&customUrl=http://127.0.0.1:8899\">{start}...{end}</a>"
            )
        } else {
            format!("<a href=\"https://solscan.io/tx/{sig}\">{start}...{end}</a>")
        }
    } else {
        format!("{start}...{end}")
    }
}

pub fn format_signature_str_with_text(sig: &str, text: impl Display) -> String {
    if cfg!(feature = "dev") {
        format!(
            "<a href=\"https://solscan.io/tx/{sig}?cluster=custom&customUrl=http://127.0.0.1:8899\">{text}</a>"
        )
    } else {
        format!("<a href=\"https://solscan.io/tx/{sig}\">{text}</a>")
    }
}
pub fn format_signature_with_text(sig: &Signature, text: impl Display) -> String {
    if cfg!(feature = "dev") {
        format!(
            "<a href=\"https://solscan.io/tx/{sig}?cluster=custom&customUrl=http://127.0.0.1:8899\">{text}</a>"
        )
    } else {
        format!("<a href=\"https://solscan.io/tx/{sig}\">{text}</a>")
    }
}

pub fn format_symbol(symbol: &str) -> String {
    const SYMBOL_LEN: usize = 8;
    let symbol = symbol.to_string();
    let symbol_len = symbol.len();
    if symbol_len > SYMBOL_LEN {
        let symbol_new: String = symbol.chars().take(SYMBOL_LEN).collect();
        symbol_new
    } else {
        symbol
    }
}
pub fn mc_formatter(num: impl Into<D128>) -> String {
    let num: D128 = num.into();
    // if dec!(999_000) < num && num < dec!(1_000_000) {
    //     return "1.00M".to_string();
    // }
    if num < dec128!(10000) {
        format!("{num:.0}")
    } else if num < dec128!(999_000) {
        format!("{}K", (num / dec128!(1000)).round(2))
    } else if num < dec128!(999_000_000) {
        format!("{}M", (num / dec128!(1_000_000)).round(2))
    } else {
        format!("{}B", (num / dec128!(1_000_000_000)).round(2))
    }
}

pub fn duration_formatter(num: &TimeDelta) -> String {
    if num.num_seconds() < 120 {
        format!("{}s", num.num_seconds())
    } else if num.num_minutes() < 120 {
        format!("{}m", num.num_minutes())
    } else if num.num_hours() < 48 {
        format!("{}h", num.num_hours())
    } else {
        format!("{}d", num.num_days())
    }
}
