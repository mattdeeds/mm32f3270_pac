#[doc = "Register `CNTH` reader"]
pub type R = crate::R<CNTH_SPEC>;
#[doc = "Register `CNTH` writer"]
pub type W = crate::W<CNTH_SPEC>;
#[doc = "Field `RTC_CNT` reader - RTC counter high"]
pub type RTC_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_CNT` writer - RTC counter high"]
pub type RTC_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC counter high"]
    #[inline(always)]
    pub fn rtc_cnt(&self) -> RTC_CNT_R {
        RTC_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter high"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cnt(&mut self) -> RTC_CNT_W<CNTH_SPEC> {
        RTC_CNT_W::new(self, 0)
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
#[doc = "Counter high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTH_SPEC;
impl crate::RegisterSpec for CNTH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cnth::R`](R) reader structure"]
impl crate::Readable for CNTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnth::W`](W) writer structure"]
impl crate::Writable for CNTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CNTH_SPEC {
    const RESET_VALUE: u16 = 0;
}
