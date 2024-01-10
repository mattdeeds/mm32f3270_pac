#[doc = "Register `DIVL` reader"]
pub type R = crate::R<DIVL_SPEC>;
#[doc = "Field `RTC_DIV` reader - RTC clock divider low"]
pub type RTC_DIV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RTC clock divider low"]
    #[inline(always)]
    pub fn rtc_div(&self) -> RTC_DIV_R {
        RTC_DIV_R::new(self.bits)
    }
}
#[doc = "Prescaler divider factor low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVL_SPEC;
impl crate::RegisterSpec for DIVL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`divl::R`](R) reader structure"]
impl crate::Readable for DIVL_SPEC {}
#[doc = "`reset()` method sets DIVL to value 0"]
impl crate::Resettable for DIVL_SPEC {
    const RESET_VALUE: u16 = 0;
}
