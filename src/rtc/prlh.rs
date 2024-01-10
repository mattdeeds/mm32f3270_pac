#[doc = "Register `PRLH` writer"]
pub type W = crate::W<PRLH_SPEC>;
#[doc = "Field `RTC_PRL` writer - RTC prescaler reload value high"]
pub type RTC_PRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - RTC prescaler reload value high"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_prl(&mut self) -> RTC_PRL_W<PRLH_SPEC> {
        RTC_PRL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Prescaler load high register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prlh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRLH_SPEC;
impl crate::RegisterSpec for PRLH_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`prlh::W`](W) writer structure"]
impl crate::Writable for PRLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PRLH to value 0"]
impl crate::Resettable for PRLH_SPEC {
    const RESET_VALUE: u16 = 0;
}
