#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    bytecntr: BYTECNTR,
    tcblkcntr: TCBLKCNTR,
    crccr: CRCCR,
    cmd_crc: CMD_CRC,
    dat_crcl: DAT_CRCL,
    dat_crch: DAT_CRCH,
    port: PORT,
    imr: IMR,
    icr: ICR,
    cardsel: CARDSEL,
    sigr: SIGR,
    mbcr: MBCR,
    blkcntr: BLKCNTR,
    tocntr: TOCNTR,
    cmd_buf0: CMD_BUF0,
    cmd_buf1: CMD_BUF1,
    cmd_buf2: CMD_BUF2,
    cmd_buf3: CMD_BUF3,
    cmd_buf4: CMD_BUF4,
    cmd_buf5: CMD_BUF5,
    cmd_buf6: CMD_BUF6,
    cmd_buf7: CMD_BUF7,
    cmd_buf8: CMD_BUF8,
    cmd_buf9: CMD_BUF9,
    cmd_buf10: CMD_BUF10,
    cmd_buf11: CMD_BUF11,
    cmd_buf12: CMD_BUF12,
    cmd_buf13: CMD_BUF13,
    cmd_buf14: CMD_BUF14,
    cmd_buf15: CMD_BUF15,
    bufcr: BUFCR,
    _reserved33: [u8; 0x7c],
    dat_buf: DAT_BUF,
}
impl RegisterBlock {
    #[doc = "0x00 - CR1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - CR2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - Data transfer byte count register"]
    #[inline(always)]
    pub const fn bytecntr(&self) -> &BYTECNTR {
        &self.bytecntr
    }
    #[doc = "0x0c - Data block count register"]
    #[inline(always)]
    pub const fn tcblkcntr(&self) -> &TCBLKCNTR {
        &self.tcblkcntr
    }
    #[doc = "0x10 - CRC control register"]
    #[inline(always)]
    pub const fn crccr(&self) -> &CRCCR {
        &self.crccr
    }
    #[doc = "0x14 - CMD_CRC register"]
    #[inline(always)]
    pub const fn cmd_crc(&self) -> &CMD_CRC {
        &self.cmd_crc
    }
    #[doc = "0x18 - CRC low data register"]
    #[inline(always)]
    pub const fn dat_crcl(&self) -> &DAT_CRCL {
        &self.dat_crcl
    }
    #[doc = "0x1c - CRC high data register"]
    #[inline(always)]
    pub const fn dat_crch(&self) -> &DAT_CRCH {
        &self.dat_crch
    }
    #[doc = "0x20 - MMC port register"]
    #[inline(always)]
    pub const fn port(&self) -> &PORT {
        &self.port
    }
    #[doc = "0x24 - Interrupt mask register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x28 - Interrupt clear mask register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x2c - card sell"]
    #[inline(always)]
    pub const fn cardsel(&self) -> &CARDSEL {
        &self.cardsel
    }
    #[doc = "0x30 - Signal register"]
    #[inline(always)]
    pub const fn sigr(&self) -> &SIGR {
        &self.sigr
    }
    #[doc = "0x34 - Multi-block data transmission register"]
    #[inline(always)]
    pub const fn mbcr(&self) -> &MBCR {
        &self.mbcr
    }
    #[doc = "0x38 - Data block count register"]
    #[inline(always)]
    pub const fn blkcntr(&self) -> &BLKCNTR {
        &self.blkcntr
    }
    #[doc = "0x3c - Data transfer timeout count register"]
    #[inline(always)]
    pub const fn tocntr(&self) -> &TOCNTR {
        &self.tocntr
    }
    #[doc = "0x40 - CMD buffer register 0"]
    #[inline(always)]
    pub const fn cmd_buf0(&self) -> &CMD_BUF0 {
        &self.cmd_buf0
    }
    #[doc = "0x44 - CMD buffer register 1"]
    #[inline(always)]
    pub const fn cmd_buf1(&self) -> &CMD_BUF1 {
        &self.cmd_buf1
    }
    #[doc = "0x48 - CMD buffer register 2"]
    #[inline(always)]
    pub const fn cmd_buf2(&self) -> &CMD_BUF2 {
        &self.cmd_buf2
    }
    #[doc = "0x4c - CMD buffer register 3"]
    #[inline(always)]
    pub const fn cmd_buf3(&self) -> &CMD_BUF3 {
        &self.cmd_buf3
    }
    #[doc = "0x50 - CMD buffer register 4"]
    #[inline(always)]
    pub const fn cmd_buf4(&self) -> &CMD_BUF4 {
        &self.cmd_buf4
    }
    #[doc = "0x54 - CMD buffer register 5"]
    #[inline(always)]
    pub const fn cmd_buf5(&self) -> &CMD_BUF5 {
        &self.cmd_buf5
    }
    #[doc = "0x58 - CMD buffer register 6"]
    #[inline(always)]
    pub const fn cmd_buf6(&self) -> &CMD_BUF6 {
        &self.cmd_buf6
    }
    #[doc = "0x5c - CMD buffer register 7"]
    #[inline(always)]
    pub const fn cmd_buf7(&self) -> &CMD_BUF7 {
        &self.cmd_buf7
    }
    #[doc = "0x60 - CMD buffer register 8"]
    #[inline(always)]
    pub const fn cmd_buf8(&self) -> &CMD_BUF8 {
        &self.cmd_buf8
    }
    #[doc = "0x64 - CMD buffer register 9"]
    #[inline(always)]
    pub const fn cmd_buf9(&self) -> &CMD_BUF9 {
        &self.cmd_buf9
    }
    #[doc = "0x68 - CMD buffer register 10"]
    #[inline(always)]
    pub const fn cmd_buf10(&self) -> &CMD_BUF10 {
        &self.cmd_buf10
    }
    #[doc = "0x6c - CMD buffer register 11"]
    #[inline(always)]
    pub const fn cmd_buf11(&self) -> &CMD_BUF11 {
        &self.cmd_buf11
    }
    #[doc = "0x70 - CMD buffer register 12"]
    #[inline(always)]
    pub const fn cmd_buf12(&self) -> &CMD_BUF12 {
        &self.cmd_buf12
    }
    #[doc = "0x74 - CMD buffer register 13"]
    #[inline(always)]
    pub const fn cmd_buf13(&self) -> &CMD_BUF13 {
        &self.cmd_buf13
    }
    #[doc = "0x78 - CMD buffer register 14"]
    #[inline(always)]
    pub const fn cmd_buf14(&self) -> &CMD_BUF14 {
        &self.cmd_buf14
    }
    #[doc = "0x7c - CMD buffer register 15"]
    #[inline(always)]
    pub const fn cmd_buf15(&self) -> &CMD_BUF15 {
        &self.cmd_buf15
    }
    #[doc = "0x80 - Buffer control register"]
    #[inline(always)]
    pub const fn bufcr(&self) -> &BUFCR {
        &self.bufcr
    }
    #[doc = "0x100 - Data buffer register"]
    #[inline(always)]
    pub const fn dat_buf(&self) -> &DAT_BUF {
        &self.dat_buf
    }
}
#[doc = "CR1 (rw) register accessor: CR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "CR1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: CR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "CR2"]
pub mod cr2;
#[doc = "BYTECNTR (rw) register accessor: Data transfer byte count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytecntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bytecntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bytecntr`]
module"]
pub type BYTECNTR = crate::Reg<bytecntr::BYTECNTR_SPEC>;
#[doc = "Data transfer byte count register"]
pub mod bytecntr;
#[doc = "TCBLKCNTR (rw) register accessor: Data block count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcblkcntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcblkcntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcblkcntr`]
module"]
pub type TCBLKCNTR = crate::Reg<tcblkcntr::TCBLKCNTR_SPEC>;
#[doc = "Data block count register"]
pub mod tcblkcntr;
#[doc = "CRCCR (rw) register accessor: CRC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr`]
module"]
pub type CRCCR = crate::Reg<crccr::CRCCR_SPEC>;
#[doc = "CRC control register"]
pub mod crccr;
#[doc = "CMD_CRC (r) register accessor: CMD_CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_crc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_crc`]
module"]
pub type CMD_CRC = crate::Reg<cmd_crc::CMD_CRC_SPEC>;
#[doc = "CMD_CRC register"]
pub mod cmd_crc;
#[doc = "DAT_CRCL (r) register accessor: CRC low data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat_crcl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat_crcl`]
module"]
pub type DAT_CRCL = crate::Reg<dat_crcl::DAT_CRCL_SPEC>;
#[doc = "CRC low data register"]
pub mod dat_crcl;
#[doc = "DAT_CRCH (r) register accessor: CRC high data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat_crch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat_crch`]
module"]
pub type DAT_CRCH = crate::Reg<dat_crch::DAT_CRCH_SPEC>;
#[doc = "CRC high data register"]
pub mod dat_crch;
#[doc = "PORT (r) register accessor: MMC port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`port::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port`]
module"]
pub type PORT = crate::Reg<port::PORT_SPEC>;
#[doc = "MMC port register"]
pub mod port;
#[doc = "IMR (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "ICR (rw) register accessor: Interrupt clear mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt clear mask register"]
pub mod icr;
#[doc = "CARDSEL (rw) register accessor: card sell\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cardsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cardsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cardsel`]
module"]
pub type CARDSEL = crate::Reg<cardsel::CARDSEL_SPEC>;
#[doc = "card sell"]
pub mod cardsel;
#[doc = "SIGR (r) register accessor: Signal register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigr`]
module"]
pub type SIGR = crate::Reg<sigr::SIGR_SPEC>;
#[doc = "Signal register"]
pub mod sigr;
#[doc = "MBCR (rw) register accessor: Multi-block data transmission register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbcr`]
module"]
pub type MBCR = crate::Reg<mbcr::MBCR_SPEC>;
#[doc = "Multi-block data transmission register"]
pub mod mbcr;
#[doc = "BLKCNTR (rw) register accessor: Data block count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkcntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkcntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blkcntr`]
module"]
pub type BLKCNTR = crate::Reg<blkcntr::BLKCNTR_SPEC>;
#[doc = "Data block count register"]
pub mod blkcntr;
#[doc = "TOCNTR (rw) register accessor: Data transfer timeout count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocntr`]
module"]
pub type TOCNTR = crate::Reg<tocntr::TOCNTR_SPEC>;
#[doc = "Data transfer timeout count register"]
pub mod tocntr;
#[doc = "CMD_BUF0 (rw) register accessor: CMD buffer register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf0`]
module"]
pub type CMD_BUF0 = crate::Reg<cmd_buf0::CMD_BUF0_SPEC>;
#[doc = "CMD buffer register 0"]
pub mod cmd_buf0;
#[doc = "CMD_BUF1 (rw) register accessor: CMD buffer register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf1`]
module"]
pub type CMD_BUF1 = crate::Reg<cmd_buf1::CMD_BUF1_SPEC>;
#[doc = "CMD buffer register 1"]
pub mod cmd_buf1;
#[doc = "CMD_BUF2 (rw) register accessor: CMD buffer register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf2`]
module"]
pub type CMD_BUF2 = crate::Reg<cmd_buf2::CMD_BUF2_SPEC>;
#[doc = "CMD buffer register 2"]
pub mod cmd_buf2;
#[doc = "CMD_BUF3 (rw) register accessor: CMD buffer register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf3`]
module"]
pub type CMD_BUF3 = crate::Reg<cmd_buf3::CMD_BUF3_SPEC>;
#[doc = "CMD buffer register 3"]
pub mod cmd_buf3;
#[doc = "CMD_BUF4 (rw) register accessor: CMD buffer register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf4`]
module"]
pub type CMD_BUF4 = crate::Reg<cmd_buf4::CMD_BUF4_SPEC>;
#[doc = "CMD buffer register 4"]
pub mod cmd_buf4;
#[doc = "CMD_BUF5 (rw) register accessor: CMD buffer register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf5`]
module"]
pub type CMD_BUF5 = crate::Reg<cmd_buf5::CMD_BUF5_SPEC>;
#[doc = "CMD buffer register 5"]
pub mod cmd_buf5;
#[doc = "CMD_BUF6 (rw) register accessor: CMD buffer register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf6`]
module"]
pub type CMD_BUF6 = crate::Reg<cmd_buf6::CMD_BUF6_SPEC>;
#[doc = "CMD buffer register 6"]
pub mod cmd_buf6;
#[doc = "CMD_BUF7 (rw) register accessor: CMD buffer register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf7`]
module"]
pub type CMD_BUF7 = crate::Reg<cmd_buf7::CMD_BUF7_SPEC>;
#[doc = "CMD buffer register 7"]
pub mod cmd_buf7;
#[doc = "CMD_BUF8 (rw) register accessor: CMD buffer register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf8`]
module"]
pub type CMD_BUF8 = crate::Reg<cmd_buf8::CMD_BUF8_SPEC>;
#[doc = "CMD buffer register 8"]
pub mod cmd_buf8;
#[doc = "CMD_BUF9 (rw) register accessor: CMD buffer register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf9`]
module"]
pub type CMD_BUF9 = crate::Reg<cmd_buf9::CMD_BUF9_SPEC>;
#[doc = "CMD buffer register 9"]
pub mod cmd_buf9;
#[doc = "CMD_BUF10 (rw) register accessor: CMD buffer register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf10`]
module"]
pub type CMD_BUF10 = crate::Reg<cmd_buf10::CMD_BUF10_SPEC>;
#[doc = "CMD buffer register 10"]
pub mod cmd_buf10;
#[doc = "CMD_BUF11 (rw) register accessor: CMD buffer register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf11`]
module"]
pub type CMD_BUF11 = crate::Reg<cmd_buf11::CMD_BUF11_SPEC>;
#[doc = "CMD buffer register 11"]
pub mod cmd_buf11;
#[doc = "CMD_BUF12 (rw) register accessor: CMD buffer register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf12`]
module"]
pub type CMD_BUF12 = crate::Reg<cmd_buf12::CMD_BUF12_SPEC>;
#[doc = "CMD buffer register 12"]
pub mod cmd_buf12;
#[doc = "CMD_BUF13 (rw) register accessor: CMD buffer register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf13`]
module"]
pub type CMD_BUF13 = crate::Reg<cmd_buf13::CMD_BUF13_SPEC>;
#[doc = "CMD buffer register 13"]
pub mod cmd_buf13;
#[doc = "CMD_BUF14 (rw) register accessor: CMD buffer register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf14`]
module"]
pub type CMD_BUF14 = crate::Reg<cmd_buf14::CMD_BUF14_SPEC>;
#[doc = "CMD buffer register 14"]
pub mod cmd_buf14;
#[doc = "CMD_BUF15 (rw) register accessor: CMD buffer register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_buf15`]
module"]
pub type CMD_BUF15 = crate::Reg<cmd_buf15::CMD_BUF15_SPEC>;
#[doc = "CMD buffer register 15"]
pub mod cmd_buf15;
#[doc = "BUFCR (rw) register accessor: Buffer control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufcr`]
module"]
pub type BUFCR = crate::Reg<bufcr::BUFCR_SPEC>;
#[doc = "Buffer control register"]
pub mod bufcr;
#[doc = "DAT_BUF (rw) register accessor: Data buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat_buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat_buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat_buf`]
module"]
pub type DAT_BUF = crate::Reg<dat_buf::DAT_BUF_SPEC>;
#[doc = "Data buffer register"]
pub mod dat_buf;
