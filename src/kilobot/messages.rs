/*
 * messages
 * Author: Dmitri Smith
 * Purpose: Define the types of messages that can be sent and the message structure
 * Created: 7/13/20
 *
 */

/*
 * From kilobot documentation
/**
 * @brief Message structure.
 *
 * A message structure is 12 bytes in length and is composed of three
 * parts: the payload (9 bytes), the message type (1 byte), and a CRC (2
 * bytes).
 *
 * @note When preparing a message for transmission, at a minimum you
 * must specify the type (use a value between 0 and 127 for user
 * messages) and the CRC (use the message_crc() function for this
 * purpose).
 *
 * @see message_crc, kilo_message_rx, kilo_message_tx,
 * kilo_message_tx_success
 */
 */

/// Possible message types that can be sent
/// Note that not all of the actual kilobot message types are currently included
enum MessageType
{
    NORMAL = 0,
    GPS,
    BOOT = 0x80,
    RESET,
    SLEEP,
    WAKEUP,
    CHARGE,
    VOLTAGE,
    RUN,
    READUID,
    CALIB,
}

/// A message that can be transmitted by the bot
/// The message structure mimics that of the actual kilobot
/// So each message is 12 bytes long, in three parts: the payload
/// (9 bytes), the type (1 byte), and a CRC (2 bytes). Kilobot documentation does not layout the
/// structure of the payload, so for the moment it is as follows:
/// (2) src_uid, (7) data, (1) type, (2) crc.
pub struct Message
{
    data: [u8; 9],
    msg_type: u8,
    msg_crc: u8,
}

impl Message
{
    /// (NOT IMPLEMENTED!) Generate a CRC for a message
    /// # Arguments
    /// * 'message' - CRC will be generated based on the data and type of this message
    pub fn generate_crc(msg: &mut Message)
    {
        msg.msg_crc = 0x00;
    }
}