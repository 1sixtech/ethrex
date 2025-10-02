use crate::rlpx::{
    Message, connection::server::Established, error::RLPxError, mojave::messages::MojaveMessage,
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
