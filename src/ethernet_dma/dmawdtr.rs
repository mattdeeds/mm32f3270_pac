#[doc = "Register `DMAWDTR` reader"]
pub type R = crate::R<DMAWDTR_SPEC>;
#[doc = "Field `RIWT` reader - *D0"]
pub type RIWT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - *D0"]
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Ethernet Watchdog register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmawdtr::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAWDTR_SPEC;
impl crate::RegisterSpec for DMAWDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmawdtr::R`](R) reader structure"]
impl crate::Readable for DMAWDTR_SPEC {}
#[doc = "`reset()` method sets DMAWDTR to value 0"]
impl crate::Resettable for DMAWDTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
