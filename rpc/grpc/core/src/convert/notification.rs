use crate::protowire::{
    spectred_response::Payload, BlockAddedNotificationMessage, NewBlockTemplateNotificationMessage, RpcNotifyCommand, SpectredResponse,
};
use crate::protowire::{
    FinalityConflictNotificationMessage, FinalityConflictResolvedNotificationMessage, MempoolChangedNotificationMessage,
    NotifyPruningPointUtxoSetOverrideRequestMessage, NotifyPruningPointUtxoSetOverrideResponseMessage,
    NotifyUtxosChangedRequestMessage, NotifyUtxosChangedResponseMessage, PruningPointUtxoSetOverrideNotificationMessage,
    SinkBlueScoreChangedNotificationMessage, StopNotifyingPruningPointUtxoSetOverrideRequestMessage,
    StopNotifyingPruningPointUtxoSetOverrideResponseMessage, StopNotifyingUtxosChangedRequestMessage,
    StopNotifyingUtxosChangedResponseMessage, UtxosChangedNotificationMessage, VirtualChainChangedNotificationMessage,
    VirtualDaaScoreChangedNotificationMessage,
};
use crate::{from, try_from};
use spectre_notify::subscription::Command;
use spectre_rpc_core::{Notification, RpcError, RpcHash};
use std::str::FromStr;
use std::sync::Arc;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: &spectre_rpc_core::Notification, SpectredResponse, { Self { id: 0, payload: Some(item.into()) } });

from!(item: &spectre_rpc_core::Notification, Payload, {
    match item {
        Notification::BlockAdded(ref notification) => Payload::BlockAddedNotification(notification.into()),
        Notification::NewBlockTemplate(ref notification) => Payload::NewBlockTemplateNotification(notification.into()),
        Notification::VirtualChainChanged(ref notification) => Payload::VirtualChainChangedNotification(notification.into()),
        Notification::FinalityConflict(ref notification) => Payload::FinalityConflictNotification(notification.into()),
        Notification::FinalityConflictResolved(ref notification) => Payload::FinalityConflictResolvedNotification(notification.into()),
        Notification::UtxosChanged(ref notification) => Payload::UtxosChangedNotification(notification.into()),
        Notification::SinkBlueScoreChanged(ref notification) => Payload::SinkBlueScoreChangedNotification(notification.into()),
        Notification::VirtualDaaScoreChanged(ref notification) => Payload::VirtualDaaScoreChangedNotification(notification.into()),
        Notification::MempoolChanged(ref notification) => {
            Payload::MempoolChangedNotification(notification.into())
        }
        Notification::PruningPointUtxoSetOverride(ref notification) => {
            Payload::PruningPointUtxoSetOverrideNotification(notification.into())
        }
    }
});

from!(item: &spectre_rpc_core::BlockAddedNotification, BlockAddedNotificationMessage, { Self { block: Some((&*item.block).into()) } });

from!(&spectre_rpc_core::NewBlockTemplateNotification, NewBlockTemplateNotificationMessage);

from!(item: &spectre_rpc_core::VirtualChainChangedNotification, VirtualChainChangedNotificationMessage, {
    Self {
        removed_chain_block_hashes: item.removed_chain_block_hashes.iter().map(|x| x.to_string()).collect(),
        added_chain_block_hashes: item.added_chain_block_hashes.iter().map(|x| x.to_string()).collect(),
        accepted_transaction_ids: item.accepted_transaction_ids.iter().map(|x| x.into()).collect(),
    }
});

from!(item: &spectre_rpc_core::FinalityConflictNotification, FinalityConflictNotificationMessage, {
    Self { violating_block_hash: item.violating_block_hash.to_string() }
});

from!(item: &spectre_rpc_core::FinalityConflictResolvedNotification, FinalityConflictResolvedNotificationMessage, {
    Self { finality_block_hash: item.finality_block_hash.to_string() }
});

from!(item: &spectre_rpc_core::UtxosChangedNotification, UtxosChangedNotificationMessage, {
    Self {
        added: item.added.iter().map(|x| x.into()).collect::<Vec<_>>(),
        removed: item.removed.iter().map(|x| x.into()).collect::<Vec<_>>(),
    }
});

from!(item: &spectre_rpc_core::SinkBlueScoreChangedNotification, SinkBlueScoreChangedNotificationMessage, {
    Self { sink_blue_score: item.sink_blue_score }
});

from!(item: &spectre_rpc_core::VirtualDaaScoreChangedNotification, VirtualDaaScoreChangedNotificationMessage, {
    Self { virtual_daa_score: item.virtual_daa_score }
});

from!(item: &spectre_rpc_core::MempoolChangedNotification, MempoolChangedNotificationMessage, {
    Self { network_mempool_size: item.network_mempool_size }
});

from!(&spectre_rpc_core::PruningPointUtxoSetOverrideNotification, PruningPointUtxoSetOverrideNotificationMessage);

from!(item: Command, RpcNotifyCommand, {
    match item {
        Command::Start => RpcNotifyCommand::NotifyStart,
        Command::Stop => RpcNotifyCommand::NotifyStop,
    }
});

from!(item: &StopNotifyingUtxosChangedRequestMessage, NotifyUtxosChangedRequestMessage, {
    Self { addresses: item.addresses.clone(), command: Command::Stop.into() }
});

from!(_item: &StopNotifyingPruningPointUtxoSetOverrideRequestMessage, NotifyPruningPointUtxoSetOverrideRequestMessage, {
    Self { command: Command::Stop.into() }
});

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

try_from!(item: &SpectredResponse, spectre_rpc_core::Notification, {
    item.payload
        .as_ref()
        .ok_or_else(|| RpcError::MissingRpcFieldError("SpectredResponse".to_string(), "payload".to_string()))?
        .try_into()?
});

try_from!(item: &Payload, spectre_rpc_core::Notification, {
    match item {
        Payload::BlockAddedNotification(ref notification) => Notification::BlockAdded(notification.try_into()?),
        Payload::NewBlockTemplateNotification(ref notification) => Notification::NewBlockTemplate(notification.try_into()?),
        Payload::VirtualChainChangedNotification(ref notification) => Notification::VirtualChainChanged(notification.try_into()?),
        Payload::FinalityConflictNotification(ref notification) => Notification::FinalityConflict(notification.try_into()?),
        Payload::FinalityConflictResolvedNotification(ref notification) => {
            Notification::FinalityConflictResolved(notification.try_into()?)
        }
        Payload::UtxosChangedNotification(ref notification) => Notification::UtxosChanged(notification.try_into()?),
        Payload::SinkBlueScoreChangedNotification(ref notification) => Notification::SinkBlueScoreChanged(notification.try_into()?),
        Payload::VirtualDaaScoreChangedNotification(ref notification) => {
            Notification::VirtualDaaScoreChanged(notification.try_into()?)
        }
        Payload::PruningPointUtxoSetOverrideNotification(ref notification) => {
            Notification::PruningPointUtxoSetOverride(notification.try_into()?)
        }
        Payload::MempoolChangedNotification(ref notification) => {
            Notification::MempoolChanged(notification.try_into()?)
        },
        _ => Err(RpcError::UnsupportedFeature)?,
    }
});

try_from!(item: &BlockAddedNotificationMessage, spectre_rpc_core::BlockAddedNotification, {
    Self {
        block: Arc::new(
            item.block
                .as_ref()
                .ok_or_else(|| RpcError::MissingRpcFieldError("BlockAddedNotificationMessage".to_string(), "block".to_string()))?
                .try_into()?,
        ),
    }
});

try_from!(&NewBlockTemplateNotificationMessage, spectre_rpc_core::NewBlockTemplateNotification);

try_from!(item: &VirtualChainChangedNotificationMessage, spectre_rpc_core::VirtualChainChangedNotification, {
    Self {
        removed_chain_block_hashes: Arc::new(
            item.removed_chain_block_hashes.iter().map(|x| RpcHash::from_str(x)).collect::<Result<Vec<_>, _>>()?,
        ),
        added_chain_block_hashes: Arc::new(
            item.added_chain_block_hashes.iter().map(|x| RpcHash::from_str(x)).collect::<Result<Vec<_>, _>>()?,
        ),
        accepted_transaction_ids: Arc::new(item.accepted_transaction_ids.iter().map(|x| x.try_into()).collect::<Result<Vec<_>, _>>()?),
    }
});

try_from!(item: &FinalityConflictNotificationMessage, spectre_rpc_core::FinalityConflictNotification, {
    Self { violating_block_hash: RpcHash::from_str(&item.violating_block_hash)? }
});

try_from!(item: &FinalityConflictResolvedNotificationMessage, spectre_rpc_core::FinalityConflictResolvedNotification, {
    Self { finality_block_hash: RpcHash::from_str(&item.finality_block_hash)? }
});

try_from!(item: &UtxosChangedNotificationMessage, spectre_rpc_core::UtxosChangedNotification, {
    Self {
        added: Arc::new(item.added.iter().map(|x| x.try_into()).collect::<Result<Vec<_>, _>>()?),
        removed: Arc::new(item.removed.iter().map(|x| x.try_into()).collect::<Result<Vec<_>, _>>()?),
    }
});

try_from!(item: &SinkBlueScoreChangedNotificationMessage, spectre_rpc_core::SinkBlueScoreChangedNotification, {
    Self { sink_blue_score: item.sink_blue_score }
});

try_from!(item: &VirtualDaaScoreChangedNotificationMessage, spectre_rpc_core::VirtualDaaScoreChangedNotification, {
    Self { virtual_daa_score: item.virtual_daa_score }
});

try_from!(item: &MempoolChangedNotificationMessage, spectre_rpc_core::MempoolChangedNotification, {
    Self { network_mempool_size: item.network_mempool_size }
});

try_from!(&PruningPointUtxoSetOverrideNotificationMessage, spectre_rpc_core::PruningPointUtxoSetOverrideNotification);

from!(item: RpcNotifyCommand, Command, {
    match item {
        RpcNotifyCommand::NotifyStart => Command::Start,
        RpcNotifyCommand::NotifyStop => Command::Stop,
    }
});

from!(item: NotifyUtxosChangedResponseMessage, StopNotifyingUtxosChangedResponseMessage, { Self { error: item.error } });

from!(item: NotifyPruningPointUtxoSetOverrideResponseMessage, StopNotifyingPruningPointUtxoSetOverrideResponseMessage, {
    Self { error: item.error }
});
