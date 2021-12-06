// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2020 by
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

use crate::transport::Error;
#[cfg(feature = "zmq")]
use crate::zmqsocket;
use crate::{
    session, LocalNode, LocalSocketAddr, NodeAddr, RemoteNodeAddr,
    RemoteSocketAddr, Session,
};

pub trait Connect {
    fn connect(&self, node: &LocalNode) -> Result<Box<dyn Session>, Error>;
}

pub trait Accept {
    fn accept(&self, node: &LocalNode) -> Result<Box<dyn Session>, Error>;
}

impl Connect for LocalSocketAddr {
    fn connect(&self, local: &LocalNode) -> Result<Box<dyn Session>, Error> {
        #[cfg(not(feature = "zmq"))]
        unimplemented!();
        #[cfg(feature = "zmq")]
        Ok(Box::new(match self {
            LocalSocketAddr::Zmq(locator) => {
                session::Raw::with_zmq_unencrypted(
                    zmqsocket::ZmqType::Req,
                    locator,
                    None,
                    None,
                )?
            }
            LocalSocketAddr::Posix(_) => unimplemented!(),
        }))
    }
}

impl Accept for LocalSocketAddr {
    fn accept(&self, local: &LocalNode) -> Result<Box<dyn Session>, Error> {
        #[cfg(not(feature = "zmq"))]
        unimplemented!();
        #[cfg(feature = "zmq")]
        Ok(Box::new(match self {
            LocalSocketAddr::Zmq(locator) => {
                session::Raw::with_zmq_unencrypted(
                    zmqsocket::ZmqType::Req,
                    locator,
                    None,
                    None,
                )?
            }
            LocalSocketAddr::Posix(_) => unimplemented!(),
        }))
    }
}

#[cfg(feature = "keygen")]
impl Connect for RemoteNodeAddr {
    fn connect(&self, local: &LocalNode) -> Result<Box<dyn Session>, Error> {
        Ok(match self.remote_addr {
            RemoteSocketAddr::Ftcp(inet) => {
                Box::new(session::Raw::connect_ftcp_encrypted(
                    local.private_key(),
                    self.node_id,
                    inet,
                )?) as Box<dyn Session>
            }
            #[cfg(feature = "zmq")]
            // TODO: (v0.3) pass specific ZMQ API type using additional
            //       `RemoteAddr` field
            RemoteSocketAddr::Zmq(socket) => {
                Box::new(session::Raw::with_zmq_unencrypted(
                    zmqsocket::ZmqType::Req,
                    &zmqsocket::ZmqSocketAddr::Tcp(socket),
                    None,
                    None,
                )?)
            }
            RemoteSocketAddr::Http(_) => unimplemented!(),
            #[cfg(feature = "websocket")]
            RemoteSocketAddr::Websocket(_) => unimplemented!(),
            RemoteSocketAddr::Smtp(_) => unimplemented!(),
        })
    }
}

#[cfg(feature = "keygen")]
impl Accept for RemoteNodeAddr {
    fn accept(&self, local: &LocalNode) -> Result<Box<dyn Session>, Error> {
        Ok(match self.remote_addr {
            RemoteSocketAddr::Ftcp(inet) => Box::new(
                session::Raw::accept_ftcp_encrypted(local.private_key(), inet)?,
            ) as Box<dyn Session>,
            #[cfg(feature = "zmq")]
            // TODO: (v0.3) pass specific ZMQ API type using additional
            //       `RemoteAddr` field
            RemoteSocketAddr::Zmq(socket) => {
                Box::new(session::Raw::with_zmq_unencrypted(
                    zmqsocket::ZmqType::Req,
                    &zmqsocket::ZmqSocketAddr::Tcp(socket),
                    None,
                    None,
                )?)
            }
            RemoteSocketAddr::Http(_) => unimplemented!(),
            #[cfg(feature = "websocket")]
            RemoteSocketAddr::Websocket(_) => unimplemented!(),
            RemoteSocketAddr::Smtp(_) => unimplemented!(),
        })
    }
}

impl Connect for NodeAddr {
    fn connect(&self, local: &LocalNode) -> Result<Box<dyn Session>, Error> {
        match self {
            NodeAddr::Local(addr) => addr.connect(local),
            NodeAddr::Remote(addr) => addr.connect(local),
        }
    }
}

impl Accept for NodeAddr {
    fn accept(&self, local: &LocalNode) -> Result<Box<dyn Session>, Error> {
        match self {
            NodeAddr::Local(addr) => addr.accept(local),
            NodeAddr::Remote(addr) => addr.accept(local),
        }
    }
}
