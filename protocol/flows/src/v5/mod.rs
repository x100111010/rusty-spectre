use self::{
    address::{ReceiveAddressesFlow, SendAddressesFlow},
    blockrelay::{flow::HandleRelayInvsFlow, handle_requests::HandleRelayBlockRequests},
    ping::{ReceivePingsFlow, SendPingsFlow},
    request_antipast::HandleAntipastRequests,
    request_block_locator::RequestBlockLocatorFlow,
    request_headers::RequestHeadersFlow,
    request_ibd_blocks::HandleIbdBlockRequests,
    request_ibd_chain_block_locator::RequestIbdChainBlockLocatorFlow,
    request_pp_proof::RequestPruningPointProofFlow,
    request_pruning_point_and_anticone::PruningPointAndItsAnticoneRequestsFlow,
    request_pruning_point_utxo_set::RequestPruningPointUtxoSetFlow,
    txrelay::flow::{RelayTransactionsFlow, RequestTransactionsFlow},
};
use crate::{flow_context::FlowContext, flow_trait::Flow};

use crate::ibd::IbdFlow;
use spectre_p2p_lib::{Router, SharedIncomingRoute, SpectredMessagePayloadType};
use spectre_utils::channel;
use std::sync::Arc;

pub(crate) mod address;
pub(crate) mod blockrelay;
pub(crate) mod ping;
pub(crate) mod request_antipast;
pub(crate) mod request_block_locator;
pub(crate) mod request_headers;
pub(crate) mod request_ibd_blocks;
pub(crate) mod request_ibd_chain_block_locator;
pub(crate) mod request_pp_proof;
pub(crate) mod request_pruning_point_and_anticone;
pub(crate) mod request_pruning_point_utxo_set;
pub(crate) mod txrelay;

pub fn register(ctx: FlowContext, router: Arc<Router>) -> Vec<Box<dyn Flow>> {
    // IBD flow <-> invs flow communication uses a job channel in order to always
    // maintain at most a single pending job which can be updated
    let (ibd_sender, relay_receiver) = channel::job();
    let body_only_ibd_permitted = false;
    let flows: Vec<Box<dyn Flow>> = vec![
        Box::new(IbdFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                SpectredMessagePayloadType::BlockHeaders,
                SpectredMessagePayloadType::DoneHeaders,
                SpectredMessagePayloadType::IbdBlockLocatorHighestHash,
                SpectredMessagePayloadType::IbdBlockLocatorHighestHashNotFound,
                SpectredMessagePayloadType::BlockWithTrustedDataV4,
                SpectredMessagePayloadType::DoneBlocksWithTrustedData,
                SpectredMessagePayloadType::IbdChainBlockLocator,
                SpectredMessagePayloadType::IbdBlock,
                SpectredMessagePayloadType::TrustedData,
                SpectredMessagePayloadType::PruningPoints,
                SpectredMessagePayloadType::PruningPointProof,
                SpectredMessagePayloadType::UnexpectedPruningPoint,
                SpectredMessagePayloadType::PruningPointUtxoSetChunk,
                SpectredMessagePayloadType::DonePruningPointUtxoSetChunks,
            ]),
            relay_receiver,
            body_only_ibd_permitted,
        )),
        Box::new(HandleRelayInvsFlow::new(
            ctx.clone(),
            router.clone(),
            SharedIncomingRoute::new(
                router.subscribe_with_capacity(vec![SpectredMessagePayloadType::InvRelayBlock], ctx.block_invs_channel_size()),
            ),
            router.subscribe(vec![SpectredMessagePayloadType::Block, SpectredMessagePayloadType::BlockLocator]),
            ibd_sender,
        )),
        Box::new(HandleRelayBlockRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestRelayBlocks]),
        )),
        Box::new(ReceivePingsFlow::new(ctx.clone(), router.clone(), router.subscribe(vec![SpectredMessagePayloadType::Ping]))),
        Box::new(SendPingsFlow::new(ctx.clone(), router.clone(), router.subscribe(vec![SpectredMessagePayloadType::Pong]))),
        Box::new(RequestHeadersFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestHeaders, SpectredMessagePayloadType::RequestNextHeaders]),
        )),
        Box::new(RequestPruningPointProofFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestPruningPointProof]),
        )),
        Box::new(RequestIbdChainBlockLocatorFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestIbdChainBlockLocator]),
        )),
        Box::new(PruningPointAndItsAnticoneRequestsFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                SpectredMessagePayloadType::RequestPruningPointAndItsAnticone,
                SpectredMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks,
            ]),
        )),
        Box::new(RequestPruningPointUtxoSetFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                SpectredMessagePayloadType::RequestPruningPointUtxoSet,
                SpectredMessagePayloadType::RequestNextPruningPointUtxoSetChunk,
            ]),
        )),
        Box::new(HandleIbdBlockRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestIbdBlocks]),
        )),
        Box::new(HandleAntipastRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestAntipast]),
        )),
        Box::new(RelayTransactionsFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe_with_capacity(
                vec![SpectredMessagePayloadType::InvTransactions],
                RelayTransactionsFlow::invs_channel_size(),
            ),
            router.subscribe_with_capacity(
                vec![SpectredMessagePayloadType::Transaction, SpectredMessagePayloadType::TransactionNotFound],
                RelayTransactionsFlow::txs_channel_size(),
            ),
        )),
        Box::new(RequestTransactionsFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestTransactions]),
        )),
        Box::new(ReceiveAddressesFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::Addresses]),
        )),
        Box::new(SendAddressesFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestAddresses]),
        )),
        Box::new(RequestBlockLocatorFlow::new(
            ctx,
            router.clone(),
            router.subscribe(vec![SpectredMessagePayloadType::RequestBlockLocator]),
        )),
    ];

    // The reject message is handled as a special case by the router
    // SpectredMessagePayloadType::Reject,

    // We do not register the below two messages since they are deprecated also in go-spectre
    // SpectredMessagePayloadType::BlockWithTrustedData,
    // SpectredMessagePayloadType::IbdBlockLocator,

    flows
}
