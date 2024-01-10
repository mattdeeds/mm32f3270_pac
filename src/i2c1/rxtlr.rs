#[doc = "Register `RXTLR` reader"]
pub type R = crate::R<RXTLR_SPEC>;
#[doc = "Register `RXTLR` writer"]
pub type W = crate::W<RXTLR_SPEC>;
#[doc = "Field `TL` reader - Receive FIFO threshold level"]
pub type TL_R = crate::FieldReader;
#[doc = "Field `TL` writer - Receive FIFO threshold level"]
pub type TL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO threshold level"]
    #[inline(always)]
    pub fn tl(&self) -> TL_R {
        TL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn tl(&mut self) -> TL_W<RXTLR_SPEC> {
        TL_W::new(self, 0)
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
#[doc = "Receive FIFO Threshold Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXTLR_SPEC;
impl crate::RegisterSpec for RXTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtlr::R`](R) reader structure"]
impl crate::Readable for RXTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxtlr::W`](W) writer structure"]
impl crate::Writable for RXTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXTLR to value 0"]
impl crate::Resettable for RXTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
