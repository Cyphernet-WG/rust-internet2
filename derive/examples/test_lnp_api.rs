#![allow(dead_code, bare_trait_objects)]

#[macro_use]
extern crate inet2_derive;

#[derive(Clone, Debug, LnpApi)]
#[lnp_api(encoding = "strict")]
#[non_exhaustive]
pub enum Reply {
    #[lnp_api(type = 0x0001)]
    Failure(String),

    /// Some attribute
    #[lnp_api(type = 0x0003)]
    Success(),

    #[lnp_api(type = 0x0005)]
    SuccessNoArgs,

    #[lnp_api(type = 0x0103)]
    Keylist(Vec<u8>),
}

fn main() {
    use core::convert::TryFrom;
    use internet2::{TypeId, TypedEnum};

    let _ = Reply::Success().get_type();
    Reply::try_from_type(TypeId::try_from(0x0003).unwrap(), &Vec::<u8>::new())
        .unwrap();
}
