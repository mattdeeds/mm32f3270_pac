#[doc = "Register `COMP2_POLL` reader"]
pub type R = crate::R<COMP2_POLL_SPEC>;
#[doc = "Register `COMP2_POLL` writer"]
pub type W = crate::W<COMP2_POLL_SPEC>;
#[doc = "Field `EN` reader - Comparator 2 polling enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 2 polling enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH` reader - Comparator 2 polling channel"]
pub type CH_R = crate::BitReader;
#[doc = "Field `CH` writer - Comparator 2 polling channel"]
pub type CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXN` reader - Comparator 2 Polling inverting input fix"]
pub type FIXN_R = crate::BitReader;
#[doc = "Field `FIXN` writer - Comparator 2 Polling inverting input fix"]
pub type FIXN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIOD` reader - Comparator 2 polling wait cycle"]
pub type PERIOD_R = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Comparator 2 polling wait cycle"]
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POUT` reader - Comparator 2 Polling output"]
pub type POUT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Comparator 2 polling enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 2 polling channel"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&self) -> FIXN_R {
        FIXN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 polling wait cycle"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 2 Polling output"]
    #[inline(always)]
    pub fn pout(&self) -> POUT_R {
        POUT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 polling enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<COMP2_POLL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 2 polling channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<COMP2_POLL_SPEC> {
        CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator 2 Polling inverting input fix"]
    #[inline(always)]
    #[must_use]
    pub fn fixn(&mut self) -> FIXN_W<COMP2_POLL_SPEC> {
        FIXN_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 2 polling wait cycle"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<COMP2_POLL_SPEC> {
        PERIOD_W::new(self, 4)
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
#[doc = "COMP2 Polling Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_poll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_poll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP2_POLL_SPEC;
impl crate::RegisterSpec for COMP2_POLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_poll::R`](R) reader structure"]
impl crate::Readable for COMP2_POLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comp2_poll::W`](W) writer structure"]
impl crate::Writable for COMP2_POLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP2_POLL to value 0"]
impl crate::Resettable for COMP2_POLL_SPEC {
    const RESET_VALUE: u32 = 0;
}
