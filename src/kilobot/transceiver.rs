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
use crate::kilobot::messages::Message;

/// The kilobot's transceiver, which operates using callbacks
/// # Fields
/// * 'message_received' - 0 if no message received, 1 if message received. Type is u8 to reflect
/// actual kilobot code
/// * 'message_tx' - Callback function that is called whenever a message is ready to be transmitted. Returns
/// a message object to be sent, or null if no message should be sent
/// * 'message_rx' - Callback function that is called whenever a message is received. Takes a message
/// object and the measured distance from the source
/// # Notes
/// * There is no 'ack' response, a message is transmitted only if there is no contention
pub struct Transceiver
{
    message_received: u8,
    message_tx: fn() -> Option<Message>,
    message_rx: fn(msg: Message, dist: u16),
    message_tx_success: fn(),
    message_rx_success: fn(),
}

impl Transceiver
{
    /// Sets the callback function to be run when a message is ready to be transmitted
    /// # Arguments
    /// * 'cb' - Function called to check if a message is ready
    pub fn set_tx_callback(&mut self, cb : fn() -> Option<Message>)
    {
        self.message_tx = cb
    }
}