#[doc = "Register `FSLR` reader"]
pub type R = crate::R<FSLR_SPEC>;
#[doc = "Register `FSLR` writer"]
pub type W = crate::W<FSLR_SPEC>;
#[doc = "Field `CNT` reader - This register sets the SCL clock low period count for standard speed"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock low period count for standard speed"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<FSLR_SPEC> {
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
#[doc = "SCL Low Period Count for Fast Speed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fslr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fslr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSLR_SPEC;
impl crate::RegisterSpec for FSLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fslr::R`](R) reader structure"]
impl crate::Readable for FSLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fslr::W`](W) writer structure"]
impl crate::Writable for FSLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSLR to value 0x82"]
impl crate::Resettable for FSLR_SPEC {
    const RESET_VALUE: u32 = 0x82;
}
