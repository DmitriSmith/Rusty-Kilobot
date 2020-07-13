/*
 * Important notes from Kilobot documentation (kilolib.h)
 * ------------------------------------------------------------------
 * The user can register callbacks to interact with the messaging
 * subsystem. There are callbacks for the events of message reception
 * (::kilo_message_rx), message transmission (::kilo_message_tx), and
 * notification of successful transmission (::kilo_message_tx_success).
 * By default every kilobot attempts to send message twice per second.
 * Advanced users can modify this through the `kilo_tx_period` variable,
 * although this is discouraged unless you know what you are doing.
 *
 * To prevent collisions the kilobot library uses a basic exponential
 * back-off strategy with carrier sensing. There are no acknowledgement
 * packets, and as such a message is considered to be successfully
 * transmitted when a kilobot is able transmit a message without
 * detecting any contention in the channel.
 * ------------------------------------------------------------------
 */
/// The kilobot's transceiver, which operates using callbacks
/// # Fields
/// * 'message_received' - 0 if no message receieved, 1 if message received. Type is u8 to reflect
/// actual kilobot code
/// * 'message_tx' - Callback function that is called whenever a message is ready to be transmitted
/// * 'message_rx' - Callback function that is called whenever a message is received
pub struct transceiver
{
    message_received: u8,
    message_tx: fn(),
    message_rx: fn(),
}