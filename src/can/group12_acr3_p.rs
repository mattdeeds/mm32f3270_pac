#[doc = "Register `GROUP12_ACR3_P` reader"]
pub type R = crate::R<GROUP12_ACR3_P_SPEC>;
#[doc = "Register `GROUP12_ACR3_P` writer"]
pub type W = crate::W<GROUP12_ACR3_P_SPEC>;
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
    pub fn ac(&mut self) -> AC_W<GROUP12_ACR3_P_SPEC> {
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
#[doc = "Peli Acceptance Code register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`group12_acr3_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`group12_acr3_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GROUP12_ACR3_P_SPEC;
impl crate::RegisterSpec for GROUP12_ACR3_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group12_acr3_p::R`](R) reader structure"]
impl crate::Readable for GROUP12_ACR3_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`group12_acr3_p::W`](W) writer structure"]
impl crate::Writable for GROUP12_ACR3_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GROUP12_ACR3_P to value 0"]
impl crate::Resettable for GROUP12_ACR3_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
