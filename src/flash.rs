#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    acr: ACR,
    keyr: KEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    cr: CR,
    ar: AR,
    _reserved6: [u8; 0x04],
    obr: OBR,
    wrpr1: WRPR1,
    wrpr2: WRPR2,
    wrpr3: WRPR3,
    wrpr4: WRPR4,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0x04 - Flash key"]
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    #[doc = "0x08 - Option byte key"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    #[doc = "0x0c - Flash status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x10 - Flash control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x14 - Flash address register"]
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    #[doc = "0x1c - Option byte register"]
    #[inline(always)]
    pub const fn obr(&self) -> &OBR {
        &self.obr
    }
    #[doc = "0x20 - Write protect register"]
    #[inline(always)]
    pub const fn wrpr1(&self) -> &WRPR1 {
        &self.wrpr1
    }
    #[doc = "0x24 - Write protect register"]
    #[inline(always)]
    pub const fn wrpr2(&self) -> &WRPR2 {
        &self.wrpr2
    }
    #[doc = "0x28 - Write protect register"]
    #[inline(always)]
    pub const fn wrpr3(&self) -> &WRPR3 {
        &self.wrpr3
    }
    #[doc = "0x2c - Write protect register"]
    #[inline(always)]
    pub const fn wrpr4(&self) -> &WRPR4 {
        &self.wrpr4
    }
}
#[doc = "ACR (rw) register accessor: Flash access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Flash access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: Flash key\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: Option byte key\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Option byte key"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: Flash status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Flash status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Flash control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "AR (w) register accessor: Flash address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "Flash address register"]
pub mod ar;
#[doc = "OBR (r) register accessor: Option byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obr`]
module"]
pub type OBR = crate::Reg<obr::OBR_SPEC>;
#[doc = "Option byte register"]
pub mod obr;
#[doc = "WRPR1 (r) register accessor: Write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr1`]
module"]
pub type WRPR1 = crate::Reg<wrpr1::WRPR1_SPEC>;
#[doc = "Write protect register"]
pub mod wrpr1;
#[doc = "WRPR2 (r) register accessor: Write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr2`]
module"]
pub type WRPR2 = crate::Reg<wrpr2::WRPR2_SPEC>;
#[doc = "Write protect register"]
pub mod wrpr2;
#[doc = "WRPR3 (r) register accessor: Write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr3`]
module"]
pub type WRPR3 = crate::Reg<wrpr3::WRPR3_SPEC>;
#[doc = "Write protect register"]
pub mod wrpr3;
#[doc = "WRPR4 (r) register accessor: Write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr4`]
module"]
pub type WRPR4 = crate::Reg<wrpr4::WRPR4_SPEC>;
#[doc = "Write protect register"]
pub mod wrpr4;
