#[doc = "Register `EXTICR1` reader"]
pub type R = crate::R<EXTICR1_SPEC>;
#[doc = "Register `EXTICR1` writer"]
pub type W = crate::W<EXTICR1_SPEC>;
#[doc = "Field `EXTI0` reader - EXTI 0 configuration"]
pub type EXTI0_R = crate::FieldReader;
#[doc = "Field `EXTI0` writer - EXTI 0 configuration"]
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI1` reader - EXTI 1 configuration"]
pub type EXTI1_R = crate::FieldReader;
#[doc = "Field `EXTI1` writer - EXTI 1 configuration"]
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI2` reader - EXTI 2 configuration"]
pub type EXTI2_R = crate::FieldReader;
#[doc = "Field `EXTI2` writer - EXTI 2 configuration"]
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI3` reader - EXTI 3 configuration"]
pub type EXTI3_R = crate::FieldReader;
#[doc = "Field `EXTI3` writer - EXTI 3 configuration"]
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI 0 configuration"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 3 configuration"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 0 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<EXTICR1_SPEC> {
        EXTI0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<EXTICR1_SPEC> {
        EXTI1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<EXTICR1_SPEC> {
        EXTI2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 3 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<EXTICR1_SPEC> {
        EXTI3_W::new(self, 12)
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
#[doc = "External interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR1_SPEC;
impl crate::RegisterSpec for EXTICR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`exticr1::R`](R) reader structure"]
impl crate::Readable for EXTICR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr1::W`](W) writer structure"]
impl crate::Writable for EXTICR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EXTICR1 to value 0"]
impl crate::Resettable for EXTICR1_SPEC {
    const RESET_VALUE: u16 = 0;
}
