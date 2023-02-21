use cosmwasm_std::Decimal;
use nico_teen::fee::{Fee, VaultFee};

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
