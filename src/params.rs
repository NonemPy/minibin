use std::ops::Deref;

use rocket::request::{FromRequest, Outcome};
use rocket::Request;

/// Gets the Host header from the request.
///
/// The inner value of this `HostHeader` will be `None` if there was no Host header
/// on the request.
pub struct HostHeader<'a>(pub Option<&'a str>);

impl<'a> Deref for HostHeader<'a> {
    type Target = Option<&'a str>;

    fn deref(&self) -> &Option<&'a str> {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for HostHeader<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<HostHeader<'a>, ()> {
        Outcome::Success(HostHeader(request.headers().get_one("Host")))
    }
}
