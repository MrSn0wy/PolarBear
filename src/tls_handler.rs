use crate::{eprintln_red, println_cyan, println_green};

#[derive(Debug)]
pub(crate) enum HttpsTls {
    ClientHello {
        content_type: u8,
        version: u16,
        length: u16,
        handshake: Box<HandshakeProtocol>,
    },
    //ServerHello {},
    //NewSessionTicket {},
    //EndOfEarlyData {},
    //EncryptedExtensions {},
    //Certificate {},
    //CertificateRequest {},
    //CertificateVerify {},
    //Finished {},
    //KeyUpdate {},
    //MessageHash {},
}

#[derive(Debug)]
pub(crate) struct HandshakeProtocol {
    handshake_type: u8,
    length: u32,
    version: u16,
    random: String,
    session_id_length: u8,
    session_id: String,
    cipher_suites_length: u16,
    cipher_suites: Vec<u16>,
    compression_methods_length: u8,
    compression_methods: Vec<u8>,
    extensions_length: u16,
    extensions: Vec<Extension>,
}

//impl HandshakeProtocol {
//    fn set_random(&mut self, new_string: String) {
//        self.random = new_string;
//    }
//}
#[derive(Debug)]
enum Extension {
    Reserved {
        extension_type: u16,
        length: u16,
        data: u8, // TODO! figure out
    },
    SupportedGroups {
        extension_type: u16,
        length: u16,
        supported_groups_list_length: u16,
        supported_groups: Vec<u16>,
    },
    CompressCertificate {
        extension_type: u16,
        length: u16,
        algorithms_length: u8,
        algorithm: u16, // TODO! figure out
    },
    SignedCertificateTimestamp {
        extension_type: u16,
        length: u16,
    },
    EncryptedClientHello {
        extension_type: u16,
        length: u16,
        client_hello_type: u8,
        cipher_suite: Vec<u16>,
        config_id: u8,
        enc_length: u16,
        enc: String,
        payload_length: u16,
        payload: String,
    },
    SignatureAlgorithms {
        extension_type: u16,
        length: u16,
        signature_hash_algorithms_length: u16,
        signature_hash_algorithms: Vec<(u8, u8)>, // TODO! not greatly implemented
    },
    SessionTicket {
        extension_type: u16,
        length: u16,
        session_ticket: u8, // TODO! figure out
    },
    ServerName {
        extension_type: u16,
        length: u16,
        server_name_indication_extension: (u16, u8, u16, String), // TODO! not greatly implemented
    },
    RenegotiationInfo {
        extension_type: u16,
        length: u16,
        renegotiation_info_extension: (u8), // TODO! not greatly implemented
    },
    EcPointFormats {
        extension_type: u16,
        length: u16,
        ec_points_formats_length: u8,
        elliptic_curves_point_formats: Vec<u8>, // TODO! not greatly implemented
    },
    ApplicationLayerProtocolNegotiation {
        extension_type: u16,
        length: u16,
        alpn_extension_length: u16,
        alpn_protocol: (u8, String, u8, String), // TODO! not greatly implemented
    },
    PskKeyExchangeModes {
        extension_type: u16,
        length: u16,
        psk_key_exchange_modes_length: u8,
        psk_key_exchange_mode: u8, // TODO! figure out
    },
    StatusRequest {
        extension_type: u16,
        length: u16,
        certificate_status_type: u8,
        responder_id_list_length: u16,
        request_extensions_length: u16,
    },
    SupportedVersions {
        extension_type: u16,
        length: u16,
        supported_versions_length: u8,
        supported_versions: Vec<u16>, // TODO! not greatly implemented
    },
    KeyShare {
        extension_type: u16,
        length: u16,
        key_share_extension: (u16, Vec<(u16, u16, String)>),
    },
    ExtendedMasterSecret {
        extension_type: u16,
        length: u16,
    },
    ApplicationSettings {
        extension_type: u16,
        length: u16,
        alps_extension_length: u16,
        supported_alpn_list: Vec<(u8, String)>,
    },
    DelegatedCredentials {
        extension_type: u16,
        length: u16,
        signature_hash_algorithms_length: u16,
        signature_hash_algorithm: Vec<(u8, u8)>,
    },
    RecordSizeLimit {
        extension_type: u16,
        length: u16,
        record_size_limit: u16,
    },
}


pub(crate) fn tls_handler(buffer: Vec<u8>) {
    // check if there is a type :3
    match buffer.first() {
        Some(num) => {
            match num {
                22 => client_hello_handler(buffer),
                _ => eprintln_red!("No clue what kind of tls message this is, are you feeling well? || type: {}", buffer[0]),
            }
        }
        _ => eprintln_red!("No clue what kind of tls message this is, are you feeling well? || type: {}", buffer[0]),
    }
}

fn client_hello_handler(buffer: Vec<u8>) {
    let mut silly = HttpsTls::ClientHello {
        content_type: buffer[0],
        version: u16::from_be_bytes([buffer[1], buffer[2]]),
        length: u16::from_be_bytes([buffer[3], buffer[4]]),
        handshake: Box::new(HandshakeProtocol {
            handshake_type: buffer[5],
            length: u32::from_be_bytes([0x0, buffer[6], buffer[7], buffer[8]]),
            version: u16::from_be_bytes([buffer[9], buffer[10]]),
            random: "".to_string(),
            session_id_length: buffer[43],
            session_id: "".to_string(),
            cipher_suites_length: 0,
            cipher_suites: vec![],
            compression_methods_length: 0,
            compression_methods: vec![],
            extensions_length: 0,
            extensions: vec![],
        }),
    };

    // modify any non-directly assignable values :3
    if let HttpsTls::ClientHello { ref mut handshake, .. } = &mut silly {
        {
            let mut temp_string: String = "".to_string();

            for byte in buffer[11..=42].iter() {
                temp_string.push_str(&format!("{:02x}", byte));
            }

            handshake.random = temp_string;
        }

        let relative_start = 43 + handshake.session_id_length as usize;

        {
            let mut temp_string: String = "".to_string();

            for byte in buffer[43..relative_start].iter() {
                temp_string.push_str(&format!("{:02x}", byte));
            }

            handshake.session_id = temp_string;
        }

        {
            handshake.cipher_suites_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]); // todo: fix this mf

            for i in (relative_start + 2..=relative_start + (handshake.cipher_suites_length as usize)).step_by(2) {
                let cipher = u16::from_be_bytes([buffer[i], buffer[i + 1]]);
                handshake.cipher_suites.push(cipher);

                println_cyan!("supported cipher: {:02x}{:02x} || index: {}", buffer[i], buffer[i+1], i+1);
            }
        }

        let relative_start = relative_start + 2 + (handshake.cipher_suites_length as usize);

        {
            let methods_length = buffer[relative_start] as usize;
            handshake.compression_methods_length = buffer[relative_start];
            println_cyan!("{} = {} is this good?", relative_start, handshake.compression_methods_length);

            for i in relative_start + 1..(relative_start + 1 + methods_length) {
                handshake.compression_methods.push(buffer[i]);

                println_cyan!("support compression: {:02x}", buffer[i]);
            }
        }

        let relative_start = relative_start + 1 + (handshake.compression_methods_length as usize);

        println_green!("{:?}", &handshake);

        {
            handshake.extensions_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]);

            let mut relative_start = relative_start + 2;
            println_green!("extension_length:{} current:{}, left over:{}", handshake.extensions_length, relative_start, handshake.length as usize - relative_start);

            while relative_start < handshake.extensions_length as usize {
                let extension_type = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]);

                //match extension_type {}

                relative_start += 2;
                let extension_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]);

                relative_start += 2;

                let extension_data: &[u8] = if (relative_start + (extension_length as usize)) < (extension_length as usize) {
                    &buffer[relative_start..(relative_start + (extension_length as usize))]
                } else {
                    //relative_extension_start = handshake_extension_length as usize;
                    print!("[TRUNCATED] ");
                    &buffer[relative_start..=(extension_length as usize)]
                };
                relative_start += extension_length as usize;
                println_green!("Extension found! Type: {} || Length: {} || Extension_data: {:?}", extension_type,extension_length,extension_data);
            }
        }
    }

    //let content_type = buffer[0];
    //let version = format!("{}.{}", buffer[1], buffer[2] - 1);
    //let length = u16::from_be_bytes([buffer[3], buffer[4]]);
    //println_green!("content_type:{} version:{} length:{}", content_type, version, length);
    //
    //// handshake protocol
    //let handshake_protocol = &buffer[5..=(length as usize)];
    //println!("handshake protocol: {:?}", handshake_protocol);
    //
    //let handshake_type = &handshake_protocol[0];
    //let handshake_length = u32::from_be_bytes([0x0, handshake_protocol[1], handshake_protocol[2], handshake_protocol[3]]);
    //let handshake_version = format!("{}.{}", handshake_protocol[4], handshake_protocol[5] - 1);

    //let mut handshake_random = String::new();
    //for byte in handshake_protocol[6..=37].iter() {
    //    handshake_random.push_str(&format!("{:02x}", byte));
    //}

    //let handshake_session_id_length = handshake_protocol[38];
    //let relative_start = 39 + handshake_session_id_length;

    //let mut handshake_session_id = String::new();
    //for byte in handshake_protocol[39..(relative_start as usize)].iter() {
    //    handshake_session_id.push_str(&format!("{:02x}", byte));
    //}

    //let handshake_cipher_suites_length = u16::from_be_bytes([handshake_protocol[relative_start as usize], handshake_protocol[(relative_start as usize) + 1]]);


    //for i in ((relative_start as usize) + 2..((relative_start as usize) + 1 + (handshake_cipher_suites_length as usize))).step_by(2) {
    //    println_cyan!("supported cipher: {:02x}{:02x} || index: {}", handshake_protocol[i], handshake_protocol[i+1], i+1);
    //}

    //let relative_start = (relative_start as usize) + 2 + (handshake_cipher_suites_length as usize);

    //let handshake_compression_method_length = handshake_protocol[relative_start];
    //println_cyan!("{} = {}", relative_start, handshake_compression_method_length);

    //for i in (handshake_compression_method_length as usize) + 2..((handshake_compression_method_length as usize) + 1 + (handshake_compression_method_length as usize)) {
    //    println_cyan!("support compression: {:02x}", handshake_protocol[i]);
    //}

    //println_green!("type:{} length:{} version:{}, random:{}, session_id_length:{}, session_id:{} cipher_suites_length:{} compression_method_length: {}", handshake_type, handshake_length, handshake_version, handshake_random, handshake_session_id_length, handshake_session_id, handshake_cipher_suites_length, handshake_compression_method_length);

    //let relative_start = relative_start + 1 + handshake_compression_method_length as usize;
    //let handshake_extension_length = u16::from_be_bytes([handshake_protocol[relative_start], handshake_protocol[relative_start + 1]]);
    //
    //
    //let mut relative_extension_start = relative_start + 2;
    //
    //println_green!("extension_length:{} current:{}, left over:{}", handshake_extension_length, relative_extension_start, length as usize - relative_extension_start);
    //
    //while relative_extension_start < handshake_extension_length as usize {
    //    let extension_type = u16::from_be_bytes([handshake_protocol[relative_extension_start], handshake_protocol[relative_extension_start + 1]]);
    //
    //    relative_extension_start += 2;
    //    let extension_length = u16::from_be_bytes([handshake_protocol[relative_extension_start], handshake_protocol[relative_extension_start + 1]]);
    //
    //    relative_extension_start += 2;
    //
    //    let extension_data: &[u8] = if (relative_extension_start + extension_length as usize) < handshake_extension_length as usize {
    //        &handshake_protocol[(relative_extension_start)..(relative_extension_start + (extension_length as usize))]
    //    } else {
    //        //relative_extension_start = handshake_extension_length as usize;
    //        print!("[TRUNCATED] ");
    //        &handshake_protocol[relative_extension_start..=handshake_extension_length as usize]
    //    };
    //    relative_extension_start += extension_length as usize;
    //    println_green!("Extension found! Type: {} || Length: {} || Extension_data: {:?}", extension_type,extension_length,extension_data);
    //}
}
