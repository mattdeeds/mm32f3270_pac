#[doc = "Register `PDER` reader"]
pub type R = crate::R<PDER_SPEC>;
#[doc = "Register `PDER` writer"]
pub type W = crate::W<PDER_SPEC>;
#[doc = "Field `CCDREPPE` reader - DMA Update repeat mode select"]
pub type CCDREPPE_R = crate::BitReader;
#[doc = "Field `CCDREPPE` writer - DMA Update repeat mode select"]
pub type CCDREPPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR1SHIFTEN` reader - CCR1 pwm shift enable"]
pub type CCR1SHIFTEN_R = crate::BitReader;
#[doc = "Field `CCR1SHIFTEN` writer - CCR1 pwm shift enable"]
pub type CCR1SHIFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR2SHIFTEN` reader - CCR2 pwm shift enable"]
pub type CCR2SHIFTEN_R = crate::BitReader;
#[doc = "Field `CCR2SHIFTEN` writer - CCR2 pwm shift enable"]
pub type CCR2SHIFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR3SHIFTEN` reader - CCR3 pwm shift enable"]
pub type CCR3SHIFTEN_R = crate::BitReader;
#[doc = "Field `CCR3SHIFTEN` writer - CCR3 pwm shift enable"]
pub type CCR3SHIFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR4SHIFTEN` reader - CCR4 pwm shift enable"]
pub type CCR4SHIFTEN_R = crate::BitReader;
#[doc = "Field `CCR4SHIFTEN` writer - CCR4 pwm shift enable"]
pub type CCR4SHIFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR5SHIFTEN` reader - CCR5 pwm shift enable"]
pub type CCR5SHIFTEN_R = crate::BitReader;
#[doc = "Field `CCR5SHIFTEN` writer - CCR5 pwm shift enable"]
pub type CCR5SHIFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Update repeat mode select"]
    #[inline(always)]
    pub fn ccdreppe(&self) -> CCDREPPE_R {
        CCDREPPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCR1 pwm shift enable"]
    #[inline(always)]
    pub fn ccr1shiften(&self) -> CCR1SHIFTEN_R {
        CCR1SHIFTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCR2 pwm shift enable"]
    #[inline(always)]
    pub fn ccr2shiften(&self) -> CCR2SHIFTEN_R {
        CCR2SHIFTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCR3 pwm shift enable"]
    #[inline(always)]
    pub fn ccr3shiften(&self) -> CCR3SHIFTEN_R {
        CCR3SHIFTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCR4 pwm shift enable"]
    #[inline(always)]
    pub fn ccr4shiften(&self) -> CCR4SHIFTEN_R {
        CCR4SHIFTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCR5 pwm shift enable"]
    #[inline(always)]
    pub fn ccr5shiften(&self) -> CCR5SHIFTEN_R {
        CCR5SHIFTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Update repeat mode select"]
    #[inline(always)]
    #[must_use]
    pub fn ccdreppe(&mut self) -> CCDREPPE_W<PDER_SPEC> {
        CCDREPPE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CCR1 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1shiften(&mut self) -> CCR1SHIFTEN_W<PDER_SPEC> {
        CCR1SHIFTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CCR2 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr2shiften(&mut self) -> CCR2SHIFTEN_W<PDER_SPEC> {
        CCR2SHIFTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - CCR3 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3shiften(&mut self) -> CCR3SHIFTEN_W<PDER_SPEC> {
        CCR3SHIFTEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - CCR4 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4shiften(&mut self) -> CCR4SHIFTEN_W<PDER_SPEC> {
        CCR4SHIFTEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - CCR5 pwm shift enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccr5shiften(&mut self) -> CCR5SHIFTEN_W<PDER_SPEC> {
        CCR5SHIFTEN_W::new(self, 5)
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
#[doc = "PWM/DMA repeat enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDER_SPEC;
impl crate::RegisterSpec for PDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pder::R`](R) reader structure"]
impl crate::Readable for PDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pder::W`](W) writer structure"]
impl crate::Writable for PDER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDER to value 0"]
impl crate::Resettable for PDER_SPEC {
    const RESET_VALUE: u32 = 0;
}
