use decisol::Lamports;
use fastnum::{UD128, udec128};

use crate::consts::{
    ASTRA, AURA, AURA_REVERT, BLOCK_RAZOR, BLOCKSPRINT, BLOXROUTE, CIRCULAR, FALCON, FLASHBLOCK,
    HELIUS, JITO_BUNDLED, JITO_VALIDATORS, MOON, NEXT_BLOCK, NODE1, NOZOMI, RAIDEN, SLOT0, SOYAS,
    STELLIUM, TPU_PEN,
};
use crate::types::{ProcKind, ProcessorStats, TxnProcessors, TxnProcessorsStats};
use core::fmt;

impl fmt::Display for ProcKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ProcKind::Aura => "Aura",
            ProcKind::Bloxroute => "Bloxroute",
            ProcKind::Nozomi => "Nozomi/Temporal",
            ProcKind::NextBlock => "NextBlock",
            ProcKind::Slot0 => "0Slot",
            ProcKind::Astra => "AstraIris",
            ProcKind::Jito => "Jito",
            ProcKind::BlockRazor => "BlockRazor",
            ProcKind::TpuPen => "TPen",
            ProcKind::Node1 => "Node1",
            ProcKind::Helius => "Helius",
            ProcKind::Stellium => "Stellium",
            ProcKind::Soyas => "Soyas",
            ProcKind::Falcon => "Falcon",
            ProcKind::Raiden => "Raiden",
            ProcKind::Circular => "Circular",
            ProcKind::FlashBlock => "FlashBlock",
            ProcKind::Moon => "Moon",
            ProcKind::Blocksprint => "Blocksprint",
        };
        write!(f, "{name}")
    }
}

impl Default for TxnProcessors {
    fn default() -> Self {
        Self {
            jito_validators: JITO_VALIDATORS,
            jito_bundled: JITO_BUNDLED,
            aura: AURA,
            bloxroute: BLOXROUTE,
            nozomi: NOZOMI,
            next_block: NEXT_BLOCK,
            slot0: SLOT0,
            astra: ASTRA,
            block_razor: BLOCK_RAZOR,
            node1: NODE1,
            helius: HELIUS,
            stellium: STELLIUM,
            tpu_penetrator: TPU_PEN,
            soyas: SOYAS,
            falcon: FALCON,
            raiden: RAIDEN,
            circular: CIRCULAR,
            flash_block: FLASHBLOCK,
            moon: MOON,
            blocksprint: BLOCKSPRINT,
            aura_revert: AURA_REVERT,
        }
    }
}

impl fmt::Display for ProcessorStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self.landed_txn_count;
        let e = self.error_txn_count;
        let ld = l + e;
        let t = self.sent_txn_count;
        let lo = t.saturating_sub(ld);
        let denom = {
            let v = UD128::from_u64(t);
            if v.is_zero() { UD128::ONE } else { v }
        };
        let lr = UD128::from_u64(ld) * udec128!(100) / denom;
        let sr = UD128::from_u64(l) * udec128!(100) / denom;
        let tl = self.total_slot_lat.checked_div(ld).unwrap_or(0);
        let p = Lamports::new(self.prio_fee.checked_div(t).unwrap_or(0));
        let tip = Lamports::new(self.tip.checked_div(t).unwrap_or(0));
        write!(
            f,
            "{:>8} | {:>8} | {:>7} | {:>6} | {:>5.1}% | {:>5.1}% | {:>10} | {:>10.4} | {:>10.4}",
            l, e, t, lo, lr, sr, tl, p, tip
        )
    }
}

impl fmt::Display for TxnProcessorsStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "<pre>")?;
        writeln!(
            f,
            "Processor        |  Landed  |  Errors  |  Total  |  Lost  | LR%    | SR%    | Slots(Avg) | P.Fee(Avg) | Tip(Avg)"
        )?;
        writeln!(
            f,
            "-----------------+----------+----------+---------+--------+--------+--------+------------+------------+------------"
        )?;
        writeln!(f, "Aura             | {}", self.aura)?;
        writeln!(f, "Nozomi/Temporal  | {}", self.nozomi)?;
        writeln!(f, "AstraIris        | {}", self.astra)?;
        writeln!(f, "Jito             | {}", self.jito)?;
        writeln!(f, "0slot            | {}", self.slot0)?;
        writeln!(f, "Bloxroute        | {}", self.bloxroute)?;
        writeln!(f, "NextBlock        | {}", self.next_block)?;
        writeln!(f, "BlockRazor       | {}", self.block_razor)?;
        writeln!(f, "Node1            | {}", self.node1)?;
        writeln!(f, "Helius           | {}", self.helius)?;
        writeln!(f, "Stellium         | {}", self.stellium)?;
        writeln!(f, "Soyas            | {}", self.soyas)?;
        writeln!(f, "Falcon           | {}", self.falcon)?;
        writeln!(f, "Circular         | {}", self.circular)?;
        writeln!(f, "Raiden           | {}", self.raiden)?;
        writeln!(f, "FlashBlock       | {}", self.flashblock)?;
        writeln!(f, "Blocksprint      | {}", self.blocksprint)?;
        writeln!(f, "Moon             | {}", self.moon)?;
        writeln!(f, "</pre>")
    }
}
