use crate::{ChExtension, ClientHello, eprintln_red, println_cyan, println_green};
use crate::HandshakeProtocol;

pub(crate) fn client_hello_handler(buffer: Vec<u8>) -> ClientHello {
    let mut client_hello = ClientHello {
        content_type: buffer[0],
        version: u16::from_be_bytes([buffer[1], buffer[2]]),
        length: u16::from_be_bytes([buffer[3], buffer[4]]),
        handshake: Box::new(HandshakeProtocol {
            handshake_type: buffer[5],
            length: u32::from_be_bytes([0x0, buffer[6], buffer[7], buffer[8]]),
            version: u16::from_be_bytes([buffer[9], buffer[10]]),
            random: vec![],
            session_id_length: buffer[43],
            legacy_session_id: vec![],
            cipher_suites_length: 0,
            cipher_suites: vec![],
            compression_methods_length: 0,
            legacy_compression_methods: vec![],
            extensions_length: 0,
            extensions: vec![],
        }),
    };

    // modify any non-directly assignable values :3
    if let ClientHello { ref mut handshake, .. } = &mut client_hello {
        {
            let mut temp_bytes: Vec<u8> = vec![];

            for byte in buffer[11..=42].iter() {
                temp_bytes.push(*byte);
            }

            handshake.random = temp_bytes;
        }
        println_cyan!("{:?}",  handshake.random);

        let relative_start = 44 + handshake.session_id_length as usize;

        {
            let mut temp_bytes: Vec<u8> = vec![];

            for byte in buffer[44..relative_start].iter() {
                temp_bytes.push(*byte);
            }

            handshake.legacy_session_id = temp_bytes;
        }

        println_cyan!("{:?}",  handshake.legacy_session_id);
        {
            println_cyan!("{} || {}", buffer[relative_start], buffer[relative_start + 1]);
            handshake.cipher_suites_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]); // todo: fix this mf

            for i in (relative_start + 2..=relative_start + (handshake.cipher_suites_length as usize)).step_by(2) {
                let cipher = u16::from_be_bytes([buffer[i], buffer[i + 1]]);
                handshake.cipher_suites.push(cipher);

                println_cyan!("supported cipher: {:02x}{:02x} || index: {}", buffer[i], buffer[i + 1], i+1);
            }
        }

        let relative_start = relative_start + 2 + (handshake.cipher_suites_length as usize);

        {
            let methods_length = buffer[relative_start] as usize;
            handshake.compression_methods_length = buffer[relative_start];
            println_cyan!("{} = {} is this good?", relative_start, handshake.compression_methods_length);

            for i in relative_start + 1..(relative_start + 1 + methods_length) {
                handshake.legacy_compression_methods.push(buffer[i]);

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

                relative_start += 2;
                let extension_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]);
                relative_start += 2;

                //let extension_data: &[u8] = if (relative_start + (extension_length as usize)) < (handshake.length as usize) {
                //    &buffer[relative_start..(relative_start + (extension_length as usize))]
                //} else {
                //    //relative_extension_start = handshake_extension_length as usize;
                //    print!("[TRUNCATED] ");
                //    &buffer[relative_start..=(handshake.length as usize)]
                //};

                let extension: Option<ChExtension> = match extension_type {
                    0 => {
                        println_cyan!("Server Name!");
                        let mut server_name_indication_extension: Vec<(u16, u8, u16, Vec<u8>)> = vec![];

                        let mut extensions_start = relative_start;
                        let mut current_length: usize = 0;

                        while current_length < extension_length as usize {
                            let server_name_list_length = u16::from_be_bytes([buffer[extensions_start], buffer[extensions_start + 1]]);
                            let server_name_type = buffer[extensions_start + 2];
                            let server_name_length = u16::from_be_bytes([buffer[extensions_start + 3], buffer[extensions_start + 4]]);

                            let mut server_name: Vec<u8> = vec![];

                            for byte in buffer[(extensions_start + 5)..=(extensions_start + 4 + (server_name_length as usize))].iter() {
                                server_name.push(*byte);
                            }

                            extensions_start = extensions_start + (server_name_length as usize) + 5;
                            current_length = current_length + (server_name_length as usize) + 5;
                            server_name_indication_extension.push((server_name_list_length, server_name_type, server_name_length, server_name));
                        }

                        Some(ChExtension::ServerName {
                            extension_type,
                            length: extension_length,
                            server_name_indication_extension,
                        })
                    }
                    1 => {
                        eprintln_red!("Max Fragment Length Isn't Supported!");
                        None
                    }
                    5 => {
                        eprintln_red!("Status Request Isn't Supported!");
                        None
                    }
                    10 => {
                        println_cyan!("Supported Groups!");

                        let mut supported_groups: Vec<u16> = vec![];

                        let supported_groups_list_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]);

                        for i in (0..(supported_groups_list_length as usize)).step_by(2) {
                            let group = u16::from_be_bytes([buffer[relative_start + 2 + i], buffer[relative_start + 3 + i]]);

                            supported_groups.push(group);
                        }

                        Some(ChExtension::SupportedGroups {
                            extension_type,
                            length: extension_length,
                            supported_groups_list_length,
                            supported_groups,
                        })
                    }
                    13 => {
                        println_cyan!("Signature algorithms!");
                        let mut signature_hash_algorithms: Vec<(u8, u8)> = vec![];

                        let signature_hash_algorithms_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]);

                        for i in 0..=(signature_hash_algorithms_length as usize / 2) {
                            let hash = buffer[relative_start + 2 + i];
                            let signature = buffer[relative_start + 3 + i];

                            signature_hash_algorithms.push((hash, signature));
                        }

                        Some(ChExtension::SignatureAlgorithms {
                            extension_type,
                            length: extension_length,
                            signature_hash_algorithms_length: 0,
                            signature_hash_algorithms,
                        })
                    }
                    14 => {
                        eprintln_red!("Use Srtp Isn't Supported!");
                        None
                    }
                    15 => {
                        eprintln_red!("Heartbeat Isn't Supported!");
                        None
                    }
                    16 => {
                        println_cyan!("Application Layer Protocol Negotiation!");

                        let mut alpn_protocol: Vec<(u8, String)> = vec![];
                        let alpn_extension_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]);

                        let mut extensions_start = relative_start + 2;
                        let mut current_length: usize = 0;

                        while current_length < alpn_extension_length as usize {
                            let alpn_string_length = buffer[extensions_start];
                            let alpn_next_protocol: String = String::from_utf8(buffer[(extensions_start + 1)..((extensions_start + 1) + alpn_string_length as usize)].to_owned()).unwrap_or_default();

                            extensions_start = (extensions_start + 1) + alpn_string_length as usize;
                            current_length = (current_length + 1) + (alpn_string_length as usize);
                            alpn_protocol.push((alpn_string_length, alpn_next_protocol));
                        }

                        Some(ChExtension::ApplicationLayerProtocolNegotiation {
                            extension_type,
                            length: extension_length,
                            alpn_extension_length,
                            alpn_protocol,
                        })
                    }
                    18 => {
                        eprintln_red!("Signed Certificate Timestamp Isn't Supported!");
                        None
                    }
                    19 => {
                        eprintln_red!("Client Certificate Type Isn't Supported!");
                        None
                    }
                    20 => {
                        eprintln_red!("Server Certificate Type Isn't Supported!");
                        None
                    }
                    21 => {
                        eprintln_red!("Padding Isn't Supported!");
                        None
                    }
                    42 => {
                        eprintln_red!("Early Data Isn't Supported!");
                        None
                    }
                    43 => {
                        println_cyan!("Supported Versions!");

                        let mut supported_versions: Vec<u16> = vec![];
                        let supported_versions_length = buffer[relative_start];

                        for i in (0..(supported_versions_length as usize)).step_by(2) {
                            let version = u16::from_be_bytes([buffer[relative_start + 1 + i], buffer[relative_start + 2 + i]]);

                            supported_versions.push(version);
                        }

                        Some(ChExtension::SupportedVersions {
                            extension_type,
                            length: extension_length,
                            supported_versions_length,
                            supported_versions,
                        })
                    }
                    44 => {
                        eprintln_red!("Cookie Isn't Supported!");
                        None
                    }
                    45 => {
                        println_cyan!("Psk Key Exchange Modes!");
                        let psk_key_exchange_modes_length = buffer[relative_start];
                        let psk_key_exchange_mode = buffer[(relative_start + 1)..((relative_start + 1) + psk_key_exchange_modes_length as usize)].to_owned();

                        Some(ChExtension::PskKeyExchangeModes {
                            extension_type,
                            length: extension_length,
                            psk_key_exchange_modes_length,
                            psk_key_exchange_mode,
                        })
                    }
                    47 => {
                        eprintln_red!("Certificate Authorities Isn't Supported!");
                        None
                    }
                    48 => {
                        eprintln_red!("Oid Filters Isn't Supported!");
                        None
                    }
                    49 => {
                        eprintln_red!("Post Handshake Auth Isn't Supported!");
                        None
                    }
                    50 => {
                        eprintln_red!("Signature Algorithms Cert Isn't Supported!");
                        None
                    }
                    51 => {
                        println_cyan!("Key Share!");

                        let mut key_share_extensions: Vec<(u16, u16, Vec<u8>)> = vec![];
                        let client_key_share_length = u16::from_be_bytes([buffer[relative_start], buffer[relative_start + 1]]);

                        let mut extensions_start = relative_start + 2;
                        let mut current_length: usize = 0;

                        while current_length < client_key_share_length as usize {
                            let group = u16::from_be_bytes([buffer[extensions_start], buffer[extensions_start + 1]]);
                            let key_exchange_length = u16::from_be_bytes([buffer[extensions_start + 2], buffer[extensions_start + 3]]);

                            let mut key_exchange: Vec<u8> = vec![];

                            for byte in buffer[(extensions_start + 4)..((extensions_start + 4) + key_exchange_length as usize)].iter() {
                                key_exchange.push(*byte);
                            }

                            extensions_start = (extensions_start + 3) + (key_exchange_length as usize) + 1;
                            current_length = (current_length + 3) + (key_exchange_length as usize) + 1;
                            key_share_extensions.push((group, key_exchange_length, key_exchange));
                        }

                        Some(ChExtension::KeyShare {
                            extension_type,
                            length: extension_length,
                            client_key_share_length,
                            key_share_extensions,
                        })
                    }
                    _ => {
                        eprintln_red!("Unsupported extension! || Type: {}", extension_type);
                        None
                    }
                };

                // if there is an extension, then push it :3
                if let Some(value) = extension {
                    handshake.extensions.push(value);
                }

                relative_start += extension_length as usize;
                //println_green!("Extension found! Type: {} || Length: {} || Extension_data: {:?}", extension_type,extension_length,extension_data);
            }
            //println_cyan!("{:?}", handshake);
        }

        println_green!("{:?}", &handshake);
    }
    return client_hello;
}
