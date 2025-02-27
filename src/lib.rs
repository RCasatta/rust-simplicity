// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

#![allow(
    // we use `bool` to represent bits and frequentely assert_eq against them
    clippy::bool_assert_comparison,
    // we use () as the environment for Core (FIXME we should probabl use a newtype)
    clippy::let_unit_value
)]

#[cfg(feature = "serde")]
pub extern crate actual_serde as serde;
#[cfg(feature = "bitcoin")]
pub extern crate bitcoin;
#[cfg(feature = "elements")]
pub extern crate elements;

/// Re-export of byteorder crate
pub extern crate byteorder;
/// Re-export of hashes crate
pub extern crate hashes;
/// Re-export of hex crate
pub extern crate hex;

#[macro_use]
mod macros;

mod analysis;
mod bit_encoding;
pub mod bit_machine;
pub mod dag;
pub mod human_encoding;
pub mod jet;
mod merkle;
pub mod node;
#[cfg(feature = "elements")]
pub mod policy;
pub mod types;
mod value;

pub use bit_encoding::decode;
pub use bit_encoding::encode;
pub use bit_encoding::BitWriter;
pub use bit_encoding::{BitIter, EarlyEndOfStreamError};

#[cfg(feature = "elements")]
pub use crate::policy::{
    sighash, Policy, Preimage32, Satisfier, SimplicityKey, ToXOnlyPubkey, Translator,
};

pub use crate::bit_machine::BitMachine;
pub use crate::encode::{encode_natural, encode_value, encode_witness};
pub use crate::merkle::{
    amr::Amr,
    cmr::Cmr,
    imr::{FirstPassImr, Imr},
    tmr::Tmr,
    FailEntropy,
};
pub use crate::node::{CommitNode, ConstructNode, RedeemNode, WitnessNode};
pub use crate::value::Value;
pub use simplicity_sys as ffi;
use std::fmt;

/// Return the version of Simplicity leaves inside a tap tree.
#[cfg(feature = "elements")]
pub fn leaf_version() -> elements::taproot::LeafVersion {
    elements::taproot::LeafVersion::from_u8(0xbe).expect("constant leaf version")
}

/// Error type for simplicity
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Decoder error
    Decode(crate::decode::Error),
    /// A disconnect node was populated at commitment time
    DisconnectCommitTime,
    /// A disconnect node was *not* populated at redeem time
    DisconnectRedeemTime,
    /// Type-checking error
    Type(crate::types::Error),
    /// Witness iterator ended early
    NoMoreWitnesses,
    /// Finalization failed; did not have enough witness data to satisfy program.
    IncompleteFinalization,
    /// Witness has different length than defined in its preamble
    InconsistentWitnessLength,
    /// Tried to parse a jet but the name wasn't recognized
    InvalidJetName(String),
    /// Policy error
    #[cfg(feature = "elements")]
    Policy(policy::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Decode(ref e) => fmt::Display::fmt(e, f),
            Error::DisconnectCommitTime => {
                f.write_str("disconnect node had two children (commit time); must have one")
            }
            Error::DisconnectRedeemTime => {
                f.write_str("disconnect node had one child (redeem time); must have two")
            }
            Error::Type(ref e) => fmt::Display::fmt(e, f),
            Error::IncompleteFinalization => f.write_str("unable to satisfy program"),
            Error::InconsistentWitnessLength => {
                f.write_str("witness has different length than defined in its preamble")
            }
            Error::InvalidJetName(s) => write!(f, "unknown jet `{}`", s),
            Error::NoMoreWitnesses => f.write_str("no more witness data available"),
            #[cfg(feature = "elements")]
            Error::Policy(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Decode(ref e) => Some(e),
            Error::DisconnectCommitTime => None,
            Error::DisconnectRedeemTime => None,
            Error::Type(ref e) => Some(e),
            Error::NoMoreWitnesses => None,
            Error::IncompleteFinalization => None,
            Error::InconsistentWitnessLength => None,
            Error::InvalidJetName(..) => None,
            #[cfg(feature = "elements")]
            Error::Policy(ref e) => Some(e),
        }
    }
}

impl From<crate::decode::Error> for Error {
    fn from(e: crate::decode::Error) -> Error {
        Error::Decode(e)
    }
}

impl From<EarlyEndOfStreamError> for Error {
    fn from(e: EarlyEndOfStreamError) -> Error {
        Error::Decode(e.into())
    }
}

impl From<crate::types::Error> for Error {
    fn from(e: crate::types::Error) -> Error {
        Error::Type(e)
    }
}

#[cfg(feature = "elements")]
impl From<policy::Error> for Error {
    fn from(e: policy::Error) -> Error {
        Error::Policy(e)
    }
}
