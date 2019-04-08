pub fn build_hello() -> Vec<u8> {
    vec![
        0, 0, 35, 40, 0, 0, 35, 40, 0, 0, 0, 0, 106, 109, 106, 120, 97, 107, 113, 118, 110, 121,
        103, 107, 116, 102, 119, 99, 111, 112, 103, 102, 98, 100, 122, 115, 115, 100, 114, 116,
        113, 114, 104, 118, 0, 0, 0, 28, 115, 116, 101, 108, 108, 97, 114, 45, 99, 111, 114, 101,
        45, 114, 117, 115, 116, 91, 97, 108, 112, 104, 97, 45, 48, 46, 48, 93, 0, 0, 45, 105, 0, 0,
        0, 0, 114, 102, 121, 110, 104, 105, 115, 100, 99, 122, 112, 112, 119, 105, 108, 121, 122,
        108, 102, 101, 107, 111, 102, 103, 109, 103, 106, 105, 98, 118, 110, 113, 115, 100, 105,
        102, 100, 111, 99, 110, 109, 113, 98, 114, 118, 116, 102, 99, 104, 120, 101, 118, 98, 103,
        99, 112, 105, 115, 110, 118, 97, 116, 100, 120, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0, 5, 1, 2,
        3, 4, 5, 0, 0, 0, 18, 17, 21, 19, 25, 24, 26, 9, 2, 28, 7, 22, 11, 30, 23, 3, 14, 4, 0, 6,
        16, 8, 29, 15, 20, 5, 10, 31, 27, 1, 13, 12,
    ]
}

pub fn build_auth_cert() -> Vec<u8> {
    vec![
        115, 100, 105, 102, 100, 111, 99, 110, 109, 113, 98, 114, 118, 116, 102, 99, 104, 120, 101,
        118, 98, 103, 99, 112, 105, 115, 110, 118, 97, 116, 100, 120, 0, 0, 0, 0, 0, 0, 0, 123, 0,
        0, 0, 5, 1, 2, 3, 4, 5, 0, 0, 0,
    ]
}

pub fn build_signature() -> Vec<u8> {
    vec![0, 0, 0, 5, 1, 2, 3, 4, 5, 0, 0, 0]
}

pub fn build_curve25519_public() -> Vec<u8> {
    vec![
        115, 100, 105, 102, 100, 111, 99, 110, 109, 113, 98, 114, 118, 116, 102, 99, 104, 120, 101,
        118, 98, 103, 99, 112, 105, 115, 110, 118, 97, 116, 100, 120,
    ]
}

pub fn build_public_key() -> Vec<u8> {
    vec![
        0, 0, 0, 0, 114, 102, 121, 110, 104, 105, 115, 100, 99, 122, 112, 112, 119, 105, 108, 121,
        122, 108, 102, 101, 107, 111, 102, 103, 109, 103, 106, 105, 98, 118, 110, 113,
    ]
}

pub fn build_operation_result_code() -> Vec<u8> {
    vec![255, 255, 255, 254]
}

pub fn build_stellar_message_hello() -> Vec<u8> {
    vec![
        0, 0, 0, 13, 0, 0, 35, 40, 0, 0, 35, 40, 0, 0, 0, 0, 106, 109, 106, 120, 97, 107, 113, 118,
        110, 121, 103, 107, 116, 102, 119, 99, 111, 112, 103, 102, 98, 100, 122, 115, 115, 100,
        114, 116, 113, 114, 104, 118, 0, 0, 0, 28, 115, 116, 101, 108, 108, 97, 114, 45, 99, 111,
        114, 101, 45, 114, 117, 115, 116, 91, 97, 108, 112, 104, 97, 45, 48, 46, 48, 93, 0, 0, 45,
        105, 0, 0, 0, 0, 114, 102, 121, 110, 104, 105, 115, 100, 99, 122, 112, 112, 119, 105, 108,
        121, 122, 108, 102, 101, 107, 111, 102, 103, 109, 103, 106, 105, 98, 118, 110, 113, 115,
        100, 105, 102, 100, 111, 99, 110, 109, 113, 98, 114, 118, 116, 102, 99, 104, 120, 101, 118,
        98, 103, 99, 112, 105, 115, 110, 118, 97, 116, 100, 120, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0,
        5, 1, 2, 3, 4, 5, 0, 0, 0, 18, 17, 21, 19, 25, 24, 26, 9, 2, 28, 7, 22, 11, 30, 23, 3, 14,
        4, 0, 6, 16, 8, 29, 15, 20, 5, 10, 31, 27, 1, 13, 12,
    ]
}

pub fn build_authenticated_message_v0() -> Vec<u8> {
    vec![
        0, 0, 0, 0, 0, 0, 16, 225, 0, 0, 0, 13, 0, 0, 35, 40, 0, 0, 35, 40, 0, 0, 0, 0, 106, 109,
        106, 120, 97, 107, 113, 118, 110, 121, 103, 107, 116, 102, 119, 99, 111, 112, 103, 102, 98,
        100, 122, 115, 115, 100, 114, 116, 113, 114, 104, 118, 0, 0, 0, 28, 115, 116, 101, 108,
        108, 97, 114, 45, 99, 111, 114, 101, 45, 114, 117, 115, 116, 91, 97, 108, 112, 104, 97, 45,
        48, 46, 48, 93, 0, 0, 45, 105, 0, 0, 0, 0, 114, 102, 121, 110, 104, 105, 115, 100, 99, 122,
        112, 112, 119, 105, 108, 121, 122, 108, 102, 101, 107, 111, 102, 103, 109, 103, 106, 105,
        98, 118, 110, 113, 115, 100, 105, 102, 100, 111, 99, 110, 109, 113, 98, 114, 118, 116, 102,
        99, 104, 120, 101, 118, 98, 103, 99, 112, 105, 115, 110, 118, 97, 116, 100, 120, 0, 0, 0,
        0, 0, 0, 0, 123, 0, 0, 0, 5, 1, 2, 3, 4, 5, 0, 0, 0, 18, 17, 21, 19, 25, 24, 26, 9, 2, 28,
        7, 22, 11, 30, 23, 3, 14, 4, 0, 6, 16, 8, 29, 15, 20, 5, 10, 31, 27, 1, 13, 12, 57, 57, 98,
        57, 102, 51, 101, 50, 55, 49, 100, 49, 102, 51, 56, 49, 56, 54, 54, 101, 97, 49, 100, 97,
        56, 101, 55, 54, 100, 52, 51, 48,
    ]
}

pub fn build_hmac_sha_256_mac() -> Vec<u8> {
    vec![
        57, 57, 98, 57, 102, 51, 101, 50, 55, 49, 100, 49, 102, 51, 56, 49, 56, 54, 54, 101, 97,
        49, 100, 97, 56, 101, 55, 54, 100, 52, 51, 48,
    ]
}

pub fn build_authenticated_message() -> Vec<u8> {
    vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 225, 0, 0, 0, 13, 0, 0, 35, 40, 0, 0, 35, 40, 0, 0, 0, 0,
        106, 109, 106, 120, 97, 107, 113, 118, 110, 121, 103, 107, 116, 102, 119, 99, 111, 112,
        103, 102, 98, 100, 122, 115, 115, 100, 114, 116, 113, 114, 104, 118, 0, 0, 0, 28, 115, 116,
        101, 108, 108, 97, 114, 45, 99, 111, 114, 101, 45, 114, 117, 115, 116, 91, 97, 108, 112,
        104, 97, 45, 48, 46, 48, 93, 0, 0, 45, 105, 0, 0, 0, 0, 114, 102, 121, 110, 104, 105, 115,
        100, 99, 122, 112, 112, 119, 105, 108, 121, 122, 108, 102, 101, 107, 111, 102, 103, 109,
        103, 106, 105, 98, 118, 110, 113, 115, 100, 105, 102, 100, 111, 99, 110, 109, 113, 98, 114,
        118, 116, 102, 99, 104, 120, 101, 118, 98, 103, 99, 112, 105, 115, 110, 118, 97, 116, 100,
        120, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0, 5, 1, 2, 3, 4, 5, 0, 0, 0, 18, 17, 21, 19, 25, 24,
        26, 9, 2, 28, 7, 22, 11, 30, 23, 3, 14, 4, 0, 6, 16, 8, 29, 15, 20, 5, 10, 31, 27, 1, 13,
        12, 57, 57, 98, 57, 102, 51, 101, 50, 55, 49, 100, 49, 102, 51, 56, 49, 56, 54, 54, 101,
        97, 49, 100, 97, 56, 101, 55, 54, 100, 52, 51, 48,
    ]
}