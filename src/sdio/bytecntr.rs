#[doc = "Register `BYTECNTR` reader"]
pub type R = crate::R<BYTECNTR_SPEC>;
#[doc = "Register `BYTECNTR` writer"]
pub type W = crate::W<BYTECNTR_SPEC>;
#[doc = "Field `CNT` reader - Data transfer byte counter"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Data transfer byte counter"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data transfer byte counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data transfer byte counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<BYTECNTR_SPEC> {
        CNT_W::new(self, 0)
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
#[doc = "Data transfer byte count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytecntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bytecntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BYTECNTR_SPEC;
impl crate::RegisterSpec for BYTECNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bytecntr::R`](R) reader structure"]
impl crate::Readable for BYTECNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bytecntr::W`](W) writer structure"]
impl crate::Writable for BYTECNTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BYTECNTR to value 0x0200"]
impl crate::Resettable for BYTECNTR_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
