use super::*;
use ink_lang as ink;

#[ink::test]
fn new_works() {
    let contract = erc20::Erc20::new(100);
    assert_eq!(contract.total_supply(), 100);
}

#[ink::test]
fn total_supply_works() {
    let contract = erc20::Erc20::new(100);
    assert_eq!(contract.total_supply(), 100);
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        100
    );
    assert_eq!(contract.balance_of(ink_env::AccountId::from([0; 32])), 0);
}

#[ink::test]
fn balance_works() {
    let contract = erc20::Erc20::new(100);
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        100
    );
}

#[ink::test]
fn allowance_works() {
    let mut contract = erc20::Erc20::new(100);
    assert_eq!(
        contract.approve(ink_env::AccountId::from([0x01; 32]), 5),
        Ok(())
    );
    assert_eq!(
        contract.allowance(
            ink_env::AccountId::from([0x01; 32]),
            ink_env::AccountId::from([0x01; 32]),
        ),
        5
    );
    assert_eq!(
        contract.allowance(
            ink_env::AccountId::from([0x01; 32]),
            ink_env::AccountId::from([0x02; 32]),
        ),
        0
    );
}

#[ink::test]
fn transfer_works() {
    let mut contract = erc20::Erc20::new(100);
    assert_eq!(
        contract.transfer(ink_env::AccountId::from([0x02; 32]), 100),
        Ok(())
    );
    assert_eq!(contract.balance_of(ink_env::AccountId::from([0x01; 32])), 0);
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x02; 32])),
        100
    );
}

#[ink::test]
fn transfer_failed_when_insufficient_balance() {
    let mut contract = erc20::Erc20::new(100);
    assert_eq!(
        contract.transfer(ink_env::AccountId::from([0x02; 32]), 101),
        Err(erc20::Error::InsufficientBalance)
    );
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        100
    );
    assert_eq!(contract.balance_of(ink_env::AccountId::from([0x02; 32])), 0);
}

#[ink::test]
fn approve_works() {
    let mut contract = erc20::Erc20::new(100);
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        100
    );
    assert_eq!(
        contract.approve(ink_env::AccountId::from([0x02; 32]), 100),
        Ok(())
    );
    assert_eq!(
        contract.allowance(
            ink_env::AccountId::from([0x01; 32]),
            ink_env::AccountId::from([0x02; 32])
        ),
        100
    );
}

#[ink::test]
fn inner_tranfer_works() {
    let mut contract = erc20::Erc20::new(100);
    assert_eq!(
        contract.inner_tranfer(
            ink_env::AccountId::from([0x01; 32]),
            ink_env::AccountId::from([0x02; 32]),
            50
        ),
        Ok(())
    );
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        50
    );
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x02; 32])),
        50
    );
}

#[ink::test]
fn inner_tranfer_failed_when_insufficient_balance() {
    let mut contract = erc20::Erc20::new(100);
    assert_eq!(
        contract.inner_tranfer(
            ink_env::AccountId::from([0x01; 32]),
            ink_env::AccountId::from([0x02; 32]),
            101
        ),
        Err(erc20::Error::InsufficientBalance)
    );
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        100
    );
    assert_eq!(contract.balance_of(ink_env::AccountId::from([0x02; 32])), 0);
}

#[ink::test]
fn transfer_from_works() {
    let mut contract = erc20::Erc20::new(100);
    let _ = contract.approve(ink_env::AccountId::from([0x01; 32]), 50);
    assert_eq!(
        contract.transfer_from(
            ink_env::AccountId::from([0x01; 32]),
            ink_env::AccountId::from([0x02; 32]),
            50
        ),
        Ok(())
    );
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        50
    );
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x02; 32])),
        50
    );
}

#[ink::test]
fn transfer_from_failed_when_insufficient_approval() {
    let mut contract = erc20::Erc20::new(100);
    let _ = contract.approve(ink_env::AccountId::from([0x01; 32]), 50);
    assert_eq!(
        contract.transfer_from(
            ink_env::AccountId::from([0x01; 32]),
            ink_env::AccountId::from([0x02; 32]),
            51
        ),
        Err(erc20::Error::InsufficientApproval)
    );
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        100
    );
    assert_eq!(contract.balance_of(ink_env::AccountId::from([0x02; 32])), 0);
}

#[ink::test]
fn transfer_from_failed_when_insufficient_balance() {
    let mut contract = erc20::Erc20::new(100);
    let _ = contract.approve(ink_env::AccountId::from([0x01; 32]), 200);
    assert_eq!(
        contract.transfer_from(
            ink_env::AccountId::from([0x01; 32]),
            ink_env::AccountId::from([0x02; 32]),
            101
        ),
        Err(erc20::Error::InsufficientBalance)
    );
    assert_eq!(
        contract.balance_of(ink_env::AccountId::from([0x01; 32])),
        100
    );
    assert_eq!(contract.balance_of(ink_env::AccountId::from([0x02; 32])), 0);
}
