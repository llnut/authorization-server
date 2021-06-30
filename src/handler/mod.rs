use crate::user_server::{Message as PbMessage, Request as PbRequest};
use tonic::Request;

pub mod user;

impl From<Request<PbMessage>> for PbRequest {
    fn from(pb_message: Request<PbMessage>) -> PbRequest {
        pb_message.get_ref().clone().request.unwrap()
    }
}
