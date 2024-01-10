#[doc = "Register `DMACUTRXBUFR` reader"]
pub type R = crate::R<DMACUTRXBUFR_SPEC>;
#[doc = "Field `CUTRXBUF` reader - Host Receive Buffer Address Pointer"]
pub type CUTRXBUF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn cutrxbuf(&self) -> CUTRXBUF_R {
        CUTRXBUF_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacutrxbufr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACUTRXBUFR_SPEC;
impl crate::RegisterSpec for DMACUTRXBUFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacutrxbufr::R`](R) reader structure"]
impl crate::Readable for DMACUTRXBUFR_SPEC {}
#[doc = "`reset()` method sets DMACUTRXBUFR to value 0"]
impl crate::Resettable for DMACUTRXBUFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
