#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dr: DR,
    cfgr: CFGR,
    cr: CR,
    chsr: CHSR,
    cmpr: CMPR,
    sr: SR,
    ch0dr: CH0DR,
    ch1dr: CH1DR,
    ch2dr: CH2DR,
    ch3dr: CH3DR,
    ch4dr: CH4DR,
    ch5dr: CH5DR,
    ch6dr: CH6DR,
    ch7dr: CH7DR,
    ch8dr: CH8DR,
    ch9dr: CH9DR,
    _reserved16: [u8; 0x10],
    ch14dr: CH14DR,
    ch15dr: CH15DR,
    sta_ext: STA_EXT,
    chany0: CHANY0,
    chany1: CHANY1,
    any_cfg: ANY_CFG,
    any_cr: ANY_CR,
    _reserved23: [u8; 0x04],
    smpr1: SMPR1,
    smpr2: SMPR2,
    _reserved25: [u8; 0x04],
    jofr0: JOFR0,
    jofr1: JOFR1,
    jofr2: JOFR2,
    jofr3: JOFR3,
    jsqr: JSQR,
    jdata: JDATA,
    _reserved31: [u8; 0x1c],
    jdr0: JDR0,
    jdr1: JDR1,
    jdr2: JDR2,
    jdr3: JDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x04 - Configure register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - Channel select register"]
    #[inline(always)]
    pub const fn chsr(&self) -> &CHSR {
        &self.chsr
    }
    #[doc = "0x10 - Compare register"]
    #[inline(always)]
    pub const fn cmpr(&self) -> &CMPR {
        &self.cmpr
    }
    #[doc = "0x14 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x18 - Channel 0 data register"]
    #[inline(always)]
    pub const fn ch0dr(&self) -> &CH0DR {
        &self.ch0dr
    }
    #[doc = "0x1c - Channel 1 data register"]
    #[inline(always)]
    pub const fn ch1dr(&self) -> &CH1DR {
        &self.ch1dr
    }
    #[doc = "0x20 - Channel 2 data register"]
    #[inline(always)]
    pub const fn ch2dr(&self) -> &CH2DR {
        &self.ch2dr
    }
    #[doc = "0x24 - Channel 3 data register"]
    #[inline(always)]
    pub const fn ch3dr(&self) -> &CH3DR {
        &self.ch3dr
    }
    #[doc = "0x28 - Channel 4 data register"]
    #[inline(always)]
    pub const fn ch4dr(&self) -> &CH4DR {
        &self.ch4dr
    }
    #[doc = "0x2c - Channel 5 data register"]
    #[inline(always)]
    pub const fn ch5dr(&self) -> &CH5DR {
        &self.ch5dr
    }
    #[doc = "0x30 - Channel 6 data register"]
    #[inline(always)]
    pub const fn ch6dr(&self) -> &CH6DR {
        &self.ch6dr
    }
    #[doc = "0x34 - Channel 7 data register"]
    #[inline(always)]
    pub const fn ch7dr(&self) -> &CH7DR {
        &self.ch7dr
    }
    #[doc = "0x38 - Channel 8 data register"]
    #[inline(always)]
    pub const fn ch8dr(&self) -> &CH8DR {
        &self.ch8dr
    }
    #[doc = "0x3c - Channel 9 data register"]
    #[inline(always)]
    pub const fn ch9dr(&self) -> &CH9DR {
        &self.ch9dr
    }
    #[doc = "0x50 - Channel 14 data register"]
    #[inline(always)]
    pub const fn ch14dr(&self) -> &CH14DR {
        &self.ch14dr
    }
    #[doc = "0x54 - Channel 15 data register"]
    #[inline(always)]
    pub const fn ch15dr(&self) -> &CH15DR {
        &self.ch15dr
    }
    #[doc = "0x58 - Extend state register"]
    #[inline(always)]
    pub const fn sta_ext(&self) -> &STA_EXT {
        &self.sta_ext
    }
    #[doc = "0x5c - Arbitrary channel channel selection register 0"]
    #[inline(always)]
    pub const fn chany0(&self) -> &CHANY0 {
        &self.chany0
    }
    #[doc = "0x60 - Arbitrary channel channel selection register 1"]
    #[inline(always)]
    pub const fn chany1(&self) -> &CHANY1 {
        &self.chany1
    }
    #[doc = "0x64 - Arbitrary channel configuration register"]
    #[inline(always)]
    pub const fn any_cfg(&self) -> &ANY_CFG {
        &self.any_cfg
    }
    #[doc = "0x68 - Arbitrary channel control register"]
    #[inline(always)]
    pub const fn any_cr(&self) -> &ANY_CR {
        &self.any_cr
    }
    #[doc = "0x70 - Any channel configuration register 1"]
    #[inline(always)]
    pub const fn smpr1(&self) -> &SMPR1 {
        &self.smpr1
    }
    #[doc = "0x74 - Any channel configuration register 2"]
    #[inline(always)]
    pub const fn smpr2(&self) -> &SMPR2 {
        &self.smpr2
    }
    #[doc = "0x7c - Injection channe 0 data compensation register"]
    #[inline(always)]
    pub const fn jofr0(&self) -> &JOFR0 {
        &self.jofr0
    }
    #[doc = "0x80 - Injection channe 1 data compensation register"]
    #[inline(always)]
    pub const fn jofr1(&self) -> &JOFR1 {
        &self.jofr1
    }
    #[doc = "0x84 - Injection channe 2 data compensation register"]
    #[inline(always)]
    pub const fn jofr2(&self) -> &JOFR2 {
        &self.jofr2
    }
    #[doc = "0x88 - Injection channe 3 data compensation register"]
    #[inline(always)]
    pub const fn jofr3(&self) -> &JOFR3 {
        &self.jofr3
    }
    #[doc = "0x8c - Injection sequence register"]
    #[inline(always)]
    pub const fn jsqr(&self) -> &JSQR {
        &self.jsqr
    }
    #[doc = "0x90 - Injection data register"]
    #[inline(always)]
    pub const fn jdata(&self) -> &JDATA {
        &self.jdata
    }
    #[doc = "0xb0 - Injection data register"]
    #[inline(always)]
    pub const fn jdr0(&self) -> &JDR0 {
        &self.jdr0
    }
    #[doc = "0xb4 - Injection data register"]
    #[inline(always)]
    pub const fn jdr1(&self) -> &JDR1 {
        &self.jdr1
    }
    #[doc = "0xb8 - Injection data register"]
    #[inline(always)]
    pub const fn jdr2(&self) -> &JDR2 {
        &self.jdr2
    }
    #[doc = "0xbc - Injection data register"]
    #[inline(always)]
    pub const fn jdr3(&self) -> &JDR3 {
        &self.jdr3
    }
}
#[doc = "DR (r) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "CFGR (rw) register accessor: Configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Configure register"]
pub mod cfgr;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "CHSR (rw) register accessor: Channel select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`]
module"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "Channel select register"]
pub mod chsr;
#[doc = "CMPR (rw) register accessor: Compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpr`]
module"]
pub type CMPR = crate::Reg<cmpr::CMPR_SPEC>;
#[doc = "Compare register"]
pub mod cmpr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CH0DR (r) register accessor: Channel 0 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0dr`]
module"]
pub type CH0DR = crate::Reg<ch0dr::CH0DR_SPEC>;
#[doc = "Channel 0 data register"]
pub mod ch0dr;
#[doc = "CH1DR (r) register accessor: Channel 1 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1dr`]
module"]
pub type CH1DR = crate::Reg<ch1dr::CH1DR_SPEC>;
#[doc = "Channel 1 data register"]
pub mod ch1dr;
#[doc = "CH2DR (r) register accessor: Channel 2 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2dr`]
module"]
pub type CH2DR = crate::Reg<ch2dr::CH2DR_SPEC>;
#[doc = "Channel 2 data register"]
pub mod ch2dr;
#[doc = "CH3DR (r) register accessor: Channel 3 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3dr`]
module"]
pub type CH3DR = crate::Reg<ch3dr::CH3DR_SPEC>;
#[doc = "Channel 3 data register"]
pub mod ch3dr;
#[doc = "CH4DR (r) register accessor: Channel 4 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4dr`]
module"]
pub type CH4DR = crate::Reg<ch4dr::CH4DR_SPEC>;
#[doc = "Channel 4 data register"]
pub mod ch4dr;
#[doc = "CH5DR (r) register accessor: Channel 5 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5dr`]
module"]
pub type CH5DR = crate::Reg<ch5dr::CH5DR_SPEC>;
#[doc = "Channel 5 data register"]
pub mod ch5dr;
#[doc = "CH6DR (r) register accessor: Channel 6 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6dr`]
module"]
pub type CH6DR = crate::Reg<ch6dr::CH6DR_SPEC>;
#[doc = "Channel 6 data register"]
pub mod ch6dr;
#[doc = "CH7DR (r) register accessor: Channel 7 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7dr`]
module"]
pub type CH7DR = crate::Reg<ch7dr::CH7DR_SPEC>;
#[doc = "Channel 7 data register"]
pub mod ch7dr;
#[doc = "CH8DR (r) register accessor: Channel 8 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8dr`]
module"]
pub type CH8DR = crate::Reg<ch8dr::CH8DR_SPEC>;
#[doc = "Channel 8 data register"]
pub mod ch8dr;
#[doc = "CH9DR (r) register accessor: Channel 9 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9dr`]
module"]
pub type CH9DR = crate::Reg<ch9dr::CH9DR_SPEC>;
#[doc = "Channel 9 data register"]
pub mod ch9dr;
#[doc = "CH14DR (r) register accessor: Channel 14 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14dr`]
module"]
pub type CH14DR = crate::Reg<ch14dr::CH14DR_SPEC>;
#[doc = "Channel 14 data register"]
pub mod ch14dr;
#[doc = "CH15DR (r) register accessor: Channel 15 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15dr`]
module"]
pub type CH15DR = crate::Reg<ch15dr::CH15DR_SPEC>;
#[doc = "Channel 15 data register"]
pub mod ch15dr;
#[doc = "STA_EXT (r) register accessor: Extend state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta_ext::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta_ext`]
module"]
pub type STA_EXT = crate::Reg<sta_ext::STA_EXT_SPEC>;
#[doc = "Extend state register"]
pub mod sta_ext;
#[doc = "CHANY0 (rw) register accessor: Arbitrary channel channel selection register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chany0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chany0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chany0`]
module"]
pub type CHANY0 = crate::Reg<chany0::CHANY0_SPEC>;
#[doc = "Arbitrary channel channel selection register 0"]
pub mod chany0;
#[doc = "CHANY1 (rw) register accessor: Arbitrary channel channel selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chany1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chany1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chany1`]
module"]
pub type CHANY1 = crate::Reg<chany1::CHANY1_SPEC>;
#[doc = "Arbitrary channel channel selection register 1"]
pub mod chany1;
#[doc = "ANY_CFG (rw) register accessor: Arbitrary channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`any_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`any_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_cfg`]
module"]
pub type ANY_CFG = crate::Reg<any_cfg::ANY_CFG_SPEC>;
#[doc = "Arbitrary channel configuration register"]
pub mod any_cfg;
#[doc = "ANY_CR (rw) register accessor: Arbitrary channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`any_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`any_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_cr`]
module"]
pub type ANY_CR = crate::Reg<any_cr::ANY_CR_SPEC>;
#[doc = "Arbitrary channel control register"]
pub mod any_cr;
#[doc = "SMPR1 (rw) register accessor: Any channel configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr1`]
module"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "Any channel configuration register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: Any channel configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr2`]
module"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "Any channel configuration register 2"]
pub mod smpr2;
#[doc = "JOFR0 (rw) register accessor: Injection channe 0 data compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jofr0`]
module"]
pub type JOFR0 = crate::Reg<jofr0::JOFR0_SPEC>;
#[doc = "Injection channe 0 data compensation register"]
pub mod jofr0;
#[doc = "JOFR1 (rw) register accessor: Injection channe 1 data compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jofr1`]
module"]
pub type JOFR1 = crate::Reg<jofr1::JOFR1_SPEC>;
#[doc = "Injection channe 1 data compensation register"]
pub mod jofr1;
#[doc = "JOFR2 (rw) register accessor: Injection channe 2 data compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jofr2`]
module"]
pub type JOFR2 = crate::Reg<jofr2::JOFR2_SPEC>;
#[doc = "Injection channe 2 data compensation register"]
pub mod jofr2;
#[doc = "JOFR3 (rw) register accessor: Injection channe 3 data compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jofr3`]
module"]
pub type JOFR3 = crate::Reg<jofr3::JOFR3_SPEC>;
#[doc = "Injection channe 3 data compensation register"]
pub mod jofr3;
#[doc = "JSQR (rw) register accessor: Injection sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jsqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jsqr`]
module"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "Injection sequence register"]
pub mod jsqr;
#[doc = "JDATA (r) register accessor: Injection data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdata`]
module"]
pub type JDATA = crate::Reg<jdata::JDATA_SPEC>;
#[doc = "Injection data register"]
pub mod jdata;
#[doc = "JDR0 (r) register accessor: Injection data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr0`]
module"]
pub type JDR0 = crate::Reg<jdr0::JDR0_SPEC>;
#[doc = "Injection data register"]
pub mod jdr0;
#[doc = "JDR1 (r) register accessor: Injection data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr1`]
module"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "Injection data register"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: Injection data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr2`]
module"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "Injection data register"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: Injection data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr3`]
module"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "Injection data register"]
pub mod jdr3;
