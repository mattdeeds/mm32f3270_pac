#[doc = "Register `CCR4FALL` reader"]
pub type R = crate::R<CCR4FALL_SPEC>;
#[doc = "Register `CCR4FALL` writer"]
pub type W = crate::W<CCR4FALL_SPEC>;
#[doc = "Field `CCR4FALL` reader - Capture/compare value for ch4 when counting down in PWM center-aligned mode"]
pub type CCR4FALL_R = crate::FieldReader<u16>;
#[doc = "Field `CCR4FALL` writer - Capture/compare value for ch4 when counting down in PWM center-aligned mode"]
pub type CCR4FALL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch4 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr4fall(&self) -> CCR4FALL_R {
        CCR4FALL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch4 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4fall(&mut self) -> CCR4FALL_W<CCR4FALL_SPEC> {
        CCR4FALL_W::new(self, 0)
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
#[doc = "pwm shift count CCR4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4fall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4fall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR4FALL_SPEC;
impl crate::RegisterSpec for CCR4FALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr4fall::R`](R) reader structure"]
impl crate::Readable for CCR4FALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr4fall::W`](W) writer structure"]
impl crate::Writable for CCR4FALL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR4FALL to value 0"]
impl crate::Resettable for CCR4FALL_SPEC {
    const RESET_VALUE: u32 = 0;
}
