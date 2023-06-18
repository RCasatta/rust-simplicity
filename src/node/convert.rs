// Rust Simplicity Library
// Written in 2023 by
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

//! Node Conversion
//!
//! This module defines a trait [`Conversion`] which is called by the
//! [`Node::convert`] method. The `Convert` trait is used to convert between
//! one node type and another, controlling several nuanced aspects of the
//! conversion. Specifically:
//!
//! 1. Cached data can be translated from the old type to the new one.
//! 2. Witness data can be provided (or translated from the old witness type,
//!    if it was nontrivial) to attach to witness nodes.
//! 3. For `case` nodes, the decision can be made to hide one of the children.
//!    In this case the `case` node is converted to an `AssertL` or `AssertR`
//!    node, depending which child was hidden.
//!

use crate::dag::PostOrderIterItem;
use crate::jet::Jet;

use super::{Inner, Node, NodeData};

use std::sync::Arc;

/// A decision about which, if any, child branches of a `case` combinator to hide
/// during a [`Node::convert_hiding`] conversion
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Hide {
    Neither,
    Left,
    Right,
}

/// The primary trait controlling how a node conversion is done.
///
/// When a node is converted using [`Node::convert`], the original DAG rooted at
/// that node is traversed in post order. For each node, the following steps are
/// taken:
///
/// 1. First, [`Self::visit_node`] is called, before any other checks are
///    done. This happens regardless of the node's type or whether it is going
///    to be pruned.
///
///    This method is provided for convenience and does not affect the course
///    of the algorithm. It has a default implementation which does nothing.
///
/// 2. Then, if the node is a witness node, `Self::convert_witness` is called
///    to obtain witness data.
///
/// 3. If the node is a case node, [`Self::prune_case`] is called to decide
///    whether to prune either child of the node (turning the `case` into an
///    `assertl` or `assertr`). The default implementation hides neither.
///
/// 4. Finally, the node's data is passed to [`Self::convert_data`], whose job
///    it is to compute the cached data for the new node. For `case` combinators
///    where one child was pruned, `convert_data` will receive an `assertl` or
///    `assertl`, as appropriate, rather than a `case`.
///
/// If any method returns an error, then iteration is aborted immediately and
/// the error returned to the caller. If the converter would like to recover
/// from errors and/or accumulate multiple errors, it needs to do this by
/// tracking errors internally.
///
/// The finalization method will not return any errors except those returned by
/// methods on [`Converter`].
pub trait Converter<N: NodeData<J>, M: NodeData<J>, J: Jet> {
    /// The error type returned by the methods on this trait.
    type Error;

    /// This method is called on every node, to inform the `Converter` about the
    /// state of the iterator.
    ///
    /// No action needs to be taken. The default implementation does nothing.
    fn visit_node(&mut self, _data: &PostOrderIterItem<&Node<N, J>>) {}

    /// For witness nodes, this method is called first to attach witness data to
    /// the node.
    ///
    /// It takes the iteration data of the current node, as well as the current
    /// witness (which in a typical scenario will be an empty structure, but
    /// with custom node types may be a placeholder or other useful information)
    ///
    /// No typechecking or other sanity-checking is done on the returned value.
    /// It is the caller's responsibility to make sure that the provided witness
    /// actually matches the type of the combinator that it is being attached to.
    fn convert_witness(
        &mut self,
        data: &PostOrderIterItem<&Node<N, J>>,
        witness: &N::Witness,
    ) -> Result<M::Witness, Self::Error>;

    /// For case nodes, this method is called first to decide which, if any, children
    /// to prune.
    ///
    /// It takes the iteration data of the current node, as well as both of its already
    /// converted children. This method returns a hiding decision.
    ///
    /// The default implementation doesn't do any hiding.
    fn prune_case(
        &mut self,
        _data: &PostOrderIterItem<&Node<N, J>>,
        _left: &Arc<Node<M, J>>,
        _right: &Arc<Node<M, J>>,
    ) -> Result<Hide, Self::Error> {
        Ok(Hide::Neither)
    }

    /// This method is called for every node, after [`Self::convert_witness`] or
    /// [`Self::prune_case`], if either is applicable.
    ///
    /// For case nodes for which [`Self::prune_case`] returned [`Hide::Left`] or
    /// [`Hide::Right`], `inner` will be an [`Inner::AssertR`] or [`Inner::AssertL`]
    /// respectively; the pruned child will then appear only as a CMR.
    ///
    /// It accepts the iteration data of the current node, from which the existing
    /// cached data can be obtained by calling `data.node.cached_data()`, as well
    /// as an `Inner` structure containing its already-converted children.
    ///
    /// Returns new cached data which will be attached to the newly-converted node.
    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&Node<N, J>>,
        inner: Inner<&Arc<Node<M, J>>, J, &M::Witness>,
    ) -> Result<M::CachedData, Self::Error>;
}
