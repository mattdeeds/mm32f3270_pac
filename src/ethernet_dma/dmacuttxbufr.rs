#[doc = "Register `DMACUTTXBUFR` reader"]
pub type R = crate::R<DMACUTTXBUFR_SPEC>;
#[doc = "Field `CUTTXBUF` reader - Host Transmit Buffer Address Pointer"]
pub type CUTTXBUF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn cuttxbuf(&self) -> CUTTXBUF_R {
        CUTTXBUF_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacuttxbufr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACUTTXBUFR_SPEC;
impl crate::RegisterSpec for DMACUTTXBUFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacuttxbufr::R`](R) reader structure"]
impl crate::Readable for DMACUTTXBUFR_SPEC {}
#[doc = "`reset()` method sets DMACUTTXBUFR to value 0"]
impl crate::Resettable for DMACUTTXBUFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
