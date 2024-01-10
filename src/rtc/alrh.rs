#[doc = "Register `ALRH` reader"]
pub type R = crate::R<ALRH_SPEC>;
#[doc = "Register `ALRH` writer"]
pub type W = crate::W<ALRH_SPEC>;
#[doc = "Field `RTC_ALR` reader - RTC alarm high"]
pub type RTC_ALR_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_ALR` writer - RTC alarm high"]
pub type RTC_ALR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC alarm high"]
    #[inline(always)]
    pub fn rtc_alr(&self) -> RTC_ALR_R {
        RTC_ALR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm high"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_alr(&mut self) -> RTC_ALR_W<ALRH_SPEC> {
        RTC_ALR_W::new(self, 0)
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
#[doc = "Alarm high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRH_SPEC;
impl crate::RegisterSpec for ALRH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`alrh::R`](R) reader structure"]
impl crate::Readable for ALRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrh::W`](W) writer structure"]
impl crate::Writable for ALRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ALRH to value 0xffff"]
impl crate::Resettable for ALRH_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
