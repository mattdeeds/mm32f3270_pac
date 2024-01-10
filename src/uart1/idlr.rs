#[doc = "Register `IDLR` reader"]
pub type R = crate::R<IDLR_SPEC>;
#[doc = "Register `IDLR` writer"]
pub type W = crate::W<IDLR_SPEC>;
#[doc = "Field `IDLR` reader - Idle data length register"]
pub type IDLR_R = crate::FieldReader<u16>;
#[doc = "Field `IDLR` writer - Idle data length register"]
pub type IDLR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Idle data length register"]
    #[inline(always)]
    pub fn idlr(&self) -> IDLR_R {
        IDLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Idle data length register"]
    #[inline(always)]
    #[must_use]
    pub fn idlr(&mut self) -> IDLR_W<IDLR_SPEC> {
        IDLR_W::new(self, 0)
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
#[doc = "Data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDLR_SPEC;
impl crate::RegisterSpec for IDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idlr::R`](R) reader structure"]
impl crate::Readable for IDLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idlr::W`](W) writer structure"]
impl crate::Writable for IDLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDLR to value 0x0c"]
impl crate::Resettable for IDLR_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
