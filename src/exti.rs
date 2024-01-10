#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    imr: IMR,
    emr: EMR,
    rtsr: RTSR,
    ftsr: FTSR,
    swier: SWIER,
    pr: PR,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt mask register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x04 - Event mask register"]
    #[inline(always)]
    pub const fn emr(&self) -> &EMR {
        &self.emr
    }
    #[doc = "0x08 - Rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr(&self) -> &RTSR {
        &self.rtsr
    }
    #[doc = "0x0c - Falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr(&self) -> &FTSR {
        &self.ftsr
    }
    #[doc = "0x10 - Software interrupt event register"]
    #[inline(always)]
    pub const fn swier(&self) -> &SWIER {
        &self.swier
    }
    #[doc = "0x14 - Pending register"]
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
}
#[doc = "IMR (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "EMR (rw) register accessor: Event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`]
module"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "Event mask register"]
pub mod emr;
#[doc = "RTSR (rw) register accessor: Rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr`]
module"]
pub type RTSR = crate::Reg<rtsr::RTSR_SPEC>;
#[doc = "Rising trigger selection register"]
pub mod rtsr;
#[doc = "FTSR (rw) register accessor: Falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr`]
module"]
pub type FTSR = crate::Reg<ftsr::FTSR_SPEC>;
#[doc = "Falling trigger selection register"]
pub mod ftsr;
#[doc = "SWIER (rw) register accessor: Software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier`]
module"]
pub type SWIER = crate::Reg<swier::SWIER_SPEC>;
#[doc = "Software interrupt event register"]
pub mod swier;
#[doc = "PR (rw) register accessor: Pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Pending register"]
pub mod pr;
