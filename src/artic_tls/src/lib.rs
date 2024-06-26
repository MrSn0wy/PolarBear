use crate::client_hello::client_hello_handler;
use crate::server_hello::server_hello_handler;

mod macros;
mod client_hello;
mod server_hello;

//#[derive(Debug)]
//pub(crate) enum HttpsTls {
//    ClientHello,
//    ServerHello
//    //NewSessionTicket {},
//    //EndOfEarlyData {},
//    //EncryptedExtensions {},
//    //Certificate {},
//    //CertificateRequest {},
//    //CertificateVerify {},
//    //Finished {},
//    //KeyUpdate {},
//    //MessageHash {},
//}

#[derive(Debug)]
pub(crate) struct ClientHello {
    content_type: u8,
    version: u16,
    length: u16,
    handshake: Box<HandshakeProtocol>,
}

#[derive(Debug)]
pub(crate) struct ServerHello {
    legacy_version: u16, // must be TLS v1.2 (0x0303)
    random: Vec<u8>, // 32 bytes generated by a secure random generator
    legacy_session_id_echo: Vec<u8>, // contents of the client's legacy_session_id field
}

#[derive(Debug)]
pub(crate) struct HandshakeProtocol {
    pub(crate) handshake_type: u8,
    pub(crate) length: u32,
    pub(crate) version: u16,
    pub(crate) random: Vec<u8>,
    pub(crate) session_id_length: u8,
    pub(crate) legacy_session_id: Vec<u8>,
    pub(crate) cipher_suites_length: u16,
    pub(crate) cipher_suites: Vec<u16>, // list of symmetric cipher options, defined in descending order of client preference
    pub(crate) compression_methods_length: u8,
    pub(crate) legacy_compression_methods: Vec<u8>,
    pub(crate) extensions_length: u16,
    pub(crate) extensions: Vec<ChExtension>,
}

/// TLS 1.3 ONLY!
#[derive(Debug)]
pub(crate) enum ChExtension {
    ServerName {
        extension_type: u16,
        length: u16,
        server_name_indication_extension: Vec<(u16, u8, u16, Vec<u8>)>,
    },
    MaxFragmentLength {}, // NOT NEEDED
    StatusRequest {
        extension_type: u16,
        length: u16,
        certificate_status_type: u8,
        responder_id_list_length: u16,
        request_extensions_length: u16,
    },
    SupportedGroups {
        extension_type: u16,
        length: u16,
        supported_groups_list_length: u16,
        supported_groups: Vec<u16>,
    },
    SignatureAlgorithms {
        extension_type: u16,
        length: u16,
        signature_hash_algorithms_length: u16,
        signature_hash_algorithms: Vec<(u8, u8)>,
    },
    UseSrtp {}, // ONLY USED FOR RTP AND RTCP
    Heartbeat {},
    ApplicationLayerProtocolNegotiation {
        extension_type: u16,
        length: u16,
        alpn_extension_length: u16,
        alpn_protocol: Vec<(u8, String)>,
    },
    SignedCertificateTimestamp {},
    ClientCertificateType {},
    ServerCertificateType {},
    Padding {},
    PreSharedKey {},
    EarlyData {},
    SupportedVersions {
        extension_type: u16,
        length: u16,
        supported_versions_length: u8,
        supported_versions: Vec<u16>,
    },
    Cookie {},
    PskKeyExchangeModes {
        extension_type: u16,
        length: u16,
        psk_key_exchange_modes_length: u8,
        psk_key_exchange_mode: Vec<u8>,
    },
    CertificateAuthorities {},
    OidFilters {},
    PostHandshakeAuth {},
    SignatureAlgorithmsCert {},
    KeyShare {
        extension_type: u16,
        length: u16,
        client_key_share_length: u16,
        key_share_extensions: Vec<(u16, u16, Vec<u8>)>,
    },
}


pub fn tls_handler(buffer: Vec<u8>) {
    // check if there is a type :3
    match buffer.first() {
        Some(num) => {
            match num {
                22 => {
                    let client_hello = client_hello_handler(buffer);
                    server_hello_handler(client_hello);
                }
                _ => eprintln_red!("No clue what kind of tls message this is, are you feeling well? || type: {}", buffer[0]),
            }
        }
        _ => eprintln_red!("No clue what kind of tls message this is, are you feeling well? || type: {}", buffer[0]),
    }
}