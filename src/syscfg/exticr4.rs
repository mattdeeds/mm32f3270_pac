#[doc = "Register `EXTICR4` reader"]
pub type R = crate::R<EXTICR4_SPEC>;
#[doc = "Register `EXTICR4` writer"]
pub type W = crate::W<EXTICR4_SPEC>;
#[doc = "Field `EXTI12` reader - EXTI 12 configuration"]
pub type EXTI12_R = crate::FieldReader;
#[doc = "Field `EXTI12` writer - EXTI 12 configuration"]
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI13` reader - EXTI 13 configuration"]
pub type EXTI13_R = crate::FieldReader;
#[doc = "Field `EXTI13` writer - EXTI 13 configuration"]
pub type EXTI13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI14` reader - EXTI 14 configuration"]
pub type EXTI14_R = crate::FieldReader;
#[doc = "Field `EXTI14` writer - EXTI 14 configuration"]
pub type EXTI14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI15` reader - EXTI 15 configuration"]
pub type EXTI15_R = crate::FieldReader;
#[doc = "Field `EXTI15` writer - EXTI 15 configuration"]
pub type EXTI15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI 12 configuration"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 configuration"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 configuration"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 15 configuration"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 12 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> EXTI12_W<EXTICR4_SPEC> {
        EXTI12_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 13 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> EXTI13_W<EXTICR4_SPEC> {
        EXTI13_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 14 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> EXTI14_W<EXTICR4_SPEC> {
        EXTI14_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 15 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> EXTI15_W<EXTICR4_SPEC> {
        EXTI15_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`exticr4::R`](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr4::W`](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    const RESET_VALUE: u16 = 0;
}
