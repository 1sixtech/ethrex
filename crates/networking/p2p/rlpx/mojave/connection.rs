use crate::rlpx::{
    Message,
    connection::server::{Established, send},
    error::RLPxError,
    mojave::messages::MojaveMessage,
    utils::log_peer_error,
};

pub(crate) async fn handle_mojave_capability_message(
    state: &mut Established,
    msg: MojaveMessage,
) -> Result<(), RLPxError> {
    let task_id = tokio::task::id();
    state
        .connection_broadcast_send
        .send((task_id, Message::Mojave(msg).into()))
        .inspect_err(|e| {
            log_peer_error(
                &state.node,
                &format!("Could not broadcast mojave message: {e}"),
            );
        })
        .map_err(|_| RLPxError::BroadcastError("Could not broadcast mojave message".to_owned()))?;
    Ok(())
}

pub(crate) async fn handle_mojave_broadcast(
    state: &mut Established,
    l2_msg: &Message,
) -> Result<(), RLPxError> {
    match l2_msg {
        msg @ Message::Mojave(MojaveMessage::Proof(_)) => send(state, msg.clone()).await,
        msg @ Message::Mojave(MojaveMessage::Block(_)) => send(state, msg.clone()).await,
        _ => Err(RLPxError::BroadcastError(format!(
            "Message {:?} is not a valid L2 message for broadcast",
            l2_msg
        )))?,
    }
}
