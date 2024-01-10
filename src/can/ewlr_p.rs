#[doc = "Register `EWLR_P` reader"]
pub type R = crate::R<EWLR_P_SPEC>;
#[doc = "Register `EWLR_P` writer"]
pub type W = crate::W<EWLR_P_SPEC>;
#[doc = "Field `EWL` reader - Programmable error warning limit"]
pub type EWL_R = crate::FieldReader;
#[doc = "Field `EWL` writer - Programmable error warning limit"]
pub type EWL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Programmable error warning limit"]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Programmable error warning limit"]
    #[inline(always)]
    #[must_use]
    pub fn ewl(&mut self) -> EWL_W<EWLR_P_SPEC> {
        EWL_W::new(self, 0)
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
#[doc = "Peli Error Warning Limit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ewlr_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ewlr_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EWLR_P_SPEC;
impl crate::RegisterSpec for EWLR_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewlr_p::R`](R) reader structure"]
impl crate::Readable for EWLR_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ewlr_p::W`](W) writer structure"]
impl crate::Writable for EWLR_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EWLR_P to value 0x60"]
impl crate::Resettable for EWLR_P_SPEC {
    const RESET_VALUE: u32 = 0x60;
}
