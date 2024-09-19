// SPDX-FileCopyrightText: 2023 Greenbone AG
//
// SPDX-License-Identifier: GPL-2.0-or-later WITH x11vnc-openssl-exception

#[cfg(test)]
mod test {

    use crate::nasl::syntax::parse;

    #[test]
    fn change_to_peek() {
        let code = r###"
send_packet( udp, pcap_active:FALSE ) x 200;
        "###;
        for stmt in parse(code) {
            stmt.unwrap();
        }
    }

    #[test]
    fn stack_overflow() {
        let code = r###"
req = raw_string(0x00, 0x00, 0x03, 0x14, 0x08, 0x14, 0xff, 0x9f,
                 0xde, 0x5d, 0x5f, 0xb3, 0x07, 0x8f, 0x49, 0xa7,
                 0x79, 0x6a, 0x03, 0x3d, 0xaf, 0x55, 0x00, 0x00,
                 0x00, 0x7e, 0x64, 0x69, 0x66, 0x66, 0x69, 0x65,
                 0x2d, 0x68, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e,
                 0x2d, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x2d, 0x65,
                 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2d,
                 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x2c, 0x64,
                 0x69, 0x66, 0x66, 0x69, 0x65, 0x2d, 0x68, 0x65,
                 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x2d, 0x67, 0x72,
                 0x6f, 0x75, 0x70, 0x2d, 0x65, 0x78, 0x63, 0x68,
                 0x61, 0x6e, 0x67, 0x65, 0x2d, 0x73, 0x68, 0x61,
                 0x31, 0x2c, 0x64, 0x69, 0x66, 0x66, 0x69, 0x65,
                 0x2d, 0x68, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e,
                 0x2d, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x31, 0x34,
                 0x2d, 0x73, 0x68, 0x61, 0x31, 0x2c, 0x64, 0x69,
                 0x66, 0x66, 0x69, 0x65, 0x2d, 0x68, 0x65, 0x6c,
                 0x6c, 0x6d, 0x61, 0x6e, 0x2d, 0x67, 0x72, 0x6f,
                 0x75, 0x70, 0x31, 0x2d, 0x73, 0x68, 0x61, 0x31,
                 0x00, 0x00, 0x00, 0x0f, 0x73, 0x73, 0x68, 0x2d,
                 0x72, 0x73, 0x61, 0x2c, 0x73, 0x73, 0x68, 0x2d,
                 0x64, 0x73, 0x73, 0x00, 0x00, 0x00, 0x9d, 0x61,
                 0x65, 0x73, 0x31, 0x32, 0x38, 0x2d, 0x63, 0x62,
                 0x63, 0x2c, 0x33, 0x64, 0x65, 0x73, 0x2d, 0x63,
                 0x62, 0x63, 0x2c, 0x62, 0x6c, 0x6f, 0x77, 0x66,
                 0x69, 0x73, 0x68, 0x2d, 0x63, 0x62, 0x63, 0x2c,
                 0x63, 0x61, 0x73, 0x74, 0x31, 0x32, 0x38, 0x2d,
                 0x63, 0x62, 0x63, 0x2c, 0x61, 0x72, 0x63, 0x66,
                 0x6f, 0x75, 0x72, 0x31, 0x32, 0x38, 0x2c, 0x61,
                 0x72, 0x63, 0x66, 0x6f, 0x75, 0x72, 0x32, 0x35,
                 0x36, 0x2c, 0x61, 0x72, 0x63, 0x66, 0x6f, 0x75,
                 0x72, 0x2c, 0x61, 0x65, 0x73, 0x31, 0x39, 0x32,
                 0x2d, 0x63, 0x62, 0x63, 0x2c, 0x61, 0x65, 0x73,
                 0x32, 0x35, 0x36, 0x2d, 0x63, 0x62, 0x63, 0x2c,
                 0x72, 0x69, 0x6a, 0x6e, 0x64, 0x61, 0x65, 0x6c,
                 0x2d, 0x63, 0x62, 0x63, 0x40, 0x6c, 0x79, 0x73,
                 0x61, 0x74, 0x6f, 0x72, 0x2e, 0x6c, 0x69, 0x75,
                 0x2e, 0x73, 0x65, 0x2c, 0x61, 0x65, 0x73, 0x31,
                 0x32, 0x38, 0x2d, 0x63, 0x74, 0x72, 0x2c, 0x61,
                 0x65, 0x73, 0x31, 0x39, 0x32, 0x2d, 0x63, 0x74,
                 0x72, 0x2c, 0x61, 0x65, 0x73, 0x32, 0x35, 0x36,
                 0x2d, 0x63, 0x74, 0x72, 0x00, 0x00, 0x00, 0x9d,
                 0x61, 0x65, 0x73, 0x31, 0x32, 0x38, 0x2d, 0x63,
                 0x62, 0x63, 0x2c, 0x33, 0x64, 0x65, 0x73, 0x2d,
                 0x63, 0x62, 0x63, 0x2c, 0x62, 0x6c, 0x6f, 0x77,
                 0x66, 0x69, 0x73, 0x68, 0x2d, 0x63, 0x62, 0x63,
                 0x2c, 0x63, 0x61, 0x73, 0x74, 0x31, 0x32, 0x38,
                 0x2d, 0x63, 0x62, 0x63, 0x2c, 0x61, 0x72, 0x63,
                 0x66, 0x6f, 0x75, 0x72, 0x31, 0x32, 0x38, 0x2c,
                 0x61, 0x72, 0x63, 0x66, 0x6f, 0x75, 0x72, 0x32,
                 0x35, 0x36, 0x2c, 0x61, 0x72, 0x63, 0x66, 0x6f,
                 0x75, 0x72, 0x2c, 0x61, 0x65, 0x73, 0x31, 0x39,
                 0x32, 0x2d, 0x63, 0x62, 0x63, 0x2c, 0x61, 0x65,
                 0x73, 0x32, 0x35, 0x36, 0x2d, 0x63, 0x62, 0x63,
                 0x2c, 0x72, 0x69, 0x6a, 0x6e, 0x64, 0x61, 0x65,
                 0x6c, 0x2d, 0x63, 0x62, 0x63, 0x40, 0x6c, 0x79,
                 0x73, 0x61, 0x74, 0x6f, 0x72, 0x2e, 0x6c, 0x69,
                 0x75, 0x2e, 0x73, 0x65, 0x2c, 0x61, 0x65, 0x73,
                 0x31, 0x32, 0x38, 0x2d, 0x63, 0x74, 0x72, 0x2c,
                 0x61, 0x65, 0x73, 0x31, 0x39, 0x32, 0x2d, 0x63,
                 0x74, 0x72, 0x2c, 0x61, 0x65, 0x73, 0x32, 0x35,
                 0x36, 0x2d, 0x63, 0x74, 0x72, 0x00, 0x00, 0x00,
                 0x69, 0x68, 0x6d, 0x61, 0x63, 0x2d, 0x6d, 0x64,
                 0x35, 0x2c, 0x68, 0x6d, 0x61, 0x63, 0x2d, 0x73,
                 0x68, 0x61, 0x31, 0x2c, 0x75, 0x6d, 0x61, 0x63,
                 0x2d, 0x36, 0x34, 0x40, 0x6f, 0x70, 0x65, 0x6e,
                 0x73, 0x73, 0x68, 0x2e, 0x63, 0x6f, 0x6d, 0x2c,
                 0x68, 0x6d, 0x61, 0x63, 0x2d, 0x72, 0x69, 0x70,
                 0x65, 0x6d, 0x64, 0x31, 0x36, 0x30, 0x2c, 0x68,
                 0x6d, 0x61, 0x63, 0x2d, 0x72, 0x69, 0x70, 0x65,
                 0x6d, 0x64, 0x31, 0x36, 0x30, 0x40, 0x6f, 0x70,
                 0x65, 0x6e, 0x73, 0x73, 0x68, 0x2e, 0x63, 0x6f,
                 0x6d, 0x2c, 0x68, 0x6d, 0x61, 0x63, 0x2d, 0x73,
                 0x68, 0x61, 0x31, 0x2d, 0x39, 0x36, 0x2c, 0x68,
                 0x6d, 0x61, 0x63, 0x2d, 0x6d, 0x64, 0x35, 0x2d,
                 0x39, 0x36, 0x00, 0x00, 0x00, 0x69, 0x68, 0x6d,
                 0x61, 0x63, 0x2d, 0x6d, 0x64, 0x35, 0x2c, 0x68,
                 0x6d, 0x61, 0x63, 0x2d, 0x73, 0x68, 0x61, 0x31,
                 0x2c, 0x75, 0x6d, 0x61, 0x63, 0x2d, 0x36, 0x34,
                 0x40, 0x6f, 0x70, 0x65, 0x6e, 0x73, 0x73, 0x68,
                 0x2e, 0x63, 0x6f, 0x6d, 0x2c, 0x68, 0x6d, 0x61,
                 0x63, 0x2d, 0x72, 0x69, 0x70, 0x65, 0x6d, 0x64,
                 0x31, 0x36, 0x30, 0x2c, 0x68, 0x6d, 0x61, 0x63,
                 0x2d, 0x72, 0x69, 0x70, 0x65, 0x6d, 0x64, 0x31,
                 0x36, 0x30, 0x40, 0x6f, 0x70, 0x65, 0x6e, 0x73,
                 0x73, 0x68, 0x2e, 0x63, 0x6f, 0x6d, 0x2c, 0x68,
                 0x6d, 0x61, 0x63, 0x2d, 0x73, 0x68, 0x61, 0x31,
                 0x2d, 0x39, 0x36, 0x2c, 0x68, 0x6d, 0x61, 0x63,
                 0x2d, 0x6d, 0x64, 0x35, 0x2d, 0x39, 0x36, 0x00,

                 ##3rd byte in this next line causes crash
                 0x00, 0x00, 0x28, 0x7a, 0x6c, 0x69, 0x62, 0x40,
                 0x6f, 0x70, 0x65, 0x6e, 0x73, 0x73, 0x68, 0x2e,
                 0x63, 0x6f, 0x6d, 0x2c, 0x7a, 0x6c, 0x69, 0x62,
                 0x2c, 0x6e, 0x6f, 0x6e, 0x65, 0x00, 0x00, 0x00,
                 0x1a, 0x7a, 0x6c, 0x69, 0x62, 0x40, 0x6f, 0x70,
                 0x65, 0x6e, 0x73, 0x73, 0x68, 0x2e, 0x63, 0x6f,
                 0x6d, 0x2c, 0x7a, 0x6c, 0x69, 0x62, 0x2c, 0x6e,
                 0x6f, 0x6e, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x0a);
        "###;
        parse(code).next();
    }

    #[test]
    fn jsp_example() {
        let code = r#"
  gms_path = gms_path + 'webapps\\appliance\\';
jsp = '<% out.println( "' + jsp_print  + '" ); %>';
        "#;
        for x in parse(code) {
            x.unwrap();
        }
    }

    #[test]
    fn unexpected_assign() {
        let code = r###"
while(y = recv(socket:soc, length:1024)) {
  buf1 += y;
}
        "###;
        for x in parse(code) {
            x.unwrap();
        }
    }

    #[test]
    fn unexpected_noop() {
        let code = r###"
  if( ! version || version == '' ) return;
        "###;
        for x in parse(code) {
            x.unwrap();
        }
    }

    #[test]
    fn unexpected_equal_operator() {
        let code = r###"
# Message Server runs on ports 36xx or 39xx
if (port < 3600 || port >= 3700)
  if (port < 3900 || port >= 4000)
    exit(0);

soc = open_sock_tcp(port);

        "###;
        for x in parse(code) {
            x.unwrap();
        }
    }

    #[test]
    fn unexpected_plusplus() {
        let code = r###"
        cookie_jar[this_cookie]++;
        "###;
        for x in parse(code) {
            x.unwrap();
        }
    }
}