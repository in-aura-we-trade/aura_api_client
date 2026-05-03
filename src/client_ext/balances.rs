use crate::types::WalletBalances;
use crate::types::WalletUtilAccountsInfo;
use decisol::Decisol;
use decisol::Lamports;
use decisol::Usd1;
use decisol::Usdc;
use decisol::Usdt;
use decisol::Wsol;
use fastnum::UD128;

impl WalletUtilAccountsInfo {
    pub fn is_util_accs_created(&self) -> bool {
        self.util_accs.pump_amm_uva && self.util_accs.custom_nonce && self.util_accs.pump_uva
        //&& self.util_accs.pump_amm_uva_ata
    }
}

impl WalletBalances {
    pub fn total_usd(&self, sol_price: UD128) -> UD128 {
        UD128::from(self.sol() + self.wsol().amount()) * sol_price
            + UD128::from(self.usdc() + self.usd1().amount() + self.usdt().amount())
    }
    pub fn sol(&self) -> Lamports {
        self.sol.balance.into()
    }
    pub fn usdc(&self) -> Usdc {
        (self.usdc_ata.balance + self.usdc_pda.balance).into()
    }
    pub fn usdc_ata(&self) -> Usdc {
        self.usdc_ata.balance.into()
    }
    pub fn usdc_pda(&self) -> Usdc {
        self.usdc_pda.balance.into()
    }
    pub fn usd1(&self) -> Usd1 {
        (self.usd1_ata.balance + self.usd1_pda.balance).into()
    }
    pub fn usd1_ata(&self) -> Usd1 {
        self.usd1_ata.balance.into()
    }
    pub fn usd1_pda(&self) -> Usd1 {
        self.usd1_pda.balance.into()
    }
    pub fn usdt(&self) -> Usdt {
        (self.usdt_ata.balance + self.usdt_pda.balance).into()
    }
    pub fn usdt_ata(&self) -> Usdt {
        self.usdt_ata.balance.into()
    }
    pub fn usdt_pda(&self) -> Usdt {
        self.usdt_pda.balance.into()
    }
    pub fn wsol(&self) -> Wsol {
        (self.wsol_ata.balance + self.wsol_pda.balance).into()
    }
    pub fn wsol_ata(&self) -> Wsol {
        self.wsol_ata.balance.into()
    }
    pub fn wsol_pda(&self) -> Wsol {
        self.wsol_pda.balance.into()
    }
}
