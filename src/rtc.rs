#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x02],
    csr: CSR,
    _reserved2: [u8; 0x02],
    prlh: PRLH,
    _reserved3: [u8; 0x02],
    prll: PRLL,
    _reserved4: [u8; 0x02],
    divh: DIVH,
    _reserved5: [u8; 0x02],
    divl: DIVL,
    _reserved6: [u8; 0x02],
    cnth: CNTH,
    _reserved7: [u8; 0x02],
    cntl: CNTL,
    alrh: ALRH,
    _reserved9: [u8; 0x02],
    alrl: ALRL,
    rtc_msrh: RTC_MSRH,
    _reserved11: [u8; 0x02],
    rtc_msrl: RTC_MSRL,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - CSR"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x08 - Prescaler load high register"]
    #[inline(always)]
    pub const fn prlh(&self) -> &PRLH {
        &self.prlh
    }
    #[doc = "0x0c - Prescaler load low register"]
    #[inline(always)]
    pub const fn prll(&self) -> &PRLL {
        &self.prll
    }
    #[doc = "0x10 - Prescaler divider factor high register"]
    #[inline(always)]
    pub const fn divh(&self) -> &DIVH {
        &self.divh
    }
    #[doc = "0x14 - Prescaler divider factor low register"]
    #[inline(always)]
    pub const fn divl(&self) -> &DIVL {
        &self.divl
    }
    #[doc = "0x18 - Counter high register"]
    #[inline(always)]
    pub const fn cnth(&self) -> &CNTH {
        &self.cnth
    }
    #[doc = "0x1c - Counter low register"]
    #[inline(always)]
    pub const fn cntl(&self) -> &CNTL {
        &self.cntl
    }
    #[doc = "0x20 - Alarm high register"]
    #[inline(always)]
    pub const fn alrh(&self) -> &ALRH {
        &self.alrh
    }
    #[doc = "0x24 - Alarm low register"]
    #[inline(always)]
    pub const fn alrl(&self) -> &ALRL {
        &self.alrl
    }
    #[doc = "0x28 - RTC millisecond alarm high register"]
    #[inline(always)]
    pub const fn rtc_msrh(&self) -> &RTC_MSRH {
        &self.rtc_msrh
    }
    #[doc = "0x2c - RTC millisecond alarm low register"]
    #[inline(always)]
    pub const fn rtc_msrl(&self) -> &RTC_MSRL {
        &self.rtc_msrl
    }
}
#[doc = "CR (rw) register accessor: configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "configuration register"]
pub mod cr;
#[doc = "CSR (rw) register accessor: CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CSR"]
pub mod csr;
#[doc = "PRLH (w) register accessor: Prescaler load high register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prlh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prlh`]
module"]
pub type PRLH = crate::Reg<prlh::PRLH_SPEC>;
#[doc = "Prescaler load high register"]
pub mod prlh;
#[doc = "PRLL (w) register accessor: Prescaler load low register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prll::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prll`]
module"]
pub type PRLL = crate::Reg<prll::PRLL_SPEC>;
#[doc = "Prescaler load low register"]
pub mod prll;
#[doc = "DIVH (r) register accessor: Prescaler divider factor high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divh`]
module"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "Prescaler divider factor high register"]
pub mod divh;
#[doc = "DIVL (r) register accessor: Prescaler divider factor low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divl`]
module"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "Prescaler divider factor low register"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: Counter high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`]
module"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "Counter high register"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: Counter low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`]
module"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "Counter low register"]
pub mod cntl;
#[doc = "ALRH (rw) register accessor: Alarm high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrh`]
module"]
pub type ALRH = crate::Reg<alrh::ALRH_SPEC>;
#[doc = "Alarm high register"]
pub mod alrh;
#[doc = "ALRL (rw) register accessor: Alarm low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrl`]
module"]
pub type ALRL = crate::Reg<alrl::ALRL_SPEC>;
#[doc = "Alarm low register"]
pub mod alrl;
#[doc = "RTC_MSRH (rw) register accessor: RTC millisecond alarm high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_msrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_msrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_msrh`]
module"]
pub type RTC_MSRH = crate::Reg<rtc_msrh::RTC_MSRH_SPEC>;
#[doc = "RTC millisecond alarm high register"]
pub mod rtc_msrh;
#[doc = "RTC_MSRL (rw) register accessor: RTC millisecond alarm low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_msrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_msrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_msrl`]
module"]
pub type RTC_MSRL = crate::Reg<rtc_msrl::RTC_MSRL_SPEC>;
#[doc = "RTC millisecond alarm low register"]
pub mod rtc_msrl;
