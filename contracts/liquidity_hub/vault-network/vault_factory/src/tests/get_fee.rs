use cosmwasm_std::Decimal;
use nico_teen::fee::{Fee, VaultFee};

pub fn get_fees() -> VaultFee {
    VaultFee {
        flash_loan_fee: Fee {
            share: Decimal::from_ratio(100u128, 3000u128),
        },
        protocol_fee: Fee {
            share: Decimal::from_ratio(100u128, 3000u128),
        },
        burn_fee: Fee {
            share: Decimal::zero(),
        },
    }
}
