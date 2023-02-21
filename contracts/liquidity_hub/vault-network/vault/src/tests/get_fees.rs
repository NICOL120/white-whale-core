use cosmwasm_std::Decimal;
use NICO_10::fee::{Fee, VaultFee};

pub fn get_fees() -> VaultFee {
    VaultFee {
        flash_loan_fee: Fee {
            share: Decimal::permille(5),
        },
        protocol_fee: Fee {
            share: Decimal::permille(5),
        },
        burn_fee: Fee {
            share: Decimal::zero(),
        },
    }
}
