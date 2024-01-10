#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    otg_istat: OTG_ISTAT,
    otg_ictrl: OTG_ICTRL,
    otg_stat: OTG_STAT,
    otg_ctrl: OTG_CTRL,
    _reserved4: [u8; 0x60],
    int_stat: INT_STAT,
    int_enb: INT_ENB,
    err_stat: ERR_STAT,
    err_enb: ERR_ENB,
    stat: STAT,
    ctl: CTL,
    addr: ADDR,
    bdt_page_01: BDT_PAGE_01,
    frm_numl: FRM_NUML,
    frm_numh: FRM_NUMH,
    token: TOKEN,
    sof_thld: SOF_THLD,
    bdt_page_02: BDT_PAGE_02,
    bdt_page_03: BDT_PAGE_03,
    _reserved18: [u8; 0x08],
    ep_ctl0: EP_CTL0,
    ep_ctl1: EP_CTL1,
    ep_ctl2: EP_CTL2,
    ep_ctl3: EP_CTL3,
    ep_ctl4: EP_CTL4,
    ep_ctl5: EP_CTL5,
    ep_ctl6: EP_CTL6,
    ep_ctl7: EP_CTL7,
    ep_ctl8: EP_CTL8,
    ep_ctl9: EP_CTL9,
    ep_ctl10: EP_CTL10,
    ep_ctl11: EP_CTL11,
    ep_ctl12: EP_CTL12,
    ep_ctl13: EP_CTL13,
    ep_ctl14: EP_CTL14,
    ep_ctl15: EP_CTL15,
}
impl RegisterBlock {
    #[doc = "0x10 - OTG Interrupt Status Register"]
    #[inline(always)]
    pub const fn otg_istat(&self) -> &OTG_ISTAT {
        &self.otg_istat
    }
    #[doc = "0x14 - OTG Interrupt Control Register"]
    #[inline(always)]
    pub const fn otg_ictrl(&self) -> &OTG_ICTRL {
        &self.otg_ictrl
    }
    #[doc = "0x18 - OTG Status Register"]
    #[inline(always)]
    pub const fn otg_stat(&self) -> &OTG_STAT {
        &self.otg_stat
    }
    #[doc = "0x1c - OTG Control register"]
    #[inline(always)]
    pub const fn otg_ctrl(&self) -> &OTG_CTRL {
        &self.otg_ctrl
    }
    #[doc = "0x80 - Interrupt status register"]
    #[inline(always)]
    pub const fn int_stat(&self) -> &INT_STAT {
        &self.int_stat
    }
    #[doc = "0x84 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_enb(&self) -> &INT_ENB {
        &self.int_enb
    }
    #[doc = "0x88 - Error interrupt status register"]
    #[inline(always)]
    pub const fn err_stat(&self) -> &ERR_STAT {
        &self.err_stat
    }
    #[doc = "0x8c - Error interrupt enable register"]
    #[inline(always)]
    pub const fn err_enb(&self) -> &ERR_ENB {
        &self.err_enb
    }
    #[doc = "0x90 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    #[doc = "0x94 - Control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x98 - Address register"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x9c - BDT page register 1"]
    #[inline(always)]
    pub const fn bdt_page_01(&self) -> &BDT_PAGE_01 {
        &self.bdt_page_01
    }
    #[doc = "0xa0 - Frame number register"]
    #[inline(always)]
    pub const fn frm_numl(&self) -> &FRM_NUML {
        &self.frm_numl
    }
    #[doc = "0xa4 - Frame number register"]
    #[inline(always)]
    pub const fn frm_numh(&self) -> &FRM_NUMH {
        &self.frm_numh
    }
    #[doc = "0xa8 - Token register"]
    #[inline(always)]
    pub const fn token(&self) -> &TOKEN {
        &self.token
    }
    #[doc = "0xac - SOF threshold register"]
    #[inline(always)]
    pub const fn sof_thld(&self) -> &SOF_THLD {
        &self.sof_thld
    }
    #[doc = "0xb0 - BDT page register 2"]
    #[inline(always)]
    pub const fn bdt_page_02(&self) -> &BDT_PAGE_02 {
        &self.bdt_page_02
    }
    #[doc = "0xb4 - BDT page register 3"]
    #[inline(always)]
    pub const fn bdt_page_03(&self) -> &BDT_PAGE_03 {
        &self.bdt_page_03
    }
    #[doc = "0xc0 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl0(&self) -> &EP_CTL0 {
        &self.ep_ctl0
    }
    #[doc = "0xc4 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl1(&self) -> &EP_CTL1 {
        &self.ep_ctl1
    }
    #[doc = "0xc8 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl2(&self) -> &EP_CTL2 {
        &self.ep_ctl2
    }
    #[doc = "0xcc - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl3(&self) -> &EP_CTL3 {
        &self.ep_ctl3
    }
    #[doc = "0xd0 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl4(&self) -> &EP_CTL4 {
        &self.ep_ctl4
    }
    #[doc = "0xd4 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl5(&self) -> &EP_CTL5 {
        &self.ep_ctl5
    }
    #[doc = "0xd8 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl6(&self) -> &EP_CTL6 {
        &self.ep_ctl6
    }
    #[doc = "0xdc - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl7(&self) -> &EP_CTL7 {
        &self.ep_ctl7
    }
    #[doc = "0xe0 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl8(&self) -> &EP_CTL8 {
        &self.ep_ctl8
    }
    #[doc = "0xe4 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl9(&self) -> &EP_CTL9 {
        &self.ep_ctl9
    }
    #[doc = "0xe8 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl10(&self) -> &EP_CTL10 {
        &self.ep_ctl10
    }
    #[doc = "0xec - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl11(&self) -> &EP_CTL11 {
        &self.ep_ctl11
    }
    #[doc = "0xf0 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl12(&self) -> &EP_CTL12 {
        &self.ep_ctl12
    }
    #[doc = "0xf4 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl13(&self) -> &EP_CTL13 {
        &self.ep_ctl13
    }
    #[doc = "0xf8 - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl14(&self) -> &EP_CTL14 {
        &self.ep_ctl14
    }
    #[doc = "0xfc - Endpoint control register"]
    #[inline(always)]
    pub const fn ep_ctl15(&self) -> &EP_CTL15 {
        &self.ep_ctl15
    }
}
#[doc = "OTG_ISTAT (rw) register accessor: OTG Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_istat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_istat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_istat`]
module"]
pub type OTG_ISTAT = crate::Reg<otg_istat::OTG_ISTAT_SPEC>;
#[doc = "OTG Interrupt Status Register"]
pub mod otg_istat;
#[doc = "OTG_ICTRL (rw) register accessor: OTG Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_ictrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_ictrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_ictrl`]
module"]
pub type OTG_ICTRL = crate::Reg<otg_ictrl::OTG_ICTRL_SPEC>;
#[doc = "OTG Interrupt Control Register"]
pub mod otg_ictrl;
#[doc = "OTG_STAT (rw) register accessor: OTG Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_stat`]
module"]
pub type OTG_STAT = crate::Reg<otg_stat::OTG_STAT_SPEC>;
#[doc = "OTG Status Register"]
pub mod otg_stat;
#[doc = "OTG_CTRL (rw) register accessor: OTG Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_ctrl`]
module"]
pub type OTG_CTRL = crate::Reg<otg_ctrl::OTG_CTRL_SPEC>;
#[doc = "OTG Control register"]
pub mod otg_ctrl;
#[doc = "INT_STAT (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_stat`]
module"]
pub type INT_STAT = crate::Reg<int_stat::INT_STAT_SPEC>;
#[doc = "Interrupt status register"]
pub mod int_stat;
#[doc = "INT_ENB (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_enb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_enb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_enb`]
module"]
pub type INT_ENB = crate::Reg<int_enb::INT_ENB_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_enb;
#[doc = "ERR_STAT (rw) register accessor: Error interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat`]
module"]
pub type ERR_STAT = crate::Reg<err_stat::ERR_STAT_SPEC>;
#[doc = "Error interrupt status register"]
pub mod err_stat;
#[doc = "ERR_ENB (rw) register accessor: Error interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_enb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_enb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_enb`]
module"]
pub type ERR_ENB = crate::Reg<err_enb::ERR_ENB_SPEC>;
#[doc = "Error interrupt enable register"]
pub mod err_enb;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register"]
pub mod addr;
#[doc = "BDT_PAGE_01 (rw) register accessor: BDT page register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdt_page_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdt_page_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdt_page_01`]
module"]
pub type BDT_PAGE_01 = crate::Reg<bdt_page_01::BDT_PAGE_01_SPEC>;
#[doc = "BDT page register 1"]
pub mod bdt_page_01;
#[doc = "FRM_NUML (r) register accessor: Frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_numl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frm_numl`]
module"]
pub type FRM_NUML = crate::Reg<frm_numl::FRM_NUML_SPEC>;
#[doc = "Frame number register"]
pub mod frm_numl;
#[doc = "FRM_NUMH (r) register accessor: Frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_numh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frm_numh`]
module"]
pub type FRM_NUMH = crate::Reg<frm_numh::FRM_NUMH_SPEC>;
#[doc = "Frame number register"]
pub mod frm_numh;
#[doc = "TOKEN (rw) register accessor: Token register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`token::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`token::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@token`]
module"]
pub type TOKEN = crate::Reg<token::TOKEN_SPEC>;
#[doc = "Token register"]
pub mod token;
#[doc = "SOF_THLD (rw) register accessor: SOF threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sof_thld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sof_thld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sof_thld`]
module"]
pub type SOF_THLD = crate::Reg<sof_thld::SOF_THLD_SPEC>;
#[doc = "SOF threshold register"]
pub mod sof_thld;
#[doc = "BDT_PAGE_02 (rw) register accessor: BDT page register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdt_page_02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdt_page_02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdt_page_02`]
module"]
pub type BDT_PAGE_02 = crate::Reg<bdt_page_02::BDT_PAGE_02_SPEC>;
#[doc = "BDT page register 2"]
pub mod bdt_page_02;
#[doc = "BDT_PAGE_03 (rw) register accessor: BDT page register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdt_page_03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdt_page_03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdt_page_03`]
module"]
pub type BDT_PAGE_03 = crate::Reg<bdt_page_03::BDT_PAGE_03_SPEC>;
#[doc = "BDT page register 3"]
pub mod bdt_page_03;
#[doc = "EP_CTL0 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl0`]
module"]
pub type EP_CTL0 = crate::Reg<ep_ctl0::EP_CTL0_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl0;
#[doc = "EP_CTL1 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl1`]
module"]
pub type EP_CTL1 = crate::Reg<ep_ctl1::EP_CTL1_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl1;
#[doc = "EP_CTL2 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl2`]
module"]
pub type EP_CTL2 = crate::Reg<ep_ctl2::EP_CTL2_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl2;
#[doc = "EP_CTL3 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl3`]
module"]
pub type EP_CTL3 = crate::Reg<ep_ctl3::EP_CTL3_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl3;
#[doc = "EP_CTL4 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl4`]
module"]
pub type EP_CTL4 = crate::Reg<ep_ctl4::EP_CTL4_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl4;
#[doc = "EP_CTL5 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl5`]
module"]
pub type EP_CTL5 = crate::Reg<ep_ctl5::EP_CTL5_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl5;
#[doc = "EP_CTL6 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl6`]
module"]
pub type EP_CTL6 = crate::Reg<ep_ctl6::EP_CTL6_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl6;
#[doc = "EP_CTL7 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl7`]
module"]
pub type EP_CTL7 = crate::Reg<ep_ctl7::EP_CTL7_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl7;
#[doc = "EP_CTL8 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl8`]
module"]
pub type EP_CTL8 = crate::Reg<ep_ctl8::EP_CTL8_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl8;
#[doc = "EP_CTL9 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl9`]
module"]
pub type EP_CTL9 = crate::Reg<ep_ctl9::EP_CTL9_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl9;
#[doc = "EP_CTL10 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl10`]
module"]
pub type EP_CTL10 = crate::Reg<ep_ctl10::EP_CTL10_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl10;
#[doc = "EP_CTL11 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl11`]
module"]
pub type EP_CTL11 = crate::Reg<ep_ctl11::EP_CTL11_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl11;
#[doc = "EP_CTL12 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl12`]
module"]
pub type EP_CTL12 = crate::Reg<ep_ctl12::EP_CTL12_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl12;
#[doc = "EP_CTL13 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl13`]
module"]
pub type EP_CTL13 = crate::Reg<ep_ctl13::EP_CTL13_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl13;
#[doc = "EP_CTL14 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl14`]
module"]
pub type EP_CTL14 = crate::Reg<ep_ctl14::EP_CTL14_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl14;
#[doc = "EP_CTL15 (rw) register accessor: Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_ctl15`]
module"]
pub type EP_CTL15 = crate::Reg<ep_ctl15::EP_CTL15_SPEC>;
#[doc = "Endpoint control register"]
pub mod ep_ctl15;
