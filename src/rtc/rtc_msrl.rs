#[doc = "Register `RTC_MSRL` reader"]
pub type R = crate::R<RTC_MSRL_SPEC>;
#[doc = "Register `RTC_MSRL` writer"]
pub type W = crate::W<RTC_MSRL_SPEC>;
#[doc = "Field `MSR` reader - RTC msec low"]
pub type MSR_R = crate::FieldReader<u16>;
#[doc = "Field `MSR` writer - RTC msec low"]
pub type MSR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC msec low"]
    #[inline(always)]
    pub fn msr(&self) -> MSR_R {
        MSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC msec low"]
    #[inline(always)]
    #[must_use]
    pub fn msr(&mut self) -> MSR_W<RTC_MSRL_SPEC> {
        MSR_W::new(self, 0)
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
#[doc = "RTC millisecond alarm low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_msrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_msrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_MSRL_SPEC;
impl crate::RegisterSpec for RTC_MSRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_msrl::R`](R) reader structure"]
impl crate::Readable for RTC_MSRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_msrl::W`](W) writer structure"]
impl crate::Writable for RTC_MSRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTC_MSRL to value 0"]
impl crate::Resettable for RTC_MSRL_SPEC {
    const RESET_VALUE: u16 = 0;
}
