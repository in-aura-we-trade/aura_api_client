use core::fmt;

use crate::types::ParsedTradeUi;

impl fmt::Debug for ParsedTradeUi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsedTradeUi::Buy {
                mint,
                ticker,
                base,
                quote,
                mc_usd,
                price_usd,
            } => {
                writeln!(f, "Buy {{")?;
                writeln!(f, "  mint: {mint},")?;
                writeln!(f, "  ticker: \"{ticker}\",")?;
                writeln!(f, "  base: {base},")?;
                writeln!(f, "  quote: {quote},")?;

                if let Some(v) = mc_usd {
                    writeln!(f, "  mc_usd: {v},")?;
                } else {
                    writeln!(f, "  mc_usd: None,")?;
                }

                writeln!(f, "  price_usd: {price_usd},")?;
                write!(f, "}}")
            }

            ParsedTradeUi::Sell {
                mint,
                ticker,
                base,
                quote,
                pnl,
                mc_usd,
                price_usd,
            } => {
                writeln!(f, "Sell {{")?;
                writeln!(f, "  mint: {mint},")?;
                writeln!(f, "  ticker: \"{ticker}\",")?;
                writeln!(f, "  base: {base},")?;
                writeln!(f, "  quote: {quote},")?;

                if let Some(v) = pnl {
                    writeln!(f, "  pnl: {v},")?;
                } else {
                    writeln!(f, "  pnl: None,")?;
                }

                if let Some(v) = mc_usd {
                    writeln!(f, "  mc_usd: {v},")?;
                } else {
                    writeln!(f, "  mc_usd: None,")?;
                }

                writeln!(f, "  price_usd: {price_usd},")?;
                write!(f, "}}")
            }
        }
    }
}
