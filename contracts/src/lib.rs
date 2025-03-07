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

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod access;
mod finance;
mod governance;
mod security;
mod token;
mod upgradeability;

pub mod traits;

// Modules with implementation of traits above
#[cfg(feature = "access_control")]
pub use access::access_control;
#[cfg(feature = "ownable")]
pub use access::ownable;
#[cfg(feature = "payment_splitter")]
pub use finance::payment_splitter;
#[cfg(feature = "timelock_controller")]
pub use governance::timelock_controller;
#[cfg(feature = "pausable")]
pub use security::pausable;
#[cfg(feature = "reentrancy_guard")]
pub use security::reentrancy_guard;
#[cfg(feature = "psp22")]
pub use token::psp22;
#[cfg(feature = "psp22_pallet")]
pub use token::psp22_pallet;
#[cfg(feature = "psp34")]
pub use token::psp34;
#[cfg(feature = "psp37")]
pub use token::psp37;
#[cfg(feature = "psp55")]
pub use token::psp55;
#[cfg(feature = "diamond")]
pub use upgradeability::diamond;
#[cfg(feature = "proxy")]
pub use upgradeability::proxy;
