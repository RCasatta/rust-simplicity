// Rust Simplicity Library
// Written in 2023 by
//   Andrew Poelstra <rust-simplicity@wpsoftware.net>
//   Christian Lewe <clewe@blockstream.com>
//   Sanket Kanjalkar <sanket1729@gmail.com>
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

//! The Simplicity Human-Readable Encoding
//!
//! This module provides the ability to decode and encode [`crate::NamedCommitNode`]s
//! in a human-readable format.
//!

mod error;
mod named_node;
mod parse;
mod serialize;

use crate::dag::{DagLike, MaxSharing};
use crate::jet::Jet;
use crate::node::{self, CommitNode};
use crate::{Cmr, Imr};

use std::collections::HashMap;
use std::str;
use std::sync::Arc;

pub use self::error::{Error, ErrorSet};
pub use self::named_node::NamedCommitNode;
pub use self::parse::parse;

/// Line/column pair
///
/// There is a similar type provided by the `santiago` library but it does not implement
/// `Copy`, among many other traits, which makes it unergonomic to use. Santiago positions
/// can be converted using `.into()`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Position {
    line: usize,
    column: usize,
}

impl<'a> From<&'a santiago::lexer::Position> for Position {
    fn from(position: &'a santiago::lexer::Position) -> Position {
        Position {
            line: position.line,
            column: position.column,
        }
    }
}

impl From<santiago::lexer::Position> for Position {
    fn from(position: santiago::lexer::Position) -> Position {
        Position {
            line: position.line,
            column: position.column,
        }
    }
}

/// For named construct nodes, we abuse the `witness` combinator to support typed holes.
///
/// We do this because `witness` nodes have free source and target arrows, and
/// allow us to store arbitrary data in them using the generic witness type of
/// the node. Holes are characterized entirely by their source and target type,
/// just like witnesses, and are labelled by their name.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum WitnessOrHole {
    /// This witness is an actual witness combinator (with no witness data attached,
    /// that comes later)
    Witness,
    /// This is a typed hole, with the given name.
    TypedHole(Arc<str>),
}

impl WitnessOrHole {
    pub fn shallow_clone(&self) -> Self {
        match self {
            WitnessOrHole::Witness => WitnessOrHole::Witness,
            WitnessOrHole::TypedHole(name) => WitnessOrHole::TypedHole(Arc::clone(name)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forest<J: Jet> {
    roots: HashMap<Arc<str>, Arc<NamedCommitNode<J>>>,
}

impl<J: Jet> Forest<J> {
    /// Parses a forest from a string
    pub fn parse(s: &str) -> Result<Self, ErrorSet> {
        parse::parse(s).map(|roots| Forest { roots })
    }

    /// Parses a program from a bytestring
    pub fn from_program(root: Arc<CommitNode<J>>) -> Self {
        let root = NamedCommitNode::from_node(&root);
        let mut roots = HashMap::new();
        roots.insert("main".into(), root);
        Forest { roots }
    }

    /// Accessor for the map of roots of this forest
    pub fn roots(&self) -> &HashMap<Arc<str>, Arc<NamedCommitNode<J>>> {
        &self.roots
    }

    /// Serialize the program in human-readable form
    pub fn string_serialize(&self) -> String {
        struct Print {
            cmr: Cmr,
            imr: Option<Imr>,
            expr_str: String,  // The X = Y part
            arrow_str: String, // The :: A -> B part
        }
        fn print_lines(lines: &[Print], skip_before_end: bool) -> String {
            let mut ret = String::new();
            let expr_width = lines.iter().map(|line| line.expr_str.len()).max().unwrap();
            let arrow_width = lines.iter().map(|line| line.arrow_str.len()).max().unwrap();
            let last_line = lines.len();
            for (n, line) in lines.iter().enumerate() {
                ret += "\n";
                if skip_before_end && n == last_line - 1 {
                    ret += "\n";
                }
                ret += &format!("-- CMR: {}\n", line.cmr);
                if let Some(imr) = line.imr {
                    ret += &format!("-- IMR: {}\n", imr);
                } else {
                    ret += "-- IMR: [undetermined]\n";
                }
                ret += &format!(
                    "{0:1$} {2:3$}\n",
                    line.expr_str, expr_width, line.arrow_str, arrow_width,
                );
            }
            ret
        }

        let mut witness_lines = vec![];
        let mut const_lines = vec![];
        let mut program_lines = vec![];
        let mut disc_idx = 0;
        // Pass 1: compute string data for every node
        for root in self.roots.values() {
            for data in root.as_ref().post_order_iter::<MaxSharing<_>>() {
                let node = data.node;
                let name = node.name();
                let mut expr_str = match node.inner() {
                    node::Inner::AssertR(cmr, _) => format!("{} := assertr #{}", name, cmr),
                    node::Inner::Fail(entropy) => format!("{} := fail {}", name, entropy),
                    node::Inner::Jet(ref j) => format!("{} := jet_{}", name, j),
                    node::Inner::Word(ref v) => {
                        format!("{} := const {}", name, serialize::DisplayWord(v))
                    }
                    inner => format!("{} := {}", name, inner),
                };
                if let Some(child) = node.left_child() {
                    expr_str.push(' ');
                    expr_str.push_str(child.name());
                }
                if let Some(child) = node.right_child() {
                    expr_str.push(' ');
                    expr_str.push_str(child.name());
                } else if let node::Inner::AssertL(_, cmr) = node.inner() {
                    expr_str.push_str(" #");
                    expr_str.push_str(&cmr.to_string());
                } else if let node::Inner::Disconnect(_, node::NoDisconnect) = node.inner() {
                    expr_str.push_str(&format!(" ?disc_expr_{}", disc_idx));
                    disc_idx += 1;
                }

                let arrow = node.arrow();
                let arrow_str = format!(": {} -> {}", arrow.source, arrow.target).replace('×', "*"); // for human-readable encoding stick with ASCII

                let print = Print {
                    cmr: node.cmr(),
                    imr: node.imr(),
                    expr_str,
                    arrow_str,
                };
                if let node::Inner::Witness(..) = node.inner() {
                    witness_lines.push(print);
                } else if let node::Inner::Word(..) = node.inner() {
                    const_lines.push(print);
                } else {
                    program_lines.push(print);
                }
            }
        }

        // Pass 2: actually print everything
        let mut ret = String::new();
        if !witness_lines.is_empty() {
            ret += "--------------\n-- Witnesses\n--------------\n";
            ret += &print_lines(&witness_lines, false);
            ret += "\n";
        }
        if !const_lines.is_empty() {
            // FIXME detect scribes
            ret += "--------------\n-- Constants\n--------------\n";
            ret += &print_lines(&const_lines, false);
            ret += "\n";
        }
        if !program_lines.is_empty() {
            ret += "--------------\n-- Program code\n--------------\n";
            ret += &print_lines(&program_lines, true /* add a blank line before main */);
        }
        ret
    }
}
