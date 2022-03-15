use std::{io::Write, fs::OpenOptions}; // bring trait into scope
use std::fs;


use borsh::{BorshSerialize, BorshDeserialize};
use spl_token::*;

/// Account state.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum AccountState {
    /// Account is not yet initialized
    Uninitialized,
    /// Account is initialized; the account owner and/or delegate may perform permitted operations
    /// on this account
    Initialized,
    /// Account has been frozen by the mint freeze authority. Neither the account owner nor
    /// the delegate are able to perform operations on this account.
    Frozen,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, PartialEq)]
pub struct Account {
    pub state: AccountState,
    pub state2: AccountState

}

impl Default for AccountState {
    fn default() -> Self {
        AccountState::Uninitialized
    }
}

fn main() {
    let state = Account{
        state: AccountState::Uninitialized,
        state2: AccountState::Initialized
    };
    let serialized = state.try_to_vec().unwrap();

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    // either use ? or unwrap since it returns a Result
    .open("file.borsh").unwrap();

    file.write_all(&serialized).unwrap();
}
