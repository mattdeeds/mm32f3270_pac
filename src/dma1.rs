#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    isr: ISR,
    ifcr: IFCR,
    ccr1: CCR1,
    cndtr1: CNDTR1,
    cpar1: CPAR1,
    cmar1: CMAR1,
    _reserved6: [u8; 0x04],
    ccr2: CCR2,
    cndtr2: CNDTR2,
    cpar2: CPAR2,
    cmar2: CMAR2,
    _reserved10: [u8; 0x04],
    ccr3: CCR3,
    cndtr3: CNDTR3,
    cpar3: CPAR3,
    cmar3: CMAR3,
    _reserved14: [u8; 0x04],
    ccr4: CCR4,
    cndtr4: CNDTR4,
    cpar4: CPAR4,
    cmar4: CMAR4,
    _reserved18: [u8; 0x04],
    ccr5: CCR5,
    cndtr5: CNDTR5,
    cpar5: CPAR5,
    cmar5: CMAR5,
    _reserved22: [u8; 0x04],
    ccr6: CCR6,
    cndtr6: CNDTR6,
    cpar6: CPAR6,
    cmar6: CMAR6,
    _reserved26: [u8; 0x04],
    ccr7: CCR7,
    cndtr7: CNDTR7,
    cpar7: CPAR7,
    cmar7: CMAR7,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x08 - Channel 1 configuration register"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    #[doc = "0x0c - Channel 1 number of data register"]
    #[inline(always)]
    pub const fn cndtr1(&self) -> &CNDTR1 {
        &self.cndtr1
    }
    #[doc = "0x10 - Channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn cpar1(&self) -> &CPAR1 {
        &self.cpar1
    }
    #[doc = "0x14 - Channel 1 memory address register"]
    #[inline(always)]
    pub const fn cmar1(&self) -> &CMAR1 {
        &self.cmar1
    }
    #[doc = "0x1c - Channel 2 configuration register"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    #[doc = "0x20 - Channel 2 number of data register"]
    #[inline(always)]
    pub const fn cndtr2(&self) -> &CNDTR2 {
        &self.cndtr2
    }
    #[doc = "0x24 - Channel 2 peripheral address register"]
    #[inline(always)]
    pub const fn cpar2(&self) -> &CPAR2 {
        &self.cpar2
    }
    #[doc = "0x28 - Channel 2 memory address register"]
    #[inline(always)]
    pub const fn cmar2(&self) -> &CMAR2 {
        &self.cmar2
    }
    #[doc = "0x30 - Channel 3 configuration register"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR3 {
        &self.ccr3
    }
    #[doc = "0x34 - Channel 3 number of data register"]
    #[inline(always)]
    pub const fn cndtr3(&self) -> &CNDTR3 {
        &self.cndtr3
    }
    #[doc = "0x38 - Channel 3 peripheral address register"]
    #[inline(always)]
    pub const fn cpar3(&self) -> &CPAR3 {
        &self.cpar3
    }
    #[doc = "0x3c - Channel 3 memory address register"]
    #[inline(always)]
    pub const fn cmar3(&self) -> &CMAR3 {
        &self.cmar3
    }
    #[doc = "0x44 - Channel 4 configuration register"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR4 {
        &self.ccr4
    }
    #[doc = "0x48 - Channel 4 number of data register"]
    #[inline(always)]
    pub const fn cndtr4(&self) -> &CNDTR4 {
        &self.cndtr4
    }
    #[doc = "0x4c - Channel 4 peripheral address register"]
    #[inline(always)]
    pub const fn cpar4(&self) -> &CPAR4 {
        &self.cpar4
    }
    #[doc = "0x50 - Channel 4 memory address register"]
    #[inline(always)]
    pub const fn cmar4(&self) -> &CMAR4 {
        &self.cmar4
    }
    #[doc = "0x58 - Channel 5 configuration register"]
    #[inline(always)]
    pub const fn ccr5(&self) -> &CCR5 {
        &self.ccr5
    }
    #[doc = "0x5c - Channel 5 number of data register"]
    #[inline(always)]
    pub const fn cndtr5(&self) -> &CNDTR5 {
        &self.cndtr5
    }
    #[doc = "0x60 - Channel 5 peripheral address register"]
    #[inline(always)]
    pub const fn cpar5(&self) -> &CPAR5 {
        &self.cpar5
    }
    #[doc = "0x64 - Channel 5 memory address register"]
    #[inline(always)]
    pub const fn cmar5(&self) -> &CMAR5 {
        &self.cmar5
    }
    #[doc = "0x6c - Channel 6 configuration register"]
    #[inline(always)]
    pub const fn ccr6(&self) -> &CCR6 {
        &self.ccr6
    }
    #[doc = "0x70 - Channel 6 number of data register"]
    #[inline(always)]
    pub const fn cndtr6(&self) -> &CNDTR6 {
        &self.cndtr6
    }
    #[doc = "0x74 - Channel 6 peripheral address register"]
    #[inline(always)]
    pub const fn cpar6(&self) -> &CPAR6 {
        &self.cpar6
    }
    #[doc = "0x78 - Channel 6 memory address register"]
    #[inline(always)]
    pub const fn cmar6(&self) -> &CMAR6 {
        &self.cmar6
    }
    #[doc = "0x80 - Channel 7 configuration register"]
    #[inline(always)]
    pub const fn ccr7(&self) -> &CCR7 {
        &self.ccr7
    }
    #[doc = "0x84 - Channel 7 number of data register"]
    #[inline(always)]
    pub const fn cndtr7(&self) -> &CNDTR7 {
        &self.cndtr7
    }
    #[doc = "0x88 - Channel 7 peripheral address register"]
    #[inline(always)]
    pub const fn cpar7(&self) -> &CPAR7 {
        &self.cpar7
    }
    #[doc = "0x8c - Channel 7 memory address register"]
    #[inline(always)]
    pub const fn cmar7(&self) -> &CMAR7 {
        &self.cmar7
    }
}
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: Channel 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "Channel 1 configuration register"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: Channel 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "Channel 2 configuration register"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: Channel 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`]
module"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "Channel 3 configuration register"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: Channel 4 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`]
module"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "Channel 4 configuration register"]
pub mod ccr4;
#[doc = "CCR5 (rw) register accessor: Channel 5 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5`]
module"]
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
#[doc = "Channel 5 configuration register"]
pub mod ccr5;
#[doc = "CCR6 (rw) register accessor: Channel 6 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr6`]
module"]
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
#[doc = "Channel 6 configuration register"]
pub mod ccr6;
#[doc = "CCR7 (rw) register accessor: Channel 7 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr7`]
module"]
pub type CCR7 = crate::Reg<ccr7::CCR7_SPEC>;
#[doc = "Channel 7 configuration register"]
pub mod ccr7;
#[doc = "CNDTR1 (rw) register accessor: Channel 1 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr1`]
module"]
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1_SPEC>;
#[doc = "Channel 1 number of data register"]
pub mod cndtr1;
#[doc = "CNDTR2 (rw) register accessor: Channel 2 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr2`]
module"]
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2_SPEC>;
#[doc = "Channel 2 number of data register"]
pub mod cndtr2;
#[doc = "CNDTR3 (rw) register accessor: Channel 3 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr3`]
module"]
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3_SPEC>;
#[doc = "Channel 3 number of data register"]
pub mod cndtr3;
#[doc = "CNDTR4 (rw) register accessor: Channel 4 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr4`]
module"]
pub type CNDTR4 = crate::Reg<cndtr4::CNDTR4_SPEC>;
#[doc = "Channel 4 number of data register"]
pub mod cndtr4;
#[doc = "CNDTR5 (rw) register accessor: Channel 5 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr5`]
module"]
pub type CNDTR5 = crate::Reg<cndtr5::CNDTR5_SPEC>;
#[doc = "Channel 5 number of data register"]
pub mod cndtr5;
#[doc = "CNDTR6 (rw) register accessor: Channel 6 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr6`]
module"]
pub type CNDTR6 = crate::Reg<cndtr6::CNDTR6_SPEC>;
#[doc = "Channel 6 number of data register"]
pub mod cndtr6;
#[doc = "CNDTR7 (rw) register accessor: Channel 7 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr7`]
module"]
pub type CNDTR7 = crate::Reg<cndtr7::CNDTR7_SPEC>;
#[doc = "Channel 7 number of data register"]
pub mod cndtr7;
#[doc = "CPAR1 (rw) register accessor: Channel 1 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar1`]
module"]
pub type CPAR1 = crate::Reg<cpar1::CPAR1_SPEC>;
#[doc = "Channel 1 peripheral address register"]
pub mod cpar1;
#[doc = "CPAR2 (rw) register accessor: Channel 2 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar2`]
module"]
pub type CPAR2 = crate::Reg<cpar2::CPAR2_SPEC>;
#[doc = "Channel 2 peripheral address register"]
pub mod cpar2;
#[doc = "CPAR3 (rw) register accessor: Channel 3 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar3`]
module"]
pub type CPAR3 = crate::Reg<cpar3::CPAR3_SPEC>;
#[doc = "Channel 3 peripheral address register"]
pub mod cpar3;
#[doc = "CPAR4 (rw) register accessor: Channel 4 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar4`]
module"]
pub type CPAR4 = crate::Reg<cpar4::CPAR4_SPEC>;
#[doc = "Channel 4 peripheral address register"]
pub mod cpar4;
#[doc = "CPAR5 (rw) register accessor: Channel 5 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar5`]
module"]
pub type CPAR5 = crate::Reg<cpar5::CPAR5_SPEC>;
#[doc = "Channel 5 peripheral address register"]
pub mod cpar5;
#[doc = "CPAR6 (rw) register accessor: Channel 6 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar6`]
module"]
pub type CPAR6 = crate::Reg<cpar6::CPAR6_SPEC>;
#[doc = "Channel 6 peripheral address register"]
pub mod cpar6;
#[doc = "CPAR7 (rw) register accessor: Channel 7 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar7`]
module"]
pub type CPAR7 = crate::Reg<cpar7::CPAR7_SPEC>;
#[doc = "Channel 7 peripheral address register"]
pub mod cpar7;
#[doc = "CMAR1 (rw) register accessor: Channel 1 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar1`]
module"]
pub type CMAR1 = crate::Reg<cmar1::CMAR1_SPEC>;
#[doc = "Channel 1 memory address register"]
pub mod cmar1;
#[doc = "CMAR2 (rw) register accessor: Channel 2 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar2`]
module"]
pub type CMAR2 = crate::Reg<cmar2::CMAR2_SPEC>;
#[doc = "Channel 2 memory address register"]
pub mod cmar2;
#[doc = "CMAR3 (rw) register accessor: Channel 3 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar3`]
module"]
pub type CMAR3 = crate::Reg<cmar3::CMAR3_SPEC>;
#[doc = "Channel 3 memory address register"]
pub mod cmar3;
#[doc = "CMAR4 (rw) register accessor: Channel 4 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar4`]
module"]
pub type CMAR4 = crate::Reg<cmar4::CMAR4_SPEC>;
#[doc = "Channel 4 memory address register"]
pub mod cmar4;
#[doc = "CMAR5 (rw) register accessor: Channel 5 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar5`]
module"]
pub type CMAR5 = crate::Reg<cmar5::CMAR5_SPEC>;
#[doc = "Channel 5 memory address register"]
pub mod cmar5;
#[doc = "CMAR6 (rw) register accessor: Channel 6 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar6`]
module"]
pub type CMAR6 = crate::Reg<cmar6::CMAR6_SPEC>;
#[doc = "Channel 6 memory address register"]
pub mod cmar6;
#[doc = "CMAR7 (rw) register accessor: Channel 7 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar7`]
module"]
pub type CMAR7 = crate::Reg<cmar7::CMAR7_SPEC>;
#[doc = "Channel 7 memory address register"]
pub mod cmar7;
