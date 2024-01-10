#[doc = "Register `CCR3FALL` reader"]
pub type R = crate::R<CCR3FALL_SPEC>;
#[doc = "Register `CCR3FALL` writer"]
pub type W = crate::W<CCR3FALL_SPEC>;
#[doc = "Field `CCR3FALL` reader - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
pub type CCR3FALL_R = crate::FieldReader<u16>;
#[doc = "Field `CCR3FALL` writer - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
pub type CCR3FALL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr3fall(&self) -> CCR3FALL_R {
        CCR3FALL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3fall(&mut self) -> CCR3FALL_W<CCR3FALL_SPEC> {
        CCR3FALL_W::new(self, 0)
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
#[doc = "pwm shift count CCR3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3fall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3fall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR3FALL_SPEC;
impl crate::RegisterSpec for CCR3FALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3fall::R`](R) reader structure"]
impl crate::Readable for CCR3FALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr3fall::W`](W) writer structure"]
impl crate::Writable for CCR3FALL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR3FALL to value 0"]
impl crate::Resettable for CCR3FALL_SPEC {
    const RESET_VALUE: u32 = 0;
}
