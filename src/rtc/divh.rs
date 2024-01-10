#[doc = "Register `DIVH` reader"]
pub type R = crate::R<DIVH_SPEC>;
#[doc = "Field `RTC_DIV` reader - RTC clock divider high"]
pub type RTC_DIV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - RTC clock divider high"]
    #[inline(always)]
    pub fn rtc_div(&self) -> RTC_DIV_R {
        RTC_DIV_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Prescaler divider factor high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVH_SPEC;
impl crate::RegisterSpec for DIVH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`divh::R`](R) reader structure"]
impl crate::Readable for DIVH_SPEC {}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DIVH_SPEC {
    const RESET_VALUE: u16 = 0;
}
