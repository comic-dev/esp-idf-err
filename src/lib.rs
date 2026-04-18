use core::convert::TryFrom;
use core::num::NonZeroI32;

#[cfg(any(target_arch = "xtensa", target_arch = "riscv32"))]
pub use esp_idf_sys::esp_err_t;

#[cfg(not(any(target_arch = "xtensa", target_arch = "riscv32")))]
#[allow(non_camel_case_types)]
pub type esp_err_t = i32;

/* ESP_IDF_ERROR_CODE_LIST */
pub const ESP_IDF_ERROR_CODES: [i32; 213] = [-1, 0x101, 0x102, 0x103, 0x104, 0x105, 0x106, 0x107, 0x108, 0x109, 0x10a, 0x10b, 0x10c, 0x10d, 0x1101, 0x1102, 0x1103, 0x1104, 0x1105, 0x1106, 0x1107, 0x1108, 0x1109, 0x110a, 0x110b, 0x110c, 0x110d, 0x110e, 0x110f, 0x1110, 0x1111, 0x1112, 0x1113, 0x1114, 0x1115, 0x1116, 0x1117, 0x1118, 0x1119, 0x1201, 0x1202, 0x1203, 0x1204, 0x1205, 0x1501, 0x1502, 0x1503, 0x1504, 0x1505, 0x1506, 0x1601, 0x1602, 0x1603, 0x1604, 0x1605, 0x1606, 0x2001, 0x2002, 0x3001, 0x3002, 0x3003, 0x3004, 0x3005, 0x3006, 0x3007, 0x3008, 0x3009, 0x300a, 0x300b, 0x300c, 0x300d, 0x300e, 0x300f, 0x3012, 0x3013, 0x3014, 0x3015, 0x3016, 0x3017, 0x3018, 0x3019, 0x301a, 0x301b, 0x301c, 0x3033, 0x3034, 0x3035, 0x3064, 0x3065, 0x3066, 0x3067, 0x3068, 0x3069, 0x306a, 0x306b, 0x306c, 0x306d, 0x3097, 0x3098, 0x3099, 0x309a, 0x309b, 0x309c, 0x4001, 0x4002, 0x4003, 0x4004, 0x4005, 0x4006, 0x4007, 0x4008, 0x4009, 0x400a, 0x400b, 0x400c, 0x400d, 0x400e, 0x400f, 0x4010, 0x4011, 0x4012, 0x4013, 0x4014, 0x4015, 0x4016, 0x4017, 0x4018, 0x4019, 0x401a, 0x5001, 0x5002, 0x5003, 0x5004, 0x5005, 0x5006, 0x5007, 0x5008, 0x5009, 0x500a, 0x500b, 0x500c, 0x500d, 0x500e, 0x6001, 0x6002, 0x6003, 0x6004, 0x6005, 0x6006, 0x7001, 0x7002, 0x7003, 0x7004, 0x7005, 0x7006, 0x7007, 0x7008, 0x7009, 0x700a, 0x700b, 0x700c, 0x8001, 0x8002, 0x8003, 0x8004, 0x8005, 0x8006, 0x8007, 0x8008, 0x8009, 0x8010, 0x8011, 0x8012, 0x8013, 0x8014, 0x8015, 0x8016, 0x8017, 0x8018, 0x8019, 0x801a, 0x801b, 0x801c, 0x801d, 0x9001, 0xb001, 0xb002, 0xb003, 0xb004, 0xb005, 0xb006, 0xb007, 0xb008, 0xc001, 0xc002, 0xc004, 0xc005, 0xd001, 0xd002, 0xd003, 0xd004, 0xd005, 0xd006, 0xd007, 0xd008, 0xe001, 0xe002, 0xe003, 0xe004, 0xf001, 0xf002, 0xf003, 0xf004];
/* ESP_IDF_ERROR_CODE_LIST */

pub enum IdfErrorError {
    SuccessCodePassed,
    InvalidErrorCode(NonZeroI32)
}

impl core::fmt::Debug for IdfErrorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
           IdfErrorError::SuccessCodePassed => {
            f.write_str("recieved an esp-idf success exit code (0) when an error code was expected")
           },
           IdfErrorError::InvalidErrorCode(code) => {
            f.write_fmt(format_args!("recieved an invalid esp-idf error code: {}", code.get()))
           }
        }
    }
}

impl core::fmt::Display for IdfErrorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            IdfErrorError::InvalidErrorCode(code) => {
                f.write_fmt(format_args!("invalid error code: {:?}", code.to_owned()))
            },
            IdfErrorError::SuccessCodePassed => {
                f.write_fmt(format_args!("supplied exit code was ESP_OK"))
            }
        }
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct IdfError(NonZeroI32);

impl core::fmt::Display for IdfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.error_info())
    }
}

impl core::fmt::Debug for IdfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.error_info())
    }
}

macro_rules! impl_try_from_for_esp_idf_error {
    ($($numtype: ty)*) => ($(
        impl TryFrom<$numtype> for IdfError {
            type Error = IdfErrorError;
            fn try_from(code: $numtype) -> Result<Self, Self::Error> {
                let nz = NonZeroI32::new(code as i32)
                    .ok_or(IdfErrorError::SuccessCodePassed)?;
                match code {
                    0 => Err(IdfErrorError::SuccessCodePassed),
                    /* ESP_IDF_ERROR_CODE_MATCH_ARM */
                    -1 | 0x101 | 0x102 | 0x103 | 0x104 | 0x105 | 0x106 | 0x107 | 0x108 | 0x109 | 0x10a | 0x10b | 0x10c | 0x10d | 0x1101 | 0x1102 | 0x1103 | 0x1104 | 0x1105 | 0x1106 | 0x1107 | 0x1108 | 0x1109 | 0x110a | 0x110b | 0x110c | 0x110d | 0x110e | 0x110f | 0x1110 | 0x1111 | 0x1112 | 0x1113 | 0x1114 | 0x1115 | 0x1116 | 0x1117 | 0x1118 | 0x1119 | 0x1201 | 0x1202 | 0x1203 | 0x1204 | 0x1205 | 0x1501 | 0x1502 | 0x1503 | 0x1504 | 0x1505 | 0x1506 | 0x1601 | 0x1602 | 0x1603 | 0x1604 | 0x1605 | 0x1606 | 0x2001 | 0x2002 | 0x3001 | 0x3002 | 0x3003 | 0x3004 | 0x3005 | 0x3006 | 0x3007 | 0x3008 | 0x3009 | 0x300a | 0x300b | 0x300c | 0x300d | 0x300e | 0x300f | 0x3012 | 0x3013 | 0x3014 | 0x3015 | 0x3016 | 0x3017 | 0x3018 | 0x3019 | 0x301a | 0x301b | 0x301c | 0x3033 | 0x3034 | 0x3035 | 0x3064 | 0x3065 | 0x3066 | 0x3067 | 0x3068 | 0x3069 | 0x306a | 0x306b | 0x306c | 0x306d | 0x3097 | 0x3098 | 0x3099 | 0x309a | 0x309b | 0x309c | 0x4001 | 0x4002 | 0x4003 | 0x4004 | 0x4005 | 0x4006 | 0x4007 | 0x4008 | 0x4009 | 0x400a | 0x400b | 0x400c | 0x400d | 0x400e | 0x400f | 0x4010 | 0x4011 | 0x4012 | 0x4013 | 0x4014 | 0x4015 | 0x4016 | 0x4017 | 0x4018 | 0x4019 | 0x401a | 0x5001 | 0x5002 | 0x5003 | 0x5004 | 0x5005 | 0x5006 | 0x5007 | 0x5008 | 0x5009 | 0x500a | 0x500b | 0x500c | 0x500d | 0x500e | 0x6001 | 0x6002 | 0x6003 | 0x6004 | 0x6005 | 0x6006 | 0x7001 | 0x7002 | 0x7003 | 0x7004 | 0x7005 | 0x7006 | 0x7007 | 0x7008 | 0x7009 | 0x700a | 0x700b | 0x700c | 0x8001 | 0x8002 | 0x8003 | 0x8004 | 0x8005 | 0x8006 | 0x8007 | 0x8008 | 0x8009 | 0x8010 | 0x8011 | 0x8012 | 0x8013 | 0x8014 | 0x8015 | 0x8016 | 0x8017 | 0x8018 | 0x8019 | 0x801a | 0x801b | 0x801c | 0x801d | 0x9001 | 0xb001 | 0xb002 | 0xb003 | 0xb004 | 0xb005 | 0xb006 | 0xb007 | 0xb008 | 0xc001 | 0xc002 | 0xc004 | 0xc005 | 0xd001 | 0xd002 | 0xd003 | 0xd004 | 0xd005 | 0xd006 | 0xd007 | 0xd008 | 0xe001 | 0xe002 | 0xe003 | 0xe004 | 0xf001 | 0xf002 | 0xf003 | 0xf004
                    /* ESP_IDF_ERROR_CODE_MATCH_ARM */
                        => Ok(IdfError(nz)),
                    _ => Err(IdfErrorError::InvalidErrorCode(nz))
                }
            }
        }
    )*);
}

impl_try_from_for_esp_idf_error! { esp_err_t }

impl IdfError {
    pub const fn error_name(&self) -> &'static str {
        match self.0.get() {
            /* ESP_IDF_ERROR_NAME_LOOKUP_TABLE */
            -1 => "ESP_FAIL",
            0x101 => "ESP_ERR_NO_MEM",
            0x102 => "ESP_ERR_INVALID_ARG",
            0x103 => "ESP_ERR_INVALID_STATE",
            0x104 => "ESP_ERR_INVALID_SIZE",
            0x105 => "ESP_ERR_NOT_FOUND",
            0x106 => "ESP_ERR_NOT_SUPPORTED",
            0x107 => "ESP_ERR_TIMEOUT",
            0x108 => "ESP_ERR_INVALID_RESPONSE",
            0x109 => "ESP_ERR_INVALID_CRC",
            0x10a => "ESP_ERR_INVALID_VERSION",
            0x10b => "ESP_ERR_INVALID_MAC",
            0x10c => "ESP_ERR_NOT_FINISHED",
            0x10d => "ESP_ERR_NOT_ALLOWED",
            0x1101 => "ESP_ERR_NVS_NOT_INITIALIZED",
            0x1102 => "ESP_ERR_NVS_NOT_FOUND",
            0x1103 => "ESP_ERR_NVS_TYPE_MISMATCH",
            0x1104 => "ESP_ERR_NVS_READ_ONLY",
            0x1105 => "ESP_ERR_NVS_NOT_ENOUGH_SPACE",
            0x1106 => "ESP_ERR_NVS_INVALID_NAME",
            0x1107 => "ESP_ERR_NVS_INVALID_HANDLE",
            0x1108 => "ESP_ERR_NVS_REMOVE_FAILED",
            0x1109 => "ESP_ERR_NVS_KEY_TOO_LONG",
            0x110a => "ESP_ERR_NVS_PAGE_FULL",
            0x110b => "ESP_ERR_NVS_INVALID_STATE",
            0x110c => "ESP_ERR_NVS_INVALID_LENGTH",
            0x110d => "ESP_ERR_NVS_NO_FREE_PAGES",
            0x110e => "ESP_ERR_NVS_VALUE_TOO_LONG",
            0x110f => "ESP_ERR_NVS_PART_NOT_FOUND",
            0x1110 => "ESP_ERR_NVS_NEW_VERSION_FOUND",
            0x1111 => "ESP_ERR_NVS_XTS_ENCR_FAILED",
            0x1112 => "ESP_ERR_NVS_XTS_DECR_FAILED",
            0x1113 => "ESP_ERR_NVS_XTS_CFG_FAILED",
            0x1114 => "ESP_ERR_NVS_XTS_CFG_NOT_FOUND",
            0x1115 => "ESP_ERR_NVS_ENCR_NOT_SUPPORTED",
            0x1116 => "ESP_ERR_NVS_KEYS_NOT_INITIALIZED",
            0x1117 => "ESP_ERR_NVS_CORRUPT_KEY_PART",
            0x1118 => "ESP_ERR_NVS_CONTENT_DIFFERS",
            0x1119 => "ESP_ERR_NVS_WRONG_ENCRYPTION",
            0x1201 => "ESP_ERR_ULP_SIZE_TOO_BIG",
            0x1202 => "ESP_ERR_ULP_INVALID_LOAD_ADDR",
            0x1203 => "ESP_ERR_ULP_DUPLICATE_LABEL",
            0x1204 => "ESP_ERR_ULP_UNDEFINED_LABEL",
            0x1205 => "ESP_ERR_ULP_BRANCH_OUT_OF_RANGE",
            0x1501 => "ESP_ERR_OTA_PARTITION_CONFLICT",
            0x1502 => "ESP_ERR_OTA_SELECT_INFO_INVALID",
            0x1503 => "ESP_ERR_OTA_VALIDATE_FAILED",
            0x1504 => "ESP_ERR_OTA_SMALL_SEC_VER",
            0x1505 => "ESP_ERR_OTA_ROLLBACK_FAILED",
            0x1506 => "ESP_ERR_OTA_ROLLBACK_INVALID_STATE",
            0x1601 => "ESP_OK_EFUSE_CNT",
            0x1602 => "ESP_ERR_EFUSE_CNT_IS_FULL",
            0x1603 => "ESP_ERR_EFUSE_REPEATED_PROG",
            0x1604 => "ESP_ERR_CODING",
            0x1605 => "ESP_ERR_NOT_ENOUGH_UNUSED_KEY_BLOCKS",
            0x1606 => "ESP_ERR_DAMAGED_READING",
            0x2001 => "ESP_ERR_IMAGE_FLASH_FAIL",
            0x2002 => "ESP_ERR_IMAGE_INVALID",
            0x3001 => "ESP_ERR_WIFI_NOT_INIT",
            0x3002 => "ESP_ERR_WIFI_NOT_STARTED",
            0x3003 => "ESP_ERR_WIFI_NOT_STOPPED",
            0x3004 => "ESP_ERR_WIFI_IF",
            0x3005 => "ESP_ERR_WIFI_MODE",
            0x3006 => "ESP_ERR_WIFI_STATE",
            0x3007 => "ESP_ERR_WIFI_CONN",
            0x3008 => "ESP_ERR_WIFI_NVS",
            0x3009 => "ESP_ERR_WIFI_MAC",
            0x300a => "ESP_ERR_WIFI_SSID",
            0x300b => "ESP_ERR_WIFI_PASSWORD",
            0x300c => "ESP_ERR_WIFI_TIMEOUT",
            0x300d => "ESP_ERR_WIFI_WAKE_FAIL",
            0x300e => "ESP_ERR_WIFI_WOULD_BLOCK",
            0x300f => "ESP_ERR_WIFI_NOT_CONNECT",
            0x3012 => "ESP_ERR_WIFI_POST",
            0x3013 => "ESP_ERR_WIFI_INIT_STATE",
            0x3014 => "ESP_ERR_WIFI_STOP_STATE",
            0x3015 => "ESP_ERR_WIFI_NOT_ASSOC",
            0x3016 => "ESP_ERR_WIFI_TX_DISALLOW",
            0x3017 => "ESP_ERR_WIFI_TWT_FULL",
            0x3018 => "ESP_ERR_WIFI_TWT_SETUP_TIMEOUT",
            0x3019 => "ESP_ERR_WIFI_TWT_SETUP_TXFAIL",
            0x301a => "ESP_ERR_WIFI_TWT_SETUP_REJECT",
            0x301b => "ESP_ERR_WIFI_DISCARD",
            0x301c => "ESP_ERR_WIFI_ROC_IN_PROGRESS",
            0x3033 => "ESP_ERR_WIFI_REGISTRAR",
            0x3034 => "ESP_ERR_WIFI_WPS_TYPE",
            0x3035 => "ESP_ERR_WIFI_WPS_SM",
            0x3064 => "ESP_ERR_ESPNOW_BASE",
            0x3065 => "ESP_ERR_ESPNOW_NOT_INIT",
            0x3066 => "ESP_ERR_ESPNOW_ARG",
            0x3067 => "ESP_ERR_ESPNOW_NO_MEM",
            0x3068 => "ESP_ERR_ESPNOW_FULL",
            0x3069 => "ESP_ERR_ESPNOW_NOT_FOUND",
            0x306a => "ESP_ERR_ESPNOW_INTERNAL",
            0x306b => "ESP_ERR_ESPNOW_EXIST",
            0x306c => "ESP_ERR_ESPNOW_IF",
            0x306d => "ESP_ERR_ESPNOW_CHAN",
            0x3097 => "ESP_ERR_DPP_FAILURE",
            0x3098 => "ESP_ERR_DPP_TX_FAILURE",
            0x3099 => "ESP_ERR_DPP_INVALID_ATTR",
            0x309a => "ESP_ERR_DPP_AUTH_TIMEOUT",
            0x309b => "ESP_ERR_DPP_INVALID_LIST",
            0x309c => "ESP_ERR_DPP_CONF_TIMEOUT",
            0x4001 => "ESP_ERR_MESH_WIFI_NOT_START",
            0x4002 => "ESP_ERR_MESH_NOT_INIT",
            0x4003 => "ESP_ERR_MESH_NOT_CONFIG",
            0x4004 => "ESP_ERR_MESH_NOT_START",
            0x4005 => "ESP_ERR_MESH_NOT_SUPPORT",
            0x4006 => "ESP_ERR_MESH_NOT_ALLOWED",
            0x4007 => "ESP_ERR_MESH_NO_MEMORY",
            0x4008 => "ESP_ERR_MESH_ARGUMENT",
            0x4009 => "ESP_ERR_MESH_EXCEED_MTU",
            0x400a => "ESP_ERR_MESH_TIMEOUT",
            0x400b => "ESP_ERR_MESH_DISCONNECTED",
            0x400c => "ESP_ERR_MESH_QUEUE_FAIL",
            0x400d => "ESP_ERR_MESH_QUEUE_FULL",
            0x400e => "ESP_ERR_MESH_NO_PARENT_FOUND",
            0x400f => "ESP_ERR_MESH_NO_ROUTE_FOUND",
            0x4010 => "ESP_ERR_MESH_OPTION_NULL",
            0x4011 => "ESP_ERR_MESH_OPTION_UNKNOWN",
            0x4012 => "ESP_ERR_MESH_XON_NO_WINDOW",
            0x4013 => "ESP_ERR_MESH_INTERFACE",
            0x4014 => "ESP_ERR_MESH_DISCARD_DUPLICATE",
            0x4015 => "ESP_ERR_MESH_DISCARD",
            0x4016 => "ESP_ERR_MESH_VOTING",
            0x4017 => "ESP_ERR_MESH_XMIT",
            0x4018 => "ESP_ERR_MESH_QUEUE_READ",
            0x4019 => "ESP_ERR_MESH_PS",
            0x401a => "ESP_ERR_MESH_RECV_RELEASE",
            0x5001 => "ESP_ERR_ESP_NETIF_INVALID_PARAMS",
            0x5002 => "ESP_ERR_ESP_NETIF_IF_NOT_READY",
            0x5003 => "ESP_ERR_ESP_NETIF_DHCPC_START_FAILED",
            0x5004 => "ESP_ERR_ESP_NETIF_DHCP_ALREADY_STARTED",
            0x5005 => "ESP_ERR_ESP_NETIF_DHCP_ALREADY_STOPPED",
            0x5006 => "ESP_ERR_ESP_NETIF_NO_MEM",
            0x5007 => "ESP_ERR_ESP_NETIF_DHCP_NOT_STOPPED",
            0x5008 => "ESP_ERR_ESP_NETIF_DRIVER_ATTACH_FAILED",
            0x5009 => "ESP_ERR_ESP_NETIF_INIT_FAILED",
            0x500a => "ESP_ERR_ESP_NETIF_DNS_NOT_CONFIGURED",
            0x500b => "ESP_ERR_ESP_NETIF_MLD6_FAILED",
            0x500c => "ESP_ERR_ESP_NETIF_IP6_ADDR_FAILED",
            0x500d => "ESP_ERR_ESP_NETIF_DHCPS_START_FAILED",
            0x500e => "ESP_ERR_ESP_NETIF_TX_FAILED",
            0x6001 => "ESP_ERR_FLASH_OP_FAIL",
            0x6002 => "ESP_ERR_FLASH_OP_TIMEOUT",
            0x6003 => "ESP_ERR_FLASH_NOT_INITIALISED",
            0x6004 => "ESP_ERR_FLASH_UNSUPPORTED_HOST",
            0x6005 => "ESP_ERR_FLASH_UNSUPPORTED_CHIP",
            0x6006 => "ESP_ERR_FLASH_PROTECTED",
            0x7001 => "ESP_ERR_HTTP_MAX_REDIRECT",
            0x7002 => "ESP_ERR_HTTP_CONNECT",
            0x7003 => "ESP_ERR_HTTP_WRITE_DATA",
            0x7004 => "ESP_ERR_HTTP_FETCH_HEADER",
            0x7005 => "ESP_ERR_HTTP_INVALID_TRANSPORT",
            0x7006 => "ESP_ERR_HTTP_CONNECTING",
            0x7007 => "ESP_ERR_HTTP_EAGAIN",
            0x7008 => "ESP_ERR_HTTP_CONNECTION_CLOSED",
            0x7009 => "ESP_ERR_HTTP_NOT_MODIFIED",
            0x700a => "ESP_ERR_HTTP_RANGE_NOT_SATISFIABLE",
            0x700b => "ESP_ERR_HTTP_READ_TIMEOUT",
            0x700c => "ESP_ERR_HTTP_INCOMPLETE_DATA",
            0x8001 => "ESP_ERR_ESP_TLS_CANNOT_RESOLVE_HOSTNAME",
            0x8002 => "ESP_ERR_ESP_TLS_CANNOT_CREATE_SOCKET",
            0x8003 => "ESP_ERR_ESP_TLS_UNSUPPORTED_PROTOCOL_FAMILY",
            0x8004 => "ESP_ERR_ESP_TLS_FAILED_CONNECT_TO_HOST",
            0x8005 => "ESP_ERR_ESP_TLS_SOCKET_SETOPT_FAILED",
            0x8006 => "ESP_ERR_ESP_TLS_CONNECTION_TIMEOUT",
            0x8007 => "ESP_ERR_ESP_TLS_SE_FAILED",
            0x8008 => "ESP_ERR_ESP_TLS_TCP_CLOSED_FIN",
            0x8009 => "ESP_ERR_ESP_TLS_SERVER_HANDSHAKE_TIMEOUT",
            0x8010 => "ESP_ERR_MBEDTLS_CERT_PARTLY_OK",
            0x8011 => "ESP_ERR_MBEDTLS_CTR_DRBG_SEED_FAILED",
            0x8012 => "ESP_ERR_MBEDTLS_SSL_SET_HOSTNAME_FAILED",
            0x8013 => "ESP_ERR_MBEDTLS_SSL_CONFIG_DEFAULTS_FAILED",
            0x8014 => "ESP_ERR_MBEDTLS_SSL_CONF_ALPN_PROTOCOLS_FAILED",
            0x8015 => "ESP_ERR_MBEDTLS_X509_CRT_PARSE_FAILED",
            0x8016 => "ESP_ERR_MBEDTLS_SSL_CONF_OWN_CERT_FAILED",
            0x8017 => "ESP_ERR_MBEDTLS_SSL_SETUP_FAILED",
            0x8018 => "ESP_ERR_MBEDTLS_SSL_WRITE_FAILED",
            0x8019 => "ESP_ERR_MBEDTLS_PK_PARSE_KEY_FAILED",
            0x801a => "ESP_ERR_MBEDTLS_SSL_HANDSHAKE_FAILED",
            0x801b => "ESP_ERR_MBEDTLS_SSL_CONF_PSK_FAILED",
            0x801c => "ESP_ERR_MBEDTLS_SSL_TICKET_SETUP_FAILED",
            0x801d => "ESP_ERR_MBEDTLS_SSL_READ_FAILED",
            0x9001 => "ESP_ERR_HTTPS_OTA_IN_PROGRESS",
            0xb001 => "ESP_ERR_HTTPD_HANDLERS_FULL",
            0xb002 => "ESP_ERR_HTTPD_HANDLER_EXISTS",
            0xb003 => "ESP_ERR_HTTPD_INVALID_REQ",
            0xb004 => "ESP_ERR_HTTPD_RESULT_TRUNC",
            0xb005 => "ESP_ERR_HTTPD_RESP_HDR",
            0xb006 => "ESP_ERR_HTTPD_RESP_SEND",
            0xb007 => "ESP_ERR_HTTPD_ALLOC_MEM",
            0xb008 => "ESP_ERR_HTTPD_TASK",
            0xc001 => "ESP_ERR_HW_CRYPTO_DS_HMAC_FAIL",
            0xc002 => "ESP_ERR_HW_CRYPTO_DS_INVALID_KEY",
            0xc004 => "ESP_ERR_HW_CRYPTO_DS_INVALID_DIGEST",
            0xc005 => "ESP_ERR_HW_CRYPTO_DS_INVALID_PADDING",
            0xd001 => "ESP_ERR_MEMPROT_MEMORY_TYPE_INVALID",
            0xd002 => "ESP_ERR_MEMPROT_SPLIT_ADDR_INVALID",
            0xd003 => "ESP_ERR_MEMPROT_SPLIT_ADDR_OUT_OF_RANGE",
            0xd004 => "ESP_ERR_MEMPROT_SPLIT_ADDR_UNALIGNED",
            0xd005 => "ESP_ERR_MEMPROT_UNIMGMT_BLOCK_INVALID",
            0xd006 => "ESP_ERR_MEMPROT_WORLD_INVALID",
            0xd007 => "ESP_ERR_MEMPROT_AREA_INVALID",
            0xd008 => "ESP_ERR_MEMPROT_CPUID_INVALID",
            0xe001 => "ESP_ERR_TCP_TRANSPORT_CONNECTION_TIMEOUT",
            0xe002 => "ESP_ERR_TCP_TRANSPORT_CONNECTION_CLOSED_BY_FIN",
            0xe003 => "ESP_ERR_TCP_TRANSPORT_CONNECTION_FAILED",
            0xe004 => "ESP_ERR_TCP_TRANSPORT_NO_MEM",
            0xf001 => "ESP_ERR_NVS_SEC_HMAC_KEY_NOT_FOUND",
            0xf002 => "ESP_ERR_NVS_SEC_HMAC_KEY_BLK_ALREADY_USED",
            0xf003 => "ESP_ERR_NVS_SEC_HMAC_KEY_GENERATION_FAILED",
            0xf004 => "ESP_ERR_NVS_SEC_HMAC_XTS_KEYS_DERIV_FAILED",
            /* ESP_IDF_ERROR_NAME_LOOKUP_TABLE */
            _ => unreachable!()
        }
    }

    pub const fn error_info(&self) -> &'static str {
        match self.0.get() {
            /* ESP_IDF_ERROR_INFO_LOOKUP_TABLE */
            -1 => "[ESP_FAIL]: Generic esp_err_t code indicating failure",
            0x101 => "[ESP_ERR_NO_MEM]: Out of memory",
            0x102 => "[ESP_ERR_INVALID_ARG]: Invalid argument",
            0x103 => "[ESP_ERR_INVALID_STATE]: Invalid state",
            0x104 => "[ESP_ERR_INVALID_SIZE]: Invalid size",
            0x105 => "[ESP_ERR_NOT_FOUND]: Requested resource not found",
            0x106 => "[ESP_ERR_NOT_SUPPORTED]: Operation or feature not supported",
            0x107 => "[ESP_ERR_TIMEOUT]: Operation timed out",
            0x108 => "[ESP_ERR_INVALID_RESPONSE]: Received response was invalid",
            0x109 => "[ESP_ERR_INVALID_CRC]: CRC or checksum was invalid",
            0x10a => "[ESP_ERR_INVALID_VERSION]: Version was invalid",
            0x10b => "[ESP_ERR_INVALID_MAC]: MAC address was invalid",
            0x10c => "[ESP_ERR_NOT_FINISHED]: Operation has not fully completed",
            0x10d => "[ESP_ERR_NOT_ALLOWED]: Operation is not allowed",
            0x1101 => "[ESP_ERR_NVS_NOT_INITIALIZED]: The storage driver is not initialized",
            0x1102 => "[ESP_ERR_NVS_NOT_FOUND]: A requested entry couldn't be found or namespace doesn’t exist yet and mode is NVS_READONLY",
            0x1103 => "[ESP_ERR_NVS_TYPE_MISMATCH]: The type of set or get operation doesn't match the type of value stored in NVS",
            0x1104 => "[ESP_ERR_NVS_READ_ONLY]: Storage handle was opened as read only",
            0x1105 => "[ESP_ERR_NVS_NOT_ENOUGH_SPACE]: There is not enough space in the underlying storage to save the value",
            0x1106 => "[ESP_ERR_NVS_INVALID_NAME]: Namespace name doesn’t satisfy constraints",
            0x1107 => "[ESP_ERR_NVS_INVALID_HANDLE]: Handle has been closed or is NULL",
            0x1108 => "[ESP_ERR_NVS_REMOVE_FAILED]: The value wasn’t updated because flash write operation has failed. The value was written however, and update will be finished after re-initialization of nvs, provided that flash operation doesn’t fail again.",
            0x1109 => "[ESP_ERR_NVS_KEY_TOO_LONG]: Key name is too long",
            0x110a => "[ESP_ERR_NVS_PAGE_FULL]: Internal error; never returned by nvs API functions",
            0x110b => "[ESP_ERR_NVS_INVALID_STATE]: NVS is in an inconsistent state due to a previous error. Call nvs_flash_init and nvs_open again, then retry.",
            0x110c => "[ESP_ERR_NVS_INVALID_LENGTH]: String or blob length is not sufficient to store data",
            0x110d => "[ESP_ERR_NVS_NO_FREE_PAGES]: NVS partition doesn't contain any empty pages. This may happen if NVS partition was truncated. Erase the whole partition and call nvs_flash_init again.",
            0x110e => "[ESP_ERR_NVS_VALUE_TOO_LONG]: Value doesn't fit into the entry or string or blob length is longer than supported by the implementation",
            0x110f => "[ESP_ERR_NVS_PART_NOT_FOUND]: Partition with specified name is not found in the partition table",
            0x1110 => "[ESP_ERR_NVS_NEW_VERSION_FOUND]: NVS partition contains data in new format and cannot be recognized by this version of code",
            0x1111 => "[ESP_ERR_NVS_XTS_ENCR_FAILED]: XTS encryption failed while writing NVS entry",
            0x1112 => "[ESP_ERR_NVS_XTS_DECR_FAILED]: XTS decryption failed while reading NVS entry",
            0x1113 => "[ESP_ERR_NVS_XTS_CFG_FAILED]: XTS configuration setting failed",
            0x1114 => "[ESP_ERR_NVS_XTS_CFG_NOT_FOUND]: XTS configuration not found",
            0x1115 => "[ESP_ERR_NVS_ENCR_NOT_SUPPORTED]: NVS encryption is not supported in this version",
            0x1116 => "[ESP_ERR_NVS_KEYS_NOT_INITIALIZED]: NVS key partition is uninitialized",
            0x1117 => "[ESP_ERR_NVS_CORRUPT_KEY_PART]: NVS key partition is corrupt",
            0x1118 => "[ESP_ERR_NVS_CONTENT_DIFFERS]: Internal error; never returned by nvs API functions. NVS key is different in comparison",
            0x1119 => "[ESP_ERR_NVS_WRONG_ENCRYPTION]: NVS partition is marked as encrypted with generic flash encryption. This is forbidden since the NVS encryption works differently.",
            0x1201 => "[ESP_ERR_ULP_SIZE_TOO_BIG]: Program doesn't fit into RTC memory reserved for the ULP",
            0x1202 => "[ESP_ERR_ULP_INVALID_LOAD_ADDR]: Load address is outside of RTC memory reserved for the ULP",
            0x1203 => "[ESP_ERR_ULP_DUPLICATE_LABEL]: More than one label with the same number was defined",
            0x1204 => "[ESP_ERR_ULP_UNDEFINED_LABEL]: Branch instructions references an undefined label",
            0x1205 => "[ESP_ERR_ULP_BRANCH_OUT_OF_RANGE]: Branch target is out of range of B instruction (try replacing with BX)",
            0x1501 => "[ESP_ERR_OTA_PARTITION_CONFLICT]: Error if request was to write or erase the current running partition",
            0x1502 => "[ESP_ERR_OTA_SELECT_INFO_INVALID]: Error if OTA data partition contains invalid content",
            0x1503 => "[ESP_ERR_OTA_VALIDATE_FAILED]: Error if OTA app image is invalid",
            0x1504 => "[ESP_ERR_OTA_SMALL_SEC_VER]: Error if the firmware has a secure version less than the running firmware.",
            0x1505 => "[ESP_ERR_OTA_ROLLBACK_FAILED]: Error if flash does not have valid firmware in passive partition and hence rollback is not possible",
            0x1506 => "[ESP_ERR_OTA_ROLLBACK_INVALID_STATE]: Error if current active firmware is still marked in pending validation state (ESP_OTA_IMG_PENDING_VERIFY), essentially first boot of firmware image post upgrade and hence firmware upgrade is not possible",
            0x1601 => "[ESP_OK_EFUSE_CNT]: OK the required number of bits is set.",
            0x1602 => "[ESP_ERR_EFUSE_CNT_IS_FULL]: Error field is full.",
            0x1603 => "[ESP_ERR_EFUSE_REPEATED_PROG]: Error repeated programming of programmed bits is strictly forbidden.",
            0x1604 => "[ESP_ERR_CODING]: Error while a encoding operation.",
            0x1605 => "[ESP_ERR_NOT_ENOUGH_UNUSED_KEY_BLOCKS]: Error not enough unused key blocks available",
            0x1606 => "[ESP_ERR_DAMAGED_READING]: Error. Burn or reset was done during a reading operation leads to damage read data. This error is internal to the efuse component and not returned by any public API.",
            0x2001 => "[ESP_ERR_IMAGE_FLASH_FAIL]: No further error description",
            0x2002 => "[ESP_ERR_IMAGE_INVALID]: No further error description",
            0x3001 => "[ESP_ERR_WIFI_NOT_INIT]: WiFi driver was not installed by esp_wifi_init",
            0x3002 => "[ESP_ERR_WIFI_NOT_STARTED]: WiFi driver was not started by esp_wifi_start",
            0x3003 => "[ESP_ERR_WIFI_NOT_STOPPED]: WiFi driver was not stopped by esp_wifi_stop",
            0x3004 => "[ESP_ERR_WIFI_IF]: WiFi interface error",
            0x3005 => "[ESP_ERR_WIFI_MODE]: WiFi mode error",
            0x3006 => "[ESP_ERR_WIFI_STATE]: WiFi internal state error",
            0x3007 => "[ESP_ERR_WIFI_CONN]: WiFi internal control block of station or soft-AP error",
            0x3008 => "[ESP_ERR_WIFI_NVS]: WiFi internal NVS module error",
            0x3009 => "[ESP_ERR_WIFI_MAC]: MAC address is invalid",
            0x300a => "[ESP_ERR_WIFI_SSID]: SSID is invalid",
            0x300b => "[ESP_ERR_WIFI_PASSWORD]: Password is invalid",
            0x300c => "[ESP_ERR_WIFI_TIMEOUT]: Timeout error",
            0x300d => "[ESP_ERR_WIFI_WAKE_FAIL]: WiFi is in sleep state(RF closed) and wakeup fail",
            0x300e => "[ESP_ERR_WIFI_WOULD_BLOCK]: The caller would block",
            0x300f => "[ESP_ERR_WIFI_NOT_CONNECT]: Station still in disconnect status",
            0x3012 => "[ESP_ERR_WIFI_POST]: Failed to post the event to WiFi task",
            0x3013 => "[ESP_ERR_WIFI_INIT_STATE]: Invalid WiFi state when init/deinit is called",
            0x3014 => "[ESP_ERR_WIFI_STOP_STATE]: Returned when WiFi is stopping",
            0x3015 => "[ESP_ERR_WIFI_NOT_ASSOC]: The WiFi connection is not associated",
            0x3016 => "[ESP_ERR_WIFI_TX_DISALLOW]: The WiFi TX is disallowed",
            0x3017 => "[ESP_ERR_WIFI_TWT_FULL]: no available flow id",
            0x3018 => "[ESP_ERR_WIFI_TWT_SETUP_TIMEOUT]: Timeout of receiving twt setup response frame, timeout times can be set during twt setup",
            0x3019 => "[ESP_ERR_WIFI_TWT_SETUP_TXFAIL]: TWT setup frame tx failed",
            0x301a => "[ESP_ERR_WIFI_TWT_SETUP_REJECT]: The twt setup request was rejected by the AP",
            0x301b => "[ESP_ERR_WIFI_DISCARD]: Discard frame",
            0x301c => "[ESP_ERR_WIFI_ROC_IN_PROGRESS]: ROC op is in progress",
            0x3033 => "[ESP_ERR_WIFI_REGISTRAR]: WPS registrar is not supported",
            0x3034 => "[ESP_ERR_WIFI_WPS_TYPE]: WPS type error",
            0x3035 => "[ESP_ERR_WIFI_WPS_SM]: WPS state machine is not initialized",
            0x3064 => "[ESP_ERR_ESPNOW_BASE]: ESPNOW error number base.",
            0x3065 => "[ESP_ERR_ESPNOW_NOT_INIT]: ESPNOW is not initialized.",
            0x3066 => "[ESP_ERR_ESPNOW_ARG]: Invalid argument",
            0x3067 => "[ESP_ERR_ESPNOW_NO_MEM]: Out of memory",
            0x3068 => "[ESP_ERR_ESPNOW_FULL]: ESPNOW peer list is full",
            0x3069 => "[ESP_ERR_ESPNOW_NOT_FOUND]: ESPNOW peer is not found",
            0x306a => "[ESP_ERR_ESPNOW_INTERNAL]: Internal error",
            0x306b => "[ESP_ERR_ESPNOW_EXIST]: ESPNOW peer has existed",
            0x306c => "[ESP_ERR_ESPNOW_IF]: Interface error",
            0x306d => "[ESP_ERR_ESPNOW_CHAN]: Channel error",
            0x3097 => "[ESP_ERR_DPP_FAILURE]: Generic failure during DPP Operation",
            0x3098 => "[ESP_ERR_DPP_TX_FAILURE]: DPP Frame Tx failed OR not Acked",
            0x3099 => "[ESP_ERR_DPP_INVALID_ATTR]: Encountered invalid DPP Attribute",
            0x309a => "[ESP_ERR_DPP_AUTH_TIMEOUT]: DPP Auth response was not received in time",
            0x309b => "[ESP_ERR_DPP_INVALID_LIST]: Channel list given in esp_supp_dpp_bootstrap_gen() is not valid or too big",
            0x309c => "[ESP_ERR_DPP_CONF_TIMEOUT]: DPP Configuration was not received in time",
            0x4001 => "[ESP_ERR_MESH_WIFI_NOT_START]: No further error description",
            0x4002 => "[ESP_ERR_MESH_NOT_INIT]: No further error description",
            0x4003 => "[ESP_ERR_MESH_NOT_CONFIG]: No further error description",
            0x4004 => "[ESP_ERR_MESH_NOT_START]: No further error description",
            0x4005 => "[ESP_ERR_MESH_NOT_SUPPORT]: No further error description",
            0x4006 => "[ESP_ERR_MESH_NOT_ALLOWED]: No further error description",
            0x4007 => "[ESP_ERR_MESH_NO_MEMORY]: No further error description",
            0x4008 => "[ESP_ERR_MESH_ARGUMENT]: No further error description",
            0x4009 => "[ESP_ERR_MESH_EXCEED_MTU]: No further error description",
            0x400a => "[ESP_ERR_MESH_TIMEOUT]: No further error description",
            0x400b => "[ESP_ERR_MESH_DISCONNECTED]: No further error description",
            0x400c => "[ESP_ERR_MESH_QUEUE_FAIL]: No further error description",
            0x400d => "[ESP_ERR_MESH_QUEUE_FULL]: No further error description",
            0x400e => "[ESP_ERR_MESH_NO_PARENT_FOUND]: No further error description",
            0x400f => "[ESP_ERR_MESH_NO_ROUTE_FOUND]: No further error description",
            0x4010 => "[ESP_ERR_MESH_OPTION_NULL]: No further error description",
            0x4011 => "[ESP_ERR_MESH_OPTION_UNKNOWN]: No further error description",
            0x4012 => "[ESP_ERR_MESH_XON_NO_WINDOW]: No further error description",
            0x4013 => "[ESP_ERR_MESH_INTERFACE]: No further error description",
            0x4014 => "[ESP_ERR_MESH_DISCARD_DUPLICATE]: No further error description",
            0x4015 => "[ESP_ERR_MESH_DISCARD]: No further error description",
            0x4016 => "[ESP_ERR_MESH_VOTING]: No further error description",
            0x4017 => "[ESP_ERR_MESH_XMIT]: No further error description",
            0x4018 => "[ESP_ERR_MESH_QUEUE_READ]: No further error description",
            0x4019 => "[ESP_ERR_MESH_PS]: No further error description",
            0x401a => "[ESP_ERR_MESH_RECV_RELEASE]: No further error description",
            0x5001 => "[ESP_ERR_ESP_NETIF_INVALID_PARAMS]: No further error description",
            0x5002 => "[ESP_ERR_ESP_NETIF_IF_NOT_READY]: No further error description",
            0x5003 => "[ESP_ERR_ESP_NETIF_DHCPC_START_FAILED]: No further error description",
            0x5004 => "[ESP_ERR_ESP_NETIF_DHCP_ALREADY_STARTED]: No further error description",
            0x5005 => "[ESP_ERR_ESP_NETIF_DHCP_ALREADY_STOPPED]: No further error description",
            0x5006 => "[ESP_ERR_ESP_NETIF_NO_MEM]: No further error description",
            0x5007 => "[ESP_ERR_ESP_NETIF_DHCP_NOT_STOPPED]: No further error description",
            0x5008 => "[ESP_ERR_ESP_NETIF_DRIVER_ATTACH_FAILED]: No further error description",
            0x5009 => "[ESP_ERR_ESP_NETIF_INIT_FAILED]: No further error description",
            0x500a => "[ESP_ERR_ESP_NETIF_DNS_NOT_CONFIGURED]: No further error description",
            0x500b => "[ESP_ERR_ESP_NETIF_MLD6_FAILED]: No further error description",
            0x500c => "[ESP_ERR_ESP_NETIF_IP6_ADDR_FAILED]: No further error description",
            0x500d => "[ESP_ERR_ESP_NETIF_DHCPS_START_FAILED]: No further error description",
            0x500e => "[ESP_ERR_ESP_NETIF_TX_FAILED]: No further error description",
            0x6001 => "[ESP_ERR_FLASH_OP_FAIL]: No further error description",
            0x6002 => "[ESP_ERR_FLASH_OP_TIMEOUT]: No further error description",
            0x6003 => "[ESP_ERR_FLASH_NOT_INITIALISED]: No further error description",
            0x6004 => "[ESP_ERR_FLASH_UNSUPPORTED_HOST]: No further error description",
            0x6005 => "[ESP_ERR_FLASH_UNSUPPORTED_CHIP]: No further error description",
            0x6006 => "[ESP_ERR_FLASH_PROTECTED]: No further error description",
            0x7001 => "[ESP_ERR_HTTP_MAX_REDIRECT]: The error exceeds the number of HTTP redirects",
            0x7002 => "[ESP_ERR_HTTP_CONNECT]: Error open the HTTP connection",
            0x7003 => "[ESP_ERR_HTTP_WRITE_DATA]: Error write HTTP data",
            0x7004 => "[ESP_ERR_HTTP_FETCH_HEADER]: Error read HTTP header from server",
            0x7005 => "[ESP_ERR_HTTP_INVALID_TRANSPORT]: There are no transport support for the input scheme",
            0x7006 => "[ESP_ERR_HTTP_CONNECTING]: HTTP connection hasn't been established yet",
            0x7007 => "[ESP_ERR_HTTP_EAGAIN]: Mapping of errno EAGAIN to esp_err_t",
            0x7008 => "[ESP_ERR_HTTP_CONNECTION_CLOSED]: Read FIN from peer and the connection closed",
            0x7009 => "[ESP_ERR_HTTP_NOT_MODIFIED]: HTTP 304 Not Modified, no update available",
            0x700a => "[ESP_ERR_HTTP_RANGE_NOT_SATISFIABLE]: HTTP 416 Range Not Satisfiable, requested range in header is incorrect",
            0x700b => "[ESP_ERR_HTTP_READ_TIMEOUT]: HTTP data read timeout",
            0x700c => "[ESP_ERR_HTTP_INCOMPLETE_DATA]: Incomplete data received, less than Content-Length or last chunk",
            0x8001 => "[ESP_ERR_ESP_TLS_CANNOT_RESOLVE_HOSTNAME]: Error if hostname couldn't be resolved upon tls connection",
            0x8002 => "[ESP_ERR_ESP_TLS_CANNOT_CREATE_SOCKET]: Failed to create socket",
            0x8003 => "[ESP_ERR_ESP_TLS_UNSUPPORTED_PROTOCOL_FAMILY]: Unsupported protocol family",
            0x8004 => "[ESP_ERR_ESP_TLS_FAILED_CONNECT_TO_HOST]: Failed to connect to host",
            0x8005 => "[ESP_ERR_ESP_TLS_SOCKET_SETOPT_FAILED]: failed to set/get socket option",
            0x8006 => "[ESP_ERR_ESP_TLS_CONNECTION_TIMEOUT]: new connection in esp_tls_low_level_conn connection timeouted",
            0x8007 => "[ESP_ERR_ESP_TLS_SE_FAILED]: No further error description",
            0x8008 => "[ESP_ERR_ESP_TLS_TCP_CLOSED_FIN]: No further error description",
            0x8009 => "[ESP_ERR_ESP_TLS_SERVER_HANDSHAKE_TIMEOUT]: TLS handshake timeout",
            0x8010 => "[ESP_ERR_MBEDTLS_CERT_PARTLY_OK]: mbedtls parse certificates was partly successful",
            0x8011 => "[ESP_ERR_MBEDTLS_CTR_DRBG_SEED_FAILED]: mbedtls api returned error",
            0x8012 => "[ESP_ERR_MBEDTLS_SSL_SET_HOSTNAME_FAILED]: mbedtls api returned error",
            0x8013 => "[ESP_ERR_MBEDTLS_SSL_CONFIG_DEFAULTS_FAILED]: mbedtls api returned error",
            0x8014 => "[ESP_ERR_MBEDTLS_SSL_CONF_ALPN_PROTOCOLS_FAILED]: mbedtls api returned error",
            0x8015 => "[ESP_ERR_MBEDTLS_X509_CRT_PARSE_FAILED]: mbedtls api returned error",
            0x8016 => "[ESP_ERR_MBEDTLS_SSL_CONF_OWN_CERT_FAILED]: mbedtls api returned error",
            0x8017 => "[ESP_ERR_MBEDTLS_SSL_SETUP_FAILED]: mbedtls api returned error",
            0x8018 => "[ESP_ERR_MBEDTLS_SSL_WRITE_FAILED]: mbedtls api returned error",
            0x8019 => "[ESP_ERR_MBEDTLS_PK_PARSE_KEY_FAILED]: mbedtls api returned failed",
            0x801a => "[ESP_ERR_MBEDTLS_SSL_HANDSHAKE_FAILED]: mbedtls api returned failed",
            0x801b => "[ESP_ERR_MBEDTLS_SSL_CONF_PSK_FAILED]: mbedtls api returned failed",
            0x801c => "[ESP_ERR_MBEDTLS_SSL_TICKET_SETUP_FAILED]: mbedtls api returned failed",
            0x801d => "[ESP_ERR_MBEDTLS_SSL_READ_FAILED]: mbedtls api returned failed",
            0x9001 => "[ESP_ERR_HTTPS_OTA_IN_PROGRESS]: No further error description",
            0xb001 => "[ESP_ERR_HTTPD_HANDLERS_FULL]: All slots for registering URI handlers have been consumed",
            0xb002 => "[ESP_ERR_HTTPD_HANDLER_EXISTS]: URI handler with same method and target URI already registered",
            0xb003 => "[ESP_ERR_HTTPD_INVALID_REQ]: Invalid request pointer",
            0xb004 => "[ESP_ERR_HTTPD_RESULT_TRUNC]: Result string truncated",
            0xb005 => "[ESP_ERR_HTTPD_RESP_HDR]: Response header field larger than supported",
            0xb006 => "[ESP_ERR_HTTPD_RESP_SEND]: Error occurred while sending response packet",
            0xb007 => "[ESP_ERR_HTTPD_ALLOC_MEM]: Failed to dynamically allocate memory for resource",
            0xb008 => "[ESP_ERR_HTTPD_TASK]: Failed to launch server task/thread",
            0xc001 => "[ESP_ERR_HW_CRYPTO_DS_HMAC_FAIL]: HMAC peripheral problem",
            0xc002 => "[ESP_ERR_HW_CRYPTO_DS_INVALID_KEY]: No further error description",
            0xc004 => "[ESP_ERR_HW_CRYPTO_DS_INVALID_DIGEST]: No further error description",
            0xc005 => "[ESP_ERR_HW_CRYPTO_DS_INVALID_PADDING]: No further error description",
            0xd001 => "[ESP_ERR_MEMPROT_MEMORY_TYPE_INVALID]: No further error description",
            0xd002 => "[ESP_ERR_MEMPROT_SPLIT_ADDR_INVALID]: No further error description",
            0xd003 => "[ESP_ERR_MEMPROT_SPLIT_ADDR_OUT_OF_RANGE]: No further error description",
            0xd004 => "[ESP_ERR_MEMPROT_SPLIT_ADDR_UNALIGNED]: No further error description",
            0xd005 => "[ESP_ERR_MEMPROT_UNIMGMT_BLOCK_INVALID]: No further error description",
            0xd006 => "[ESP_ERR_MEMPROT_WORLD_INVALID]: No further error description",
            0xd007 => "[ESP_ERR_MEMPROT_AREA_INVALID]: No further error description",
            0xd008 => "[ESP_ERR_MEMPROT_CPUID_INVALID]: No further error description",
            0xe001 => "[ESP_ERR_TCP_TRANSPORT_CONNECTION_TIMEOUT]: Connection has timed out",
            0xe002 => "[ESP_ERR_TCP_TRANSPORT_CONNECTION_CLOSED_BY_FIN]: Read FIN from peer and the connection has closed (in a clean way)",
            0xe003 => "[ESP_ERR_TCP_TRANSPORT_CONNECTION_FAILED]: Failed to connect to the peer",
            0xe004 => "[ESP_ERR_TCP_TRANSPORT_NO_MEM]: Memory allocation failed",
            0xf001 => "[ESP_ERR_NVS_SEC_HMAC_KEY_NOT_FOUND]: HMAC Key required to generate the NVS encryption keys not found",
            0xf002 => "[ESP_ERR_NVS_SEC_HMAC_KEY_BLK_ALREADY_USED]: Provided eFuse block for HMAC key generation is already in use",
            0xf003 => "[ESP_ERR_NVS_SEC_HMAC_KEY_GENERATION_FAILED]: Failed to generate/write the HMAC key to eFuse",
            0xf004 => "[ESP_ERR_NVS_SEC_HMAC_XTS_KEYS_DERIV_FAILED]: Failed to derive the NVS encryption keys based on the HMAC-based scheme",
            /* ESP_IDF_ERROR_INFO_LOOKUP_TABLE */
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_success_code() {
        let _ = IdfError::try_from(0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_code() {
        let _ = IdfError::try_from(12).unwrap();
    }

    #[test]
    fn test_valid_error_code_equ() {
        let err = IdfError::try_from(0xe002).unwrap();
        assert_eq!(err, IdfError(NonZeroI32::new(0xe002).unwrap()));
    }

    #[test]
    fn test_valid_error_code_name() {
       let err = IdfError::try_from(0xe002).unwrap();
        assert_eq!(err.error_name(), "ESP_ERR_TCP_TRANSPORT_CONNECTION_CLOSED_BY_FIN");
    }

    #[test]
    fn test_valid_error_code_info() {
       let err = IdfError::try_from(0xe002).unwrap();
        assert_eq!(err.error_info(), "[ESP_ERR_TCP_TRANSPORT_CONNECTION_CLOSED_BY_FIN]: Read FIN from peer and the connection has closed (in a clean way)");
    }

    #[test]
    fn test_valid_error_code_format() {
       let err = IdfError::try_from(0xe002).unwrap();
       assert_eq!(format!("{err}"), "[ESP_ERR_TCP_TRANSPORT_CONNECTION_CLOSED_BY_FIN]: Read FIN from peer and the connection has closed (in a clean way)");
    }
}