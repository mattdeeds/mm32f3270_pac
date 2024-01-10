#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    rtccr: RTCCR,
    _reserved1: [u8; 0x02],
    cr: CR,
    _reserved2: [u8; 0x02],
    csr: CSR,
    _reserved3: [u8; 0x06],
    dr1: DR1,
    dr2: DR2,
    dr3: DR3,
    dr4: DR4,
    dr5: DR5,
    dr6: DR6,
    dr7: DR7,
    dr8: DR8,
    dr9: DR9,
    dr10: DR10,
    dr11: DR11,
    dr12: DR12,
    dr13: DR13,
    dr14: DR14,
    dr15: DR15,
    dr16: DR16,
    dr17: DR17,
    dr18: DR18,
    dr19: DR19,
    dr20: DR20,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC clock calibration register"]
    #[inline(always)]
    pub const fn rtccr(&self) -> &RTCCR {
        &self.rtccr
    }
    #[doc = "0x04 - Backup control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x08 - BKP control/status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x10 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr1(&self) -> &DR1 {
        &self.dr1
    }
    #[doc = "0x14 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr2(&self) -> &DR2 {
        &self.dr2
    }
    #[doc = "0x18 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr3(&self) -> &DR3 {
        &self.dr3
    }
    #[doc = "0x1c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr4(&self) -> &DR4 {
        &self.dr4
    }
    #[doc = "0x20 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr5(&self) -> &DR5 {
        &self.dr5
    }
    #[doc = "0x24 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr6(&self) -> &DR6 {
        &self.dr6
    }
    #[doc = "0x28 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr7(&self) -> &DR7 {
        &self.dr7
    }
    #[doc = "0x2c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        &self.dr8
    }
    #[doc = "0x30 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr9(&self) -> &DR9 {
        &self.dr9
    }
    #[doc = "0x34 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr10(&self) -> &DR10 {
        &self.dr10
    }
    #[doc = "0x38 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr11(&self) -> &DR11 {
        &self.dr11
    }
    #[doc = "0x3c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr12(&self) -> &DR12 {
        &self.dr12
    }
    #[doc = "0x40 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr13(&self) -> &DR13 {
        &self.dr13
    }
    #[doc = "0x44 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr14(&self) -> &DR14 {
        &self.dr14
    }
    #[doc = "0x48 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr15(&self) -> &DR15 {
        &self.dr15
    }
    #[doc = "0x4c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr16(&self) -> &DR16 {
        &self.dr16
    }
    #[doc = "0x50 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr17(&self) -> &DR17 {
        &self.dr17
    }
    #[doc = "0x54 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr18(&self) -> &DR18 {
        &self.dr18
    }
    #[doc = "0x58 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr19(&self) -> &DR19 {
        &self.dr19
    }
    #[doc = "0x5c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr20(&self) -> &DR20 {
        &self.dr20
    }
}
#[doc = "RTCCR (rw) register accessor: RTC clock calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccr`]
module"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "RTC clock calibration register"]
pub mod rtccr;
#[doc = "CR (rw) register accessor: Backup control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Backup control register"]
pub mod cr;
#[doc = "CSR (rw) register accessor: BKP control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "BKP control/status register"]
pub mod csr;
#[doc = "DR1 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`]
module"]
pub type DR1 = crate::Reg<dr1::DR1_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`]
module"]
pub type DR2 = crate::Reg<dr2::DR2_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`]
module"]
pub type DR3 = crate::Reg<dr3::DR3_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr3;
#[doc = "DR4 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`]
module"]
pub type DR4 = crate::Reg<dr4::DR4_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr4;
#[doc = "DR5 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`]
module"]
pub type DR5 = crate::Reg<dr5::DR5_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr5;
#[doc = "DR6 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`]
module"]
pub type DR6 = crate::Reg<dr6::DR6_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr6;
#[doc = "DR7 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`]
module"]
pub type DR7 = crate::Reg<dr7::DR7_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr7;
#[doc = "DR8 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`]
module"]
pub type DR8 = crate::Reg<dr8::DR8_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr8;
#[doc = "DR9 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr9`]
module"]
pub type DR9 = crate::Reg<dr9::DR9_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr9;
#[doc = "DR10 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr10`]
module"]
pub type DR10 = crate::Reg<dr10::DR10_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr10;
#[doc = "DR11 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr11`]
module"]
pub type DR11 = crate::Reg<dr11::DR11_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr11;
#[doc = "DR12 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr12`]
module"]
pub type DR12 = crate::Reg<dr12::DR12_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr12;
#[doc = "DR13 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr13`]
module"]
pub type DR13 = crate::Reg<dr13::DR13_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr13;
#[doc = "DR14 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr14`]
module"]
pub type DR14 = crate::Reg<dr14::DR14_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr14;
#[doc = "DR15 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr15`]
module"]
pub type DR15 = crate::Reg<dr15::DR15_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr15;
#[doc = "DR16 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr16`]
module"]
pub type DR16 = crate::Reg<dr16::DR16_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr16;
#[doc = "DR17 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr17`]
module"]
pub type DR17 = crate::Reg<dr17::DR17_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr17;
#[doc = "DR18 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr18`]
module"]
pub type DR18 = crate::Reg<dr18::DR18_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr18;
#[doc = "DR19 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr19`]
module"]
pub type DR19 = crate::Reg<dr19::DR19_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr19;
#[doc = "DR20 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr20`]
module"]
pub type DR20 = crate::Reg<dr20::DR20_SPEC>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr20;
