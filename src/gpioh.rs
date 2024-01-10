#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    crl: CRL,
    crh: CRH,
    idr: IDR,
    odr: ODR,
    bsrr: BSRR,
    brr: BRR,
    lckr: LCKR,
    dcr: DCR,
    afrl: AFRL,
    afrh: AFRH,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration low register"]
    #[inline(always)]
    pub const fn crl(&self) -> &CRL {
        &self.crl
    }
    #[doc = "0x04 - configuration high register"]
    #[inline(always)]
    pub const fn crh(&self) -> &CRH {
        &self.crh
    }
    #[doc = "0x08 - input data register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x0c - output data register"]
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    #[doc = "0x10 - bit set/reset register"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    #[doc = "0x14 - bit reset register"]
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    #[doc = "0x18 - Port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
    #[doc = "0x1c - Port output open drain control register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    #[doc = "0x20 - Port Multiplexing Function Low Register"]
    #[inline(always)]
    pub const fn afrl(&self) -> &AFRL {
        &self.afrl
    }
    #[doc = "0x24 - Port Multiplexing Function High Register"]
    #[inline(always)]
    pub const fn afrh(&self) -> &AFRH {
        &self.afrh
    }
}
#[doc = "CRL (rw) register accessor: configuration low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crl`]
module"]
pub type CRL = crate::Reg<crl::CRL_SPEC>;
#[doc = "configuration low register"]
pub mod crl;
#[doc = "CRH (rw) register accessor: configuration high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crh`]
module"]
pub type CRH = crate::Reg<crh::CRH_SPEC>;
#[doc = "configuration high register"]
pub mod crh;
#[doc = "IDR (r) register accessor: input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "input data register"]
pub mod idr;
#[doc = "ODR (rw) register accessor: output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "output data register"]
pub mod odr;
#[doc = "BSRR (w) register accessor: bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`]
module"]
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
#[doc = "bit set/reset register"]
pub mod bsrr;
#[doc = "BRR (w) register accessor: bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "bit reset register"]
pub mod brr;
#[doc = "LCKR (rw) register accessor: Port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`]
module"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "Port configuration lock register"]
pub mod lckr;
#[doc = "DCR (rw) register accessor: Port output open drain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`]
module"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "Port output open drain control register"]
pub mod dcr;
#[doc = "AFRL (rw) register accessor: Port Multiplexing Function Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`]
module"]
pub type AFRL = crate::Reg<afrl::AFRL_SPEC>;
#[doc = "Port Multiplexing Function Low Register"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: Port Multiplexing Function High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrh`]
module"]
pub type AFRH = crate::Reg<afrh::AFRH_SPEC>;
#[doc = "Port Multiplexing Function High Register"]
pub mod afrh;
