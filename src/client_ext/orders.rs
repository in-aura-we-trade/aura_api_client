use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;
use decisol::D128;
use decisol::QuoteLamports;
use decisol::UD128;
use decisol::dec128;
use decisol::udec128;

use crate::consts::LIMIT_ORDER_EXP_DUR_DAYS;
use crate::types::ApiLimitOrder;
use crate::types::Direction;

use crate::types::MarketExecuteMode;
use crate::types::OrderEventTrigger;
use crate::types::OrderId;
use crate::types::OrderState;
use crate::types::RawOrder;
use crate::types::SwapAmount;
use crate::types::Target;
use crate::types::TradeType;
use crate::utils::duration_formatter;
use crate::utils::mc_formatter;
use crate::utils::price_formatter;

impl ApiLimitOrder {
    pub fn id(&self) -> Option<OrderId> {
        match &self.state {
            OrderState::Placed {
                id,
                left_attempts: _,
                expire_timestamp_utc: _,
                status: _,
                activate_timestamp_utc: _,
            } => Some(*id),
            OrderState::Api {
                id,
                expire_dur: _,
                activate_dur: _,
            } => *id,
        }
    }
    pub fn expire_time_delta(&self) -> TimeDelta {
        match &self.state {
            OrderState::Placed {
                id: _,
                left_attempts: _,
                expire_timestamp_utc,
                status: _,
                activate_timestamp_utc: _,
            } => DateTime::from_timestamp_secs(*expire_timestamp_utc)
                .map(|x| x.signed_duration_since(Utc::now()))
                .unwrap_or(TimeDelta::zero()),
            OrderState::Api {
                id: _,
                expire_dur,
                activate_dur: _,
            } => expire_dur.unwrap_or_else(|| TimeDelta::days(LIMIT_ORDER_EXP_DUR_DAYS as i64)),
        }
    }
    pub fn activation_time_delta(&self) -> TimeDelta {
        match &self.state {
            OrderState::Placed {
                id: _,
                left_attempts: _,
                expire_timestamp_utc: _,
                status: _,
                activate_timestamp_utc,
            } => DateTime::from_timestamp_secs(*activate_timestamp_utc as i64)
                .map(|x| x.signed_duration_since(Utc::now()))
                .unwrap_or(TimeDelta::zero()),
            OrderState::Api {
                id: _,
                expire_dur: _,
                activate_dur,
            } => activate_dur.unwrap_or_else(TimeDelta::zero),
        }
    }
    pub fn set_expire_delta(&mut self, v: TimeDelta) {
        match &mut self.state {
            OrderState::Placed {
                id: _,
                left_attempts: _,
                expire_timestamp_utc,
                status: _,
                activate_timestamp_utc: _,
            } => *expire_timestamp_utc = (Utc::now() + v).timestamp(),
            OrderState::Api {
                id: _,
                expire_dur,
                activate_dur: _,
            } => {
                *expire_dur = Some(v);
            }
        }
    }
    pub fn set_activate_delta(&mut self, v: TimeDelta) {
        match &mut self.state {
            OrderState::Placed {
                id: _,
                left_attempts: _,
                expire_timestamp_utc: _,
                status: _,
                activate_timestamp_utc,
            } => *activate_timestamp_utc = (Utc::now() + v).timestamp() as u64,
            OrderState::Api {
                id: _,
                expire_dur: _,
                activate_dur,
            } => {
                *activate_dur = Some(v);
            }
        }
    }
    pub fn set_perc_amount(&mut self, v: UD128) {
        match &mut self.order.amount {
            SwapAmount::SellInit | SwapAmount::SellOut(_) => {
                self.order.amount = SwapAmount::SellPerc { amount: v }
            }
            SwapAmount::Buy(_) => self.order.amount = SwapAmount::BuyPerc { amount: v },

            SwapAmount::BuyPerc { amount } | SwapAmount::SellPerc { amount } => {
                *amount = v;
            }
        }
    }
    pub fn set_fixed_amount(&mut self, v: QuoteLamports) {
        match &mut self.order.amount {
            SwapAmount::SellOut(quote_lamports) | SwapAmount::Buy(quote_lamports) => {
                *quote_lamports = v;
            }
            SwapAmount::SellPerc { amount: _ } | SwapAmount::SellInit => {
                self.order.amount = SwapAmount::SellOut(v);
            }
            SwapAmount::BuyPerc { amount: _ } => {
                self.order.amount = SwapAmount::Buy(v);
            }
        }
    }
}

impl SwapAmount {
    pub fn side(&self) -> TradeType {
        match self {
            SwapAmount::Buy(_) | SwapAmount::BuyPerc { amount: _ } => TradeType::Buy,
            SwapAmount::SellPerc { amount: _ } | SwapAmount::SellOut(_) | SwapAmount::SellInit => {
                TradeType::Sell
            }
        }
    }
}

impl Target {
    pub fn direction(&self) -> Option<Direction> {
        match self {
            Target::Price {
                price: _,
                direction,
            }
            | Target::Profit {
                direction,
                init_profit_perc: _,
                recalced_profit: _,
            }
            | Target::MovingPerc {
                price_perc: _,
                local_ath: _,
                direction,
            }
            | Target::PricePerc {
                price_perc: _,
                price: _,
                direction,
            }
            | Target::Mcap {
                mcap: _,
                price: _,
                direction,
            } => Some(*direction),
            Target::Market { mode: _ } => None,
        }
    }
}

impl OrderEventTrigger {
    pub fn ui(&self) -> &'static str {
        match self {
            OrderEventTrigger::Immediate => "Inst.",
            OrderEventTrigger::Migration => "Migr.",
            OrderEventTrigger::DevBuy => "DevB.",
            OrderEventTrigger::DevSell => "DevS.",
        }
    }
}

pub enum OrderKindTrigger {
    //-100% mapped to 0-1
    Profit(UD128),
    Price(UD128),
    PricePerc(UD128),
    Mcap(UD128),
}

impl OrderKindTrigger {
    pub fn to_string_ui(&self, sol_price: UD128) -> String {
        match self {
            OrderKindTrigger::Profit(decimal) => {
                format!(
                    "🎯{:.1}% pnl",
                    (decimal.to_signed() - D128::ONE) * dec128!(100)
                )
            }
            OrderKindTrigger::Price(decimal) => {
                format!(
                    "🎯{}$ price",
                    price_formatter(*decimal * sol_price, 4, false)
                )
            }
            OrderKindTrigger::PricePerc(decimal) => {
                format!(
                    "🎯{:.1}% price",
                    (decimal.to_signed() - dec128!(1)) * dec128!(100)
                )
            }
            OrderKindTrigger::Mcap(v) => format!("🎯{}$ mcap", mc_formatter(*v)),
        }
    }
    pub fn to_value_ui(&self) -> D128 {
        match self {
            OrderKindTrigger::Profit(decimal) => decimal.to_signed(),
            OrderKindTrigger::Price(decimal) => decimal.to_signed(),
            OrderKindTrigger::PricePerc(decimal) => decimal.to_signed(),
            OrderKindTrigger::Mcap(decimal) => decimal.to_signed(),
        }
    }
}
pub enum OrderKind<'a> {
    TakeProfit {
        trigger: OrderKindTrigger,
        amount: &'a SwapAmount,
    },
    StopLoss {
        trigger: OrderKindTrigger,
        amount: &'a SwapAmount,
    },
    BuyPush {
        trigger: OrderKindTrigger,
        amount: &'a SwapAmount,
    },
    BuyDip {
        trigger: OrderKindTrigger,
        amount: &'a SwapAmount,
    },
    MarketBuy {
        mode: MarketExecuteMode,
        amount: &'a SwapAmount,
    },
    MarketSell {
        mode: MarketExecuteMode,
        amount: &'a SwapAmount,
    },
    TrailingBuyPush {
        trigger: OrderKindTrigger,
        amount: &'a SwapAmount,
    },
    TralingBuyDip {
        trigger: OrderKindTrigger,
        amount: &'a SwapAmount,
    },
    TrailingTakeProfit {
        trigger: OrderKindTrigger,
        amount: &'a SwapAmount,
    },
    TralingStopLoss {
        trigger: OrderKindTrigger,
        amount: &'a SwapAmount,
    },
}
impl<'a> OrderKind<'a> {
    pub fn amount(&'a self) -> &'a SwapAmount {
        match self {
            OrderKind::TakeProfit { trigger: _, amount }
            | OrderKind::StopLoss { trigger: _, amount }
            | OrderKind::BuyPush { trigger: _, amount }
            | OrderKind::BuyDip { trigger: _, amount }
            | OrderKind::MarketBuy { amount, mode: _ }
            | OrderKind::MarketSell { amount, mode: _ }
            | OrderKind::TrailingBuyPush { trigger: _, amount }
            | OrderKind::TralingBuyDip { trigger: _, amount }
            | OrderKind::TrailingTakeProfit { trigger: _, amount }
            | OrderKind::TralingStopLoss { trigger: _, amount } => amount,
        }
    }
    pub fn value_ui(&self) -> D128 {
        match self {
            OrderKind::TakeProfit { trigger, amount: _ }
            | OrderKind::StopLoss { trigger, amount: _ }
            | OrderKind::BuyPush { trigger, amount: _ }
            | OrderKind::BuyDip { trigger, amount: _ }
            | OrderKind::TrailingBuyPush { trigger, amount: _ }
            | OrderKind::TralingBuyDip { trigger, amount: _ }
            | OrderKind::TrailingTakeProfit { trigger, amount: _ }
            | OrderKind::TralingStopLoss { trigger, amount: _ } => trigger.to_value_ui(),
            OrderKind::MarketBuy { amount: _, mode: _ } => D128::MAX,
            OrderKind::MarketSell { amount: _, mode: _ } => D128::MAX - D128::ONE,
        }
    }
    pub fn name_ui(&self) -> String {
        let res = match self {
            OrderKind::TakeProfit {
                trigger: _,
                amount: _,
            } => "TakeProfit",
            OrderKind::StopLoss {
                trigger: _,
                amount: _,
            } => "StopLoss",
            OrderKind::BuyPush {
                trigger: _,
                amount: _,
            } => "BuyPush",
            OrderKind::BuyDip {
                trigger: _,
                amount: _,
            } => "BuyDip",
            OrderKind::MarketBuy { amount: _, mode } => match mode {
                MarketExecuteMode::Always => "Buy MARKET",
                MarketExecuteMode::OnlyInProfit => "Buy MARKET in Profit",
                MarketExecuteMode::OnlyInLoss => "Buy MARKET in Loss",
            },
            OrderKind::MarketSell { amount: _, mode } => match mode {
                MarketExecuteMode::Always => "Sell MARKET",
                MarketExecuteMode::OnlyInProfit => "Sell MARKET in Profit",
                MarketExecuteMode::OnlyInLoss => "Sell MARKET in Loss",
            },
            OrderKind::TrailingBuyPush {
                trigger: _,
                amount: _,
            } => "Trail.BuyPush",
            OrderKind::TralingBuyDip {
                trigger: _,
                amount: _,
            } => "Trail.BuyDip",
            OrderKind::TrailingTakeProfit {
                trigger: _,
                amount: _,
            } => "⚠️Trail.TP",
            OrderKind::TralingStopLoss {
                trigger: _,
                amount: _,
            } => "Trail.SL",
        };
        res.to_string()
    }
}

impl SwapAmount {
    pub fn to_string_ui(&self) -> String {
        match self {
            SwapAmount::SellOut(v) | SwapAmount::Buy(v) => match v.quote_kind() {
                decisol::QuoteKind::Usd => format!("💰{:.2} $", v),
                decisol::QuoteKind::Sol => format!("💰{:.2} SOL", v),
            },
            SwapAmount::SellPerc { amount } | SwapAmount::BuyPerc { amount } => {
                format!("💰{:.1}%", *amount * udec128!(100))
            }
            SwapAmount::SellInit => "💰INIT".to_string(),
        }
    }
}
impl RawOrder {
    pub const fn is_market_execute_imm(&self) -> bool {
        if let Target::Market { mode } = &self.target {
            match mode {
                MarketExecuteMode::Always => true,
                MarketExecuteMode::OnlyInProfit => false,
                MarketExecuteMode::OnlyInLoss => false,
            }
        } else {
            false
        }
    }
    pub fn value_ui_inner(
        &self,
        sol_price: UD128,
        trigger: OrderEventTrigger,
        delay: TimeDelta,
    ) -> String {
        let kind = self.kind();
        let placement = trigger.ui();
        let name = kind.name_ui();
        let amount = kind.amount().to_string_ui();

        let delay = if delay <= TimeDelta::zero() {
            "".to_string()
        } else {
            format!("|🕰️{}", duration_formatter(&delay))
        };

        match &kind {
            OrderKind::TakeProfit { trigger, amount: _ }
            | OrderKind::StopLoss { trigger, amount: _ }
            | OrderKind::BuyPush { trigger, amount: _ }
            | OrderKind::BuyDip { trigger, amount: _ }
            | OrderKind::TrailingBuyPush { trigger, amount: _ }
            | OrderKind::TralingBuyDip { trigger, amount: _ }
            | OrderKind::TrailingTakeProfit { trigger, amount: _ }
            | OrderKind::TralingStopLoss { trigger, amount: _ } => format!(
                "{placement}{}| {name} | {amount} | {}",
                delay,
                trigger.to_string_ui(sol_price)
            ),
            OrderKind::MarketSell { amount: _, mode: _ }
            | OrderKind::MarketBuy { amount: _, mode: _ } => {
                format!("{placement}{}| {name} {amount}", delay)
            }
        }
    }
    pub fn side(&self) -> TradeType {
        self.amount.side()
    }
    pub fn kind<'a>(&'a self) -> OrderKind<'a> {
        match &self.target {
            Target::Market { mode } => match self.side() {
                TradeType::Buy => OrderKind::MarketBuy {
                    amount: &self.amount,
                    mode: mode.clone(),
                },
                TradeType::Sell => OrderKind::MarketSell {
                    amount: &self.amount,
                    mode: mode.clone(),
                },
            },
            Target::Price { price, direction } => {
                let trigger = OrderKindTrigger::Price(*price);
                self.kind_from_direction_side(*direction, trigger)
            }
            Target::Mcap {
                mcap,
                price: _,
                direction,
            } => {
                let trigger = OrderKindTrigger::Mcap(*mcap);
                self.kind_from_direction_side(*direction, trigger)
            }
            Target::PricePerc {
                price_perc,
                price,
                direction,
            } => {
                let trigger = match price {
                    Some(v) => OrderKindTrigger::Price(*v),
                    None => OrderKindTrigger::PricePerc(*price_perc),
                };
                self.kind_from_direction_side(*direction, trigger)
            }
            Target::Profit {
                direction,
                init_profit_perc,
                recalced_profit,
            } => {
                let profit_perc = recalced_profit.as_ref().unwrap_or(init_profit_perc);
                let trigger = OrderKindTrigger::Profit(*profit_perc);
                self.kind_from_direction_side(*direction, trigger)
            }
            Target::MovingPerc {
                price_perc,
                local_ath,
                direction,
            } => {
                let trigger = match local_ath {
                    Some(v) => OrderKindTrigger::Price(*price_perc * *v),
                    None => OrderKindTrigger::PricePerc(*price_perc),
                };
                match direction {
                    Direction::Above => match self.side() {
                        TradeType::Buy => OrderKind::TrailingBuyPush {
                            trigger,
                            amount: &self.amount,
                        },
                        TradeType::Sell => OrderKind::TrailingTakeProfit {
                            trigger,
                            amount: &self.amount,
                        },
                    },
                    Direction::Below => match self.side() {
                        TradeType::Buy => OrderKind::TralingBuyDip {
                            trigger,
                            amount: &self.amount,
                        },
                        TradeType::Sell => OrderKind::TralingStopLoss {
                            trigger,
                            amount: &self.amount,
                        },
                    },
                }
            }
        }
    }
    fn kind_from_direction_side(
        &self,
        direction: Direction,
        trigger: OrderKindTrigger,
    ) -> OrderKind<'_> {
        match direction {
            Direction::Above => match self.side() {
                TradeType::Buy => OrderKind::BuyPush {
                    trigger,
                    amount: &self.amount,
                },
                TradeType::Sell => OrderKind::TakeProfit {
                    trigger,
                    amount: &self.amount,
                },
            },
            Direction::Below => match self.side() {
                TradeType::Buy => OrderKind::BuyDip {
                    trigger,
                    amount: &self.amount,
                },
                TradeType::Sell => OrderKind::StopLoss {
                    trigger,
                    amount: &self.amount,
                },
            },
        }
    }
}

#[derive(Default)]
pub struct MarketOrdersDeduper {
    dev_buy: MarketOrdersDeduperKind,
    dev_sell: MarketOrdersDeduperKind,
    migration: MarketOrdersDeduperKind,
    imm: MarketOrdersDeduperKind,
}
impl MarketOrdersDeduper {
    pub fn read(mut self, orders: &[ApiLimitOrder]) -> Self {
        for ord in orders {
            match ord.trigger {
                OrderEventTrigger::Immediate => self.imm.read(ord),
                OrderEventTrigger::Migration => self.migration.read(ord),
                OrderEventTrigger::DevBuy => self.dev_buy.read(ord),
                OrderEventTrigger::DevSell => self.dev_sell.read(ord),
            }
        }
        self
    }
    pub fn dedup(&mut self, ord: &ApiLimitOrder) -> bool {
        match ord.trigger {
            OrderEventTrigger::Immediate => self.imm.dedup(ord),
            OrderEventTrigger::Migration => self.migration.dedup(ord),
            OrderEventTrigger::DevBuy => self.dev_buy.dedup(ord),
            OrderEventTrigger::DevSell => self.dev_sell.dedup(ord),
        }
    }
}

#[derive(Default)]
struct MarketOrdersDeduperKind {
    always: MarketKind,
    in_profit: MarketKind,
    in_loss: MarketKind,
}

#[derive(Default)]
struct MarketKind {
    buy: bool,
    sell: bool,
}

impl MarketOrdersDeduperKind {
    fn read(&mut self, ord: &ApiLimitOrder) {
        let kind = if let Target::Market { mode } = &ord.order.target {
            match mode {
                MarketExecuteMode::Always => &mut self.always,
                MarketExecuteMode::OnlyInProfit => &mut self.in_profit,
                MarketExecuteMode::OnlyInLoss => &mut self.in_loss,
            }
        } else {
            return;
        };

        let seen = match ord.order.side() {
            TradeType::Buy => &mut kind.buy,
            TradeType::Sell => &mut kind.sell,
        };
        *seen = true;
    }
    fn dedup(&mut self, ord: &ApiLimitOrder) -> bool {
        let kind = if let Target::Market { mode } = &ord.order.target {
            match mode {
                MarketExecuteMode::Always => &mut self.always,
                MarketExecuteMode::OnlyInProfit => &mut self.in_profit,
                MarketExecuteMode::OnlyInLoss => &mut self.in_loss,
            }
        } else {
            return false;
        };

        let seen = match ord.order.side() {
            TradeType::Buy => &mut kind.buy,
            TradeType::Sell => &mut kind.sell,
        };
        if *seen {
            true
        } else {
            *seen = true;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use proto_rs::{ProtoDecoder, bytes::Bytes};

    use crate::types::{ApiLimitOrder, UpdateTokenLimitOrders};

    #[test]
    fn test_round_trip_order() {
        let encoded = vec![
            10u8, 5, 8, 199, 241, 133, 103, 18, 8, 18, 6, 18, 4, 8, 128, 245, 36, 26, 52, 10, 4, 8,
            2, 24, 1, 18, 5, 8, 192, 150, 177, 2, 26, 5, 8, 128, 173, 226, 4, 34, 8, 34, 6, 10, 4,
            8, 13, 24, 1, 42, 8, 26, 6, 10, 4, 8, 5, 24, 1, 50, 8, 8, 1, 24, 1, 40, 1, 64, 1, 64,
            10, 42, 34, 10, 32, 116, 74, 188, 129, 166, 216, 118, 242, 40, 80, 239, 197, 22, 132,
            53, 158, 32, 120, 224, 28, 20, 154, 215, 197, 237, 21, 63, 175, 227, 38, 78, 31,
        ];
        let decoded = ApiLimitOrder::decode(Bytes::from(encoded), Default::default()).unwrap();
        println!("Decoded: {decoded:?}")
    }
    #[test]
    fn test_round_trip_update_orders() {
        let encoded = vec![
            10u8, 34, 10, 32, 198, 250, 122, 243, 190, 219, 173, 58, 61, 101, 243, 106, 171, 201,
            116, 49, 177, 187, 228, 194, 210, 246, 224, 228, 124, 166, 2, 3, 69, 47, 93, 97, 18,
            107, 10, 5, 8, 199, 241, 133, 103, 18, 8, 18, 6, 18, 4, 8, 128, 245, 36, 26, 52, 10, 4,
            8, 2, 24, 1, 18, 5, 8, 192, 150, 177, 2, 26, 5, 8, 128, 173, 226, 4, 34, 8, 34, 6, 10,
            4, 8, 13, 24, 1, 42, 8, 26, 6, 10, 4, 8, 5, 24, 1, 50, 8, 8, 1, 24, 1, 40, 1, 64, 1,
            64, 10, 42, 34, 10, 32, 116, 74, 188, 129, 166, 216, 118, 242, 40, 80, 239, 197, 22,
            132, 53, 158, 32, 120, 224, 28, 20, 154, 215, 197, 237, 21, 63, 175, 227, 38, 78, 31,
        ];
        let decoded =
            UpdateTokenLimitOrders::decode(Bytes::from(encoded), Default::default()).unwrap();
        println!("Decoded: {decoded:?}")
    }
}
