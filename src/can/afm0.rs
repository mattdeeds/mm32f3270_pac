#[doc = "Register `AFM0` reader"]
pub type R = crate::R<AFM0_SPEC>;
#[doc = "Register `AFM0` writer"]
pub type W = crate::W<AFM0_SPEC>;
#[doc = "Field `AFM_7_1` reader - Acceptance filter mode"]
pub type AFM_7_1_R = crate::FieldReader;
#[doc = "Field `AFM_7_1` writer - Acceptance filter mode"]
pub type AFM_7_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_7_1(&self) -> AFM_7_1_R {
        AFM_7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Acceptance filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn afm_7_1(&mut self) -> AFM_7_1_W<AFM0_SPEC> {
        AFM_7_1_W::new(self, 1)
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
#[doc = "Filter Mode register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFM0_SPEC;
impl crate::RegisterSpec for AFM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm0::R`](R) reader structure"]
impl crate::Readable for AFM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afm0::W`](W) writer structure"]
impl crate::Writable for AFM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM0 to value 0"]
impl crate::Resettable for AFM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
