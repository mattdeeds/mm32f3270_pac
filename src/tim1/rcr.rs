#[doc = "Register `RCR` reader"]
pub type R = crate::R<RCR_SPEC>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RCR_SPEC>;
#[doc = "Field `REP` reader - Repetition counter value"]
pub type REP_R = crate::FieldReader;
#[doc = "Field `REP` writer - Repetition counter value"]
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REP_CNT` reader - Repetition counter value of realtime writing"]
pub type REP_CNT_R = crate::FieldReader;
#[doc = "Field `REP_CNT` writer - Repetition counter value of realtime writing"]
pub type REP_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Repetition counter value of realtime writing"]
    #[inline(always)]
    pub fn rep_cnt(&self) -> REP_CNT_R {
        REP_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<RCR_SPEC> {
        REP_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Repetition counter value of realtime writing"]
    #[inline(always)]
    #[must_use]
    pub fn rep_cnt(&mut self) -> REP_CNT_W<RCR_SPEC> {
        REP_CNT_W::new(self, 8)
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
#[doc = "repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
