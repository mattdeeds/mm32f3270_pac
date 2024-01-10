#[doc = "Register `PRLL` writer"]
pub type W = crate::W<PRLL_SPEC>;
#[doc = "Field `RTC_PRL` writer - RTC prescaler reload value low"]
pub type RTC_PRL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC prescaler reload value low"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_prl(&mut self) -> RTC_PRL_W<PRLL_SPEC> {
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
#[doc = "Prescaler load low register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prll::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRLL_SPEC;
impl crate::RegisterSpec for PRLL_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`prll::W`](W) writer structure"]
impl crate::Writable for PRLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PRLL to value 0"]
impl crate::Resettable for PRLL_SPEC {
    const RESET_VALUE: u16 = 0;
}
