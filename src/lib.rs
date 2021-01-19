// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2019 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![recursion_limit = "256"]
// Coding conventions
#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    //dead_code,
    //missing_docs
)]
// TODO: when we will be ready for the release #![deny(missing_docs, dead_code)]
// This is required because of incomplete rust async implementation and can be
// removed after async trait feature completion in rust compiler
#![cfg_attr(feature = "async", allow(where_clauses_object_safety))]

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate amplify_derive;
#[macro_use]
extern crate strict_encoding;
#[macro_use]
extern crate lazy_static;

extern crate chacha20poly1305;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_with;
#[cfg(feature = "serde")]
extern crate serde_crate as serde;

pub mod addr {
    pub use inet2_addr::*;
}

pub mod application;
pub mod presentation;
pub mod session;
pub mod transport;

pub use application::rpc_connection;
pub use presentation::encoding as lightning_encoding;
pub use presentation::payload::{TypeId, TypedEnum};
pub use presentation::{
    encoding, payload, tlv, CreateUnmarshaller, Payload, UnknownTypeError,
    Unmarshall, UnmarshallFn, Unmarshaller,
};
pub use session::{
    Accept, Connect, Decrypt, Encrypt, LocalNode, NodeAddr, NoiseDecryptor,
    NoiseEncryptor, NoiseTranscoder, PartialNodeAddr, PlainTranscoder,
    RemoteNodeAddr, Session, Split, ToNodeAddr, ToRemoteNodeAddr, Transcode,
};
pub use transport::{
    ftcp, websocket, zmqsocket, Duplex, FramingProtocol, LocalSocketAddr,
    RemoteSocketAddr, RoutedFrame,
};

pub const LNP_MSG_MAX_LEN: usize = core::u16::MAX as usize;

pub const LIGHTNING_P2P_DEFAULT_PORT: u16 = 9735;

#[cfg(feature = "zmq")]
pub use transport::{ZmqSocketAddr, ZmqType, ZMQ_CONTEXT};

/// Trait used by different address types (transport-, session- and
/// presentation-based) for getting scheme part of the URL
pub trait UrlString {
    /// Returns full URL scheme string (i.e. including `:` or `://` parts)
    /// corresponding to the provided address
    fn url_scheme(&self) -> &'static str;

    /// Returns URL string representation for a given node or socket address. If
    /// you need full URL address, please use [`Url::from()`] instead (this
    /// will require `url` feature for LNP/BP Core Library).
    fn to_url_string(&self) -> String;
}

use inet2_addr::NoOnionSupportError;

/// Error extracting transport-level address types ([`FramingProtocol`],
/// [`LocalAddr`], [`RemoteAddr`]) and session-level node types ([`NodeAddr`],
/// [`RemoteNodeAddr`]) from string, URLs and other data types
#[derive(Clone, PartialEq, Eq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum AddrError {
    /// Unknown protocol name in URL scheme ({0})
    UnknownProtocol(String),

    /// The provided URL scheme {0} was not recognized
    UnknownUrlScheme(String),

    /// Can't parse URL from the given string
    #[cfg(feature = "url")]
    #[from]
    MalformedUrl(url::ParseError),

    /// Malformed IP address.
    /// NB: DNS addressing is not used since it is considered insecure in terms
    ///     of censorship resistance, so you need to provide it in a form of
    ///     either IPv4 or IPv6 address. If you need Tor support use other
    ///     protocol type supporting Tor.
    #[from]
    MalformedIpAddr(std::net::AddrParseError),

    /// Malformed IP or Onion address.
    /// NB: DNS addressing is not used since it is considered insecure in terms
    ///     of censorship resistance, so you need to provide it in a form of
    ///     either IPv4, IPv6 address or Tor v2, v3 address (w/o `.onion`
    ///     suffix)
    #[from]
    MalformedInetAddr(inet2_addr::AddrParseError),

    /// Invalid public key data representing node id
    #[from(bitcoin::secp256k1::Error)]
    InvalidPubkey,

    /// No host information found in URL, while it is required for the given
    /// scheme
    HostRequired,

    /// No port information found in URL, while it is required for the given
    /// scheme
    PortRequired,

    /// Unexpected URL authority data (part before '@' in URL) which must be
    /// omitted
    UnexpectedAuthority,

    /// Used scheme must not contain information about host
    UnexpectedHost,

    /// Used scheme must not contain information about port
    UnexpectedPort,

    /// Unsupported ZMQ API type ({0}). List of supported APIs:
    /// - `rpc`
    /// - `p2p`
    /// - `sub`
    /// - `esb`
    InvalidZmqType(String),

    /// No ZMQ API type information for URL scheme that requires one.
    ZmqTypeRequired,

    /// `Inproc` ZMQ type requires ZMQ context which exsits only in runtime and
    /// can't be persisted. This, it can't be provided through this type.
    ZmqContextRequired,

    /// The provided protocol can't be used for {0}
    Unsupported(&'static str),

    /// Onion addresses are not supported by this socket type
    #[from(NoOnionSupportError)]
    NoOnionSupport,
}
