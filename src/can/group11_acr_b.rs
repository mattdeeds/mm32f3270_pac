#[doc = "Register `GROUP11_ACR_B` reader"]
pub type R = crate::R<GROUP11_ACR_B_SPEC>;
#[doc = "Register `GROUP11_ACR_B` writer"]
pub type W = crate::W<GROUP11_ACR_B_SPEC>;
#[doc = "Field `AC` reader - Acceptance code"]
pub type AC_R = crate::FieldReader;
#[doc = "Field `AC` writer - Acceptance code"]
pub type AC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acceptance code"]
    #[inline(always)]
    pub fn ac(&self) -> AC_R {
        AC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance code"]
    #[inline(always)]
    #[must_use]
    pub fn ac(&mut self) -> AC_W<GROUP11_ACR_B_SPEC> {
        AC_W::new(self, 0)
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
#[doc = "Basic Acceptance Code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`group11_acr_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`group11_acr_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GROUP11_ACR_B_SPEC;
impl crate::RegisterSpec for GROUP11_ACR_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group11_acr_b::R`](R) reader structure"]
impl crate::Readable for GROUP11_ACR_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`group11_acr_b::W`](W) writer structure"]
impl crate::Writable for GROUP11_ACR_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GROUP11_ACR_B to value 0"]
impl crate::Resettable for GROUP11_ACR_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
