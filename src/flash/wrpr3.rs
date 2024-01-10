#[doc = "Register `WRPR3` reader"]
pub type R = crate::R<WRPR3_SPEC>;
#[doc = "Field `WRP` reader - Write protect"]
pub type WRP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(self.bits)
    }
}
#[doc = "Write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPR3_SPEC;
impl crate::RegisterSpec for WRPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr3::R`](R) reader structure"]
impl crate::Readable for WRPR3_SPEC {}
#[doc = "`reset()` method sets WRPR3 to value 0xffff_ffff"]
impl crate::Resettable for WRPR3_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
