// xsd:
pub mod common;
#[allow(unused_imports)]
pub mod metadatastream;
pub mod onvif;
pub mod radiometry;
pub mod rules;
pub mod soap_envelope;
pub mod types;
pub mod validate;
pub mod xmlmime;
pub mod xop;

// wsdl:
#[allow(unused_imports)]
pub mod accesscontrol;
#[allow(unused_imports)]
pub mod accessrules;
#[allow(unused_imports)]
pub mod actionengine;
#[allow(unused_imports)]
pub mod advancedsecurity;
#[allow(unused_imports)]
pub mod analytics;
#[allow(unused_imports)]
pub mod authenticationbehavior;
#[allow(unused_imports)]
pub mod b_2;
#[allow(unused_imports)]
pub mod bf_2;
#[allow(unused_imports)]
pub mod credential;
#[allow(unused_imports)]
pub mod deviceio;
#[allow(unused_imports)]
pub mod devicemgmt;
#[allow(unused_imports)]
pub mod display;
#[allow(unused_imports)]
pub mod doorcontrol;
#[allow(unused_imports)]
pub mod event;
#[allow(unused_imports)]
pub mod imaging;
#[allow(unused_imports)]
pub mod media;
#[allow(unused_imports)]
pub mod media2;
#[allow(unused_imports)]
pub mod provisioning;
#[allow(unused_imports)]
pub mod ptz;
#[allow(unused_imports)]
pub mod receiver;
#[allow(unused_imports)]
pub mod recording;
#[allow(unused_imports)]
pub mod replay;
#[allow(unused_imports)]
pub mod schedule;
#[allow(unused_imports)]
pub mod search;
#[allow(unused_imports)]
pub mod t_1;
#[allow(unused_imports)]
pub mod thermal;
#[allow(unused_imports)]
pub mod uplink;
#[allow(unused_imports)]
pub mod ws_addr;
pub mod ws_discovery;

#[cfg(test)]
mod tests;
