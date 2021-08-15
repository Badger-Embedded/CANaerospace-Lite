use crate::{message::CANAerospaceMessage, types::{DataType, MessageCode}};

pub trait CANAerospaceHandler {

    /// CANAerospace Service Request Handler
    /// If returns message then it will be automatically sent as response
    /// If it returns (x, `None`) then there will be no response for the message. (x stands for Any)
    fn handle_service_request(&mut self, message: CANAerospaceMessage) -> (Option<MessageCode>, Option<DataType>);

    /// CANAerospace Message Handler
    ///
    /// All can bus messages but service messages([`MessageType::NSH`], [`MessageType::NSL`]) will be handled by this function
    fn handle_messages(&mut self, message: CANAerospaceMessage);
}
