#[doc = "Register `WRPR1` reader"]
pub type R = crate::R<WRPR1_SPEC>;
#[doc = "Field `WRP` reader - Write protect"]
pub type WRP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(self.bits)
    }
}
#[doc = "Write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPR1_SPEC;
impl crate::RegisterSpec for WRPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr1::R`](R) reader structure"]
impl crate::Readable for WRPR1_SPEC {}
#[doc = "`reset()` method sets WRPR1 to value 0xffff_ffff"]
impl crate::Resettable for WRPR1_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
