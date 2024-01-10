#[doc = "Register `AFM1` reader"]
pub type R = crate::R<AFM1_SPEC>;
#[doc = "Register `AFM1` writer"]
pub type W = crate::W<AFM1_SPEC>;
#[doc = "Field `AFM_15_8` reader - Acceptance filter mode"]
pub type AFM_15_8_R = crate::FieldReader;
#[doc = "Field `AFM_15_8` writer - Acceptance filter mode"]
pub type AFM_15_8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_15_8(&self) -> AFM_15_8_R {
        AFM_15_8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn afm_15_8(&mut self) -> AFM_15_8_W<AFM1_SPEC> {
        AFM_15_8_W::new(self, 0)
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
#[doc = "Filter Mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFM1_SPEC;
impl crate::RegisterSpec for AFM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm1::R`](R) reader structure"]
impl crate::Readable for AFM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afm1::W`](W) writer structure"]
impl crate::Writable for AFM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM1 to value 0"]
impl crate::Resettable for AFM1_SPEC {
    const RESET_VALUE: u32 = 0;
}
