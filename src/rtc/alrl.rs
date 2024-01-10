#[doc = "Register `ALRL` reader"]
pub type R = crate::R<ALRL_SPEC>;
#[doc = "Register `ALRL` writer"]
pub type W = crate::W<ALRL_SPEC>;
#[doc = "Field `RTC_ALR` reader - RTC alarm low"]
pub type RTC_ALR_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_ALR` writer - RTC alarm low"]
pub type RTC_ALR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC alarm low"]
    #[inline(always)]
    pub fn rtc_alr(&self) -> RTC_ALR_R {
        RTC_ALR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm low"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_alr(&mut self) -> RTC_ALR_W<ALRL_SPEC> {
        RTC_ALR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Alarm low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRL_SPEC;
impl crate::RegisterSpec for ALRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrl::R`](R) reader structure"]
impl crate::Readable for ALRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrl::W`](W) writer structure"]
impl crate::Writable for ALRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRL to value 0xffff"]
impl crate::Resettable for ALRL_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
