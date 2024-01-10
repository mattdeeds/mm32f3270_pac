#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    comp1_csr: COMP1_CSR,
    comp2_csr: COMP2_CSR,
    _reserved2: [u8; 0x04],
    comp_crv: COMP_CRV,
    comp1_poll: COMP1_POLL,
    comp2_poll: COMP2_POLL,
}
impl RegisterBlock {
    #[doc = "0x0c - COMP1 Control State Register"]
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &COMP1_CSR {
        &self.comp1_csr
    }
    #[doc = "0x10 - COMP2 Control State Register"]
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &COMP2_CSR {
        &self.comp2_csr
    }
    #[doc = "0x18 - COMP Extern Reference Voltage"]
    #[inline(always)]
    pub const fn comp_crv(&self) -> &COMP_CRV {
        &self.comp_crv
    }
    #[doc = "0x1c - COMP1 Polling Output Register"]
    #[inline(always)]
    pub const fn comp1_poll(&self) -> &COMP1_POLL {
        &self.comp1_poll
    }
    #[doc = "0x20 - COMP2 Polling Output Register"]
    #[inline(always)]
    pub const fn comp2_poll(&self) -> &COMP2_POLL {
        &self.comp2_poll
    }
}
#[doc = "COMP1_CSR (rw) register accessor: COMP1 Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_csr`]
module"]
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSR_SPEC>;
#[doc = "COMP1 Control State Register"]
pub mod comp1_csr;
#[doc = "COMP2_CSR (rw) register accessor: COMP2 Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_csr`]
module"]
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSR_SPEC>;
#[doc = "COMP2 Control State Register"]
pub mod comp2_csr;
#[doc = "COMP_CRV (rw) register accessor: COMP Extern Reference Voltage\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_crv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_crv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_crv`]
module"]
pub type COMP_CRV = crate::Reg<comp_crv::COMP_CRV_SPEC>;
#[doc = "COMP Extern Reference Voltage"]
pub mod comp_crv;
#[doc = "COMP1_POLL (rw) register accessor: COMP1 Polling Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_poll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_poll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_poll`]
module"]
pub type COMP1_POLL = crate::Reg<comp1_poll::COMP1_POLL_SPEC>;
#[doc = "COMP1 Polling Output Register"]
pub mod comp1_poll;
#[doc = "COMP2_POLL (rw) register accessor: COMP2 Polling Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_poll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_poll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_poll`]
module"]
pub type COMP2_POLL = crate::Reg<comp2_poll::COMP2_POLL_SPEC>;
#[doc = "COMP2 Polling Output Register"]
pub mod comp2_poll;
