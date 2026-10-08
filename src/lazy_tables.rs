use std::hint::black_box;
use std::sync::LazyLock;

use crate::arch::software::{generate_table_u16, generate_table_u32, generate_table_u64};

macro_rules! define_tables {
    ($module:ident, $word:ty, $generate:ident, $width:literal;
     $($name:ident => ($poly:expr, $reflected:expr)),+;
     $($alias:ident => $target:ident),* $(,)?) => {
        $(
            pub static $name: LazyLock<Box<[[$word; 256]; 16]>> = LazyLock::new(|| {
                // Keep optimization from replacing runtime generation with embedded table bytes.
                Box::new($generate($width, black_box($poly), $reflected))
            });
        )+
        $(pub use self::$target as $alias;)*

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn generated_tables_match_prebuilt() {
                $(assert_eq!(
                    LazyLock::force(&$name).as_ref(),
                    &crate::prebuilt_tables::$module::$name,
                    stringify!($name),
                );)+
                $(assert_eq!(
                    LazyLock::force(&$alias).as_ref(),
                    &crate::prebuilt_tables::$module::$alias,
                    stringify!($alias),
                );)*
            }
        }
    };
}

pub mod crc16 {
    use super::*;

    define_tables!(crc16, u16, generate_table_u16, 16;
        CRC16_ARC_TABLE => (0x8005, true),
        CRC16_CDMA2000_TABLE => (0xc867, false),
        CRC16_CMS_TABLE => (0x8005, false),
        CRC16_DECT_R_TABLE => (0x0589, false),
        CRC16_DNP_TABLE => (0x3d65, true),
        CRC16_EN_13757_TABLE => (0x3d65, false),
        CRC16_GENIBUS_TABLE => (0x1021, false),
        CRC16_IBM_SDLC_TABLE => (0x1021, true),
        CRC16_LJ1200_TABLE => (0x6f63, false),
        CRC16_M17_TABLE => (0x5935, false),
        CRC16_NRSC_5_TABLE => (0x080b, true),
        CRC16_OPENSAFETY_B_TABLE => (0x755b, false),
        CRC16_PROFIBUS_TABLE => (0x1dcf, false),
        CRC16_T10_DIF_TABLE => (0x8bb7, false),
        CRC16_TELEDISK_TABLE => (0xa097, false);
        CRC16_DDS_110_TABLE => CRC16_CMS_TABLE,
        CRC16_DECT_X_TABLE => CRC16_DECT_R_TABLE,
        CRC16_GSM_TABLE => CRC16_GENIBUS_TABLE,
        CRC16_IBM_3740_TABLE => CRC16_GENIBUS_TABLE,
        CRC16_ISO_IEC_14443_3_A_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_KERMIT_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_MAXIM_DOW_TABLE => CRC16_ARC_TABLE,
        CRC16_MCRF4XX_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_MODBUS_TABLE => CRC16_ARC_TABLE,
        CRC16_OPENSAFETY_A_TABLE => CRC16_M17_TABLE,
        CRC16_RIELLO_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_SPI_FUJITSU_TABLE => CRC16_GENIBUS_TABLE,
        CRC16_TMS37157_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_UMTS_TABLE => CRC16_CMS_TABLE,
        CRC16_USB_TABLE => CRC16_ARC_TABLE,
        CRC16_XMODEM_TABLE => CRC16_GENIBUS_TABLE,
    );
}

pub mod crc32 {
    use super::*;

    define_tables!(crc32, u32, generate_table_u32, 32;
        CRC32_AIXM_TABLE => (0x814141ab, false),
        CRC32_AUTOSAR_TABLE => (0xf4acfb13, true),
        CRC32_BASE91_D_TABLE => (0xa833982b, true),
        CRC32_BZIP2_TABLE => (0x04c11db7, false),
        CRC32_CD_ROM_EDC_TABLE => (0x8001801b, true),
        CRC32_ISCSI_TABLE => (0x1edc6f41, true),
        CRC32_ISO_HDLC_TABLE => (0x04c11db7, true),
        CRC32_MEF_TABLE => (0x741b8cd7, true),
        CRC32_XFER_TABLE => (0x000000af, false);
        CRC32_CKSUM_TABLE => CRC32_BZIP2_TABLE,
        CRC32_JAMCRC_TABLE => CRC32_ISO_HDLC_TABLE,
        CRC32_MPEG_2_TABLE => CRC32_BZIP2_TABLE,
    );
}

pub mod crc64 {
    use super::*;

    define_tables!(crc64, u64, generate_table_u64, 64;
        CRC64_ECMA_182_TABLE => (0x42f0e1eba9ea3693, false),
        CRC64_GO_ISO_TABLE => (0x000000000000001b, true),
        CRC64_MS_TABLE => (0x259c84cba6426349, true),
        CRC64_NVME_TABLE => (0xad93d23594c93659, true),
        CRC64_REDIS_TABLE => (0xad93d23594c935a9, true),
        CRC64_XZ_TABLE => (0x42f0e1eba9ea3693, true);
        CRC64_WE_TABLE => CRC64_ECMA_182_TABLE,
    );
}
