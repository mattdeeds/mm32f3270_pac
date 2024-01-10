#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNT_SPEC>;
#[doc = "Field `CNT` reader - Current value of watchdog counter"]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Current value of watchdog counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNT_SPEC {}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
