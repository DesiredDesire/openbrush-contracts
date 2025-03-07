// author: Konrad Wierzbik <konrad.wierzbik@gmail.com>
// strongly based on Supercolony Openbrush PSP22

// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    psp55,
    psp55::Internal as _,
    traits::psp55::*,
};
pub use psp55::{
    Internal as _,
    Transfer as _,
};

use ink_env::{
    CallFlags,
    Error as EnvError,
};
use ink_prelude::vec::Vec;
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Storage,
        String,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub supply: Balance,
    pub balances: Mapping<AccountId, Balance>,
    pub allowances: Mapping<(AccountId, AccountId), Balance, AllowancesKey>,
    pub _reserved: Option<()>,
}

pub struct AllowancesKey;

impl<'a> TypeGuard<'a> for AllowancesKey {
    type Type = &'a (&'a AccountId, &'a AccountId);
}

impl<T: Storage<Data>> PSP55 for T {
    default fn total_supply(&self) -> Balance {
        self.data().supply.clone()
    }

    default fn balance_of(&self, owner: AccountId) -> Balance {
        self._balance_of(&owner)
    }

    default fn allowance(&self, spender: AccountId, owner: AccountId) -> Balance {
        self._allowance(&spender, &owner)
    }

    default fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP55Error> {
        let from = Self::env().caller();
        let allowance = self._allowance(&from, &to);
        if allowance < value {
            return Err(PSP55Error::InsufficientAllowance)
        }

        self._transfer_from_to(from, to, value, data)?;
        Ok(())
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP55Error> {
        let caller = Self::env().caller();
        let allowance = self._allowance(&caller, &to);

        if allowance < value {
            return Err(PSP55Error::InsufficientAllowance)
        }

        self._approve_from_to(caller, to, allowance - value)?;
        self._transfer_from_to(from, to, value, data)?;
        Ok(())
    }

    default fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP55Error> {
        let owner = Self::env().caller();
        self._approve_from_to(spender, owner, value)?;
        Ok(())
    }

    default fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP55Error> {
        let owner = Self::env().caller();
        self._approve_from_to(spender, owner, self._allowance(&spender, &owner) + delta_value)
    }

    default fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP55Error> {
        let owner = Self::env().caller();
        let allowance = self._allowance(&spender, &owner);

        if allowance < delta_value {
            return Err(PSP55Error::InsufficientAllowance)
        }

        self._approve_from_to(spender, owner, allowance - delta_value)
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance);
    fn _emit_approval_event(&self, _spender: AccountId, _owner: AccountId, _amount: Balance);

    fn _balance_of(&self, owner: &AccountId) -> Balance;
    fn _allowance(&self, spender: &AccountId, owner: &AccountId) -> Balance;

    fn _do_safe_transfer_check(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        value: &Balance,
        data: &Vec<u8>,
    ) -> Result<(), PSP55Error>;

    fn _transfer_from_to(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP55Error>;

    fn _approve_from_to(&mut self, spender: AccountId, owner: AccountId, amount: Balance) -> Result<(), PSP55Error>;

    fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP55Error>;

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP55Error>;
}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}
    default fn _emit_approval_event(&self, _spender: AccountId, _owner: AccountId, _amount: Balance) {}

    default fn _balance_of(&self, owner: &AccountId) -> Balance {
        self.data().balances.get(owner).unwrap_or(0)
    }

    default fn _allowance(&self, spender: &AccountId, owner: &AccountId) -> Balance {
        self.data().allowances.get(&(spender, owner)).unwrap_or(0)
    }

    default fn _do_safe_transfer_check(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        value: &Balance,
        data: &Vec<u8>,
    ) -> Result<(), PSP55Error> {
        self.flush();
        let builder = PSP55ReceiverRef::before_received_builder(
            to,
            Self::env().caller(),
            from.clone(),
            value.clone(),
            data.clone(),
        )
        .call_flags(CallFlags::default().set_allow_reentry(true));
        let result = match builder.fire() {
            Ok(result) => {
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => {
                match e {
                    // `NotCallable` means that the receiver is not a contract.

                    // `CalleeTrapped` means that the receiver has no method called `before_received` or it failed inside.
                    // First case is expected. Second - not. But we can't tell them apart so it is a positive case for now.
                    // https://github.com/paritytech/ink/issues/1002
                    EnvError::NotCallable | EnvError::CalleeTrapped => Ok(()),
                    _ => {
                        Err(PSP55Error::SafeTransferCheckFailed(String::from(
                            "Error during call to receiver",
                        )))
                    }
                }
            }
        };
        self.load();
        result?;
        Ok(())
    }

    default fn _transfer_from_to(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP55Error> {
        if from.is_zero() {
            return Err(PSP55Error::ZeroSenderAddress)
        }
        if to.is_zero() {
            return Err(PSP55Error::ZeroRecipientAddress)
        }

        let from_balance = self._balance_of(&from);

        if from_balance < amount {
            return Err(PSP55Error::InsufficientBalance)
        }

        self._before_token_transfer(Some(&from), Some(&to), &amount)?;

        self.data().balances.insert(&from, &(from_balance - amount));

        self._do_safe_transfer_check(&from, &to, &amount, &data)?;

        let to_balance = self._balance_of(&to);
        self.data().balances.insert(&to, &(to_balance + amount));

        self._after_token_transfer(Some(&from), Some(&to), &amount)?;
        self._emit_transfer_event(Some(from), Some(to), amount);

        Ok(())
    }

    default fn _approve_from_to(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
    ) -> Result<(), PSP55Error> {
        if owner.is_zero() {
            return Err(PSP55Error::ZeroSenderAddress)
        }
        if spender.is_zero() {
            return Err(PSP55Error::ZeroRecipientAddress)
        }

        self.data().allowances.insert(&(&spender, &owner), &amount);
        self._emit_approval_event(spender, owner, amount);
        Ok(())
    }

    default fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP55Error> {
        if account.is_zero() {
            return Err(PSP55Error::ZeroRecipientAddress)
        }

        self._before_token_transfer(None, Some(&account), &amount)?;
        let mut new_balance = self._balance_of(&account);
        new_balance += amount;
        self.data().balances.insert(&account, &new_balance);
        self.data().supply += amount;
        self._after_token_transfer(None, Some(&account), &amount)?;
        self._emit_transfer_event(None, Some(account), amount);

        Ok(())
    }

    default fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP55Error> {
        if account.is_zero() {
            return Err(PSP55Error::ZeroRecipientAddress)
        }

        let mut from_balance = self._balance_of(&account);

        if from_balance < amount {
            return Err(PSP55Error::InsufficientBalance)
        }

        self._before_token_transfer(Some(&account), None, &amount)?;

        from_balance -= amount;
        self.data().balances.insert(&account, &from_balance);
        self.data().supply -= amount;
        self._after_token_transfer(Some(&account), None, &amount)?;
        self._emit_transfer_event(Some(account), None, amount);

        Ok(())
    }
}

pub trait Transfer {
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP55Error>;

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP55Error>;
}

impl<T: Storage<Data>> Transfer for T {
    default fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP55Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP55Error> {
        Ok(())
    }
}
