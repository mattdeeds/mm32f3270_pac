#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dmabsr: DMABSR,
    dmatxpdr: DMATXPDR,
    dmarxpdr: DMARXPDR,
    dmatxdsar: DMATXDSAR,
    dmatdlar: DMATDLAR,
    dmasr: DMASR,
    dmamdr: DMAMDR,
    dmaier: DMAIER,
    dmaflcr: DMAFLCR,
    dmawdtr: DMAWDTR,
    _reserved10: [u8; 0x20],
    dmacuttxdsar: DMACUTTXDSAR,
    _reserved11: [u8; 0x04],
    dmacuttxbufr: DMACUTTXBUFR,
    dmacutrxbufr: DMACUTRXBUFR,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    #[inline(always)]
    pub const fn dmabsr(&self) -> &DMABSR {
        &self.dmabsr
    }
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    #[inline(always)]
    pub const fn dmatxpdr(&self) -> &DMATXPDR {
        &self.dmatxpdr
    }
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    #[inline(always)]
    pub const fn dmarxpdr(&self) -> &DMARXPDR {
        &self.dmarxpdr
    }
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    #[inline(always)]
    pub const fn dmatxdsar(&self) -> &DMATXDSAR {
        &self.dmatxdsar
    }
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    #[inline(always)]
    pub const fn dmatdlar(&self) -> &DMATDLAR {
        &self.dmatdlar
    }
    #[doc = "0x14 - Ethernet DMA status register"]
    #[inline(always)]
    pub const fn dmasr(&self) -> &DMASR {
        &self.dmasr
    }
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    #[inline(always)]
    pub const fn dmamdr(&self) -> &DMAMDR {
        &self.dmamdr
    }
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    #[inline(always)]
    pub const fn dmaier(&self) -> &DMAIER {
        &self.dmaier
    }
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    #[inline(always)]
    pub const fn dmaflcr(&self) -> &DMAFLCR {
        &self.dmaflcr
    }
    #[doc = "0x24 - Ethernet Watchdog register"]
    #[inline(always)]
    pub const fn dmawdtr(&self) -> &DMAWDTR {
        &self.dmawdtr
    }
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    #[inline(always)]
    pub const fn dmacuttxdsar(&self) -> &DMACUTTXDSAR {
        &self.dmacuttxdsar
    }
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    #[inline(always)]
    pub const fn dmacuttxbufr(&self) -> &DMACUTTXBUFR {
        &self.dmacuttxbufr
    }
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    #[inline(always)]
    pub const fn dmacutrxbufr(&self) -> &DMACUTRXBUFR {
        &self.dmacutrxbufr
    }
}
#[doc = "DMABSR (rw) register accessor: Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabsr`]
module"]
pub type DMABSR = crate::Reg<dmabsr::DMABSR_SPEC>;
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabsr;
#[doc = "DMATXPDR (rw) register accessor: Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatxpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatxpdr`]
module"]
pub type DMATXPDR = crate::Reg<dmatxpdr::DMATXPDR_SPEC>;
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatxpdr;
#[doc = "DMARXPDR (rw) register accessor: EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarxpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarxpdr`]
module"]
pub type DMARXPDR = crate::Reg<dmarxpdr::DMARXPDR_SPEC>;
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarxpdr;
#[doc = "DMATXDSAR (rw) register accessor: Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxdsar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatxdsar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatxdsar`]
module"]
pub type DMATXDSAR = crate::Reg<dmatxdsar::DMATXDSAR_SPEC>;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmatxdsar;
#[doc = "DMATDLAR (rw) register accessor: Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatdlar`]
module"]
pub type DMATDLAR = crate::Reg<dmatdlar::DMATDLAR_SPEC>;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdlar;
#[doc = "DMASR (rw) register accessor: Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasr`]
module"]
pub type DMASR = crate::Reg<dmasr::DMASR_SPEC>;
#[doc = "Ethernet DMA status register"]
pub mod dmasr;
#[doc = "DMAMDR (rw) register accessor: Ethernet DMA operation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamdr`]
module"]
pub type DMAMDR = crate::Reg<dmamdr::DMAMDR_SPEC>;
#[doc = "Ethernet DMA operation mode register"]
pub mod dmamdr;
#[doc = "DMAIER (rw) register accessor: Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaier`]
module"]
pub type DMAIER = crate::Reg<dmaier::DMAIER_SPEC>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaier;
#[doc = "DMAFLCR (r) register accessor: Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaflcr::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaflcr`]
module"]
pub type DMAFLCR = crate::Reg<dmaflcr::DMAFLCR_SPEC>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmaflcr;
#[doc = "DMAWDTR (r) register accessor: Ethernet Watchdog register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmawdtr::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmawdtr`]
module"]
pub type DMAWDTR = crate::Reg<dmawdtr::DMAWDTR_SPEC>;
#[doc = "Ethernet Watchdog register"]
pub mod dmawdtr;
#[doc = "DMACUTTXDSAR (r) register accessor: Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacuttxdsar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacuttxdsar`]
module"]
pub type DMACUTTXDSAR = crate::Reg<dmacuttxdsar::DMACUTTXDSAR_SPEC>;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmacuttxdsar;
#[doc = "DMACUTTXBUFR (r) register accessor: Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacuttxbufr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacuttxbufr`]
module"]
pub type DMACUTTXBUFR = crate::Reg<dmacuttxbufr::DMACUTTXBUFR_SPEC>;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmacuttxbufr;
#[doc = "DMACUTRXBUFR (r) register accessor: Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacutrxbufr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacutrxbufr`]
module"]
pub type DMACUTRXBUFR = crate::Reg<dmacutrxbufr::DMACUTRXBUFR_SPEC>;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmacutrxbufr;
