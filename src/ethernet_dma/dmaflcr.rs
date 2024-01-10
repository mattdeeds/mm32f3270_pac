#[doc = "Register `DMAFLCR` reader"]
pub type R = crate::R<DMAFLCR_SPEC>;
#[doc = "Field `BNAC` reader - Missed frames by the controller"]
pub type BNAC_R = crate::FieldReader<u16>;
#[doc = "Field `BNAF` reader - Overflow bit for missed framecounter"]
pub type BNAF_R = crate::BitReader;
#[doc = "Field `OVFC` reader - Missed frames by the application"]
pub type OVFC_R = crate::FieldReader<u16>;
#[doc = "Field `OVFF` reader - Overflow bit for FIFO overflowcounter"]
pub type OVFF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    pub fn bnac(&self) -> BNAC_R {
        BNAC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed framecounter"]
    #[inline(always)]
    pub fn bnaf(&self) -> BNAF_R {
        BNAF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    pub fn ovfc(&self) -> OVFC_R {
        OVFC_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflowcounter"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaflcr::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAFLCR_SPEC;
impl crate::RegisterSpec for DMAFLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaflcr::R`](R) reader structure"]
impl crate::Readable for DMAFLCR_SPEC {}
#[doc = "`reset()` method sets DMAFLCR to value 0"]
impl crate::Resettable for DMAFLCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
