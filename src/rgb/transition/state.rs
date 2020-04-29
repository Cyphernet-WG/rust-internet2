// LNP/BP Rust Library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[non_exhaustive]
#[derive(Clone, PartialEq, Debug, Display)]
#[display_from(Debug)]
pub enum Data {
    Balance(amount::Commitment),
    Binary(Box<[u8]>),
    None,
    // TODO: Add other supported bound state types according to the schema
}

use super::commit::StateCommitment;
use crate::{
    common::Wrapper,
    rgb::{data, seal},
};

#[derive(Clone, PartialEq, Debug, Display)]
#[display_from(Debug)]
pub enum Partial {
    Commitment(StateCommitment),
    State(Bound),
}

#[derive(Clone, PartialEq, Debug, Display)]
#[display_from(Debug)]
pub struct Bound {
    pub id: seal::Type,
    pub seal: seal::Seal,
    pub val: data::Data,
}

wrapper!(
    State,
    _StatePhantom,
    Vec<Partial>,
    doc = "Set of partial state data"
);
