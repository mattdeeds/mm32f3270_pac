#[doc = "Register `CSHR` reader"]
pub type R = crate::R<CSHR_SPEC>;
#[doc = "Register `CSHR` writer"]
pub type W = crate::W<CSHR_SPEC>;
#[doc = "Field `CSHR` reader - hit statistics"]
pub type CSHR_R = crate::FieldReader<u32>;
#[doc = "Field `CSHR` writer - hit statistics"]
pub type CSHR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hit statistics"]
    #[inline(always)]
    pub fn cshr(&self) -> CSHR_R {
        CSHR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - hit statistics"]
    #[inline(always)]
    #[must_use]
    pub fn cshr(&mut self) -> CSHR_W<CSHR_SPEC> {
        CSHR_W::new(self, 0)
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
#[doc = "Cache statistics hit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cshr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cshr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSHR_SPEC;
impl crate::RegisterSpec for CSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cshr::R`](R) reader structure"]
impl crate::Readable for CSHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cshr::W`](W) writer structure"]
impl crate::Writable for CSHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSHR to value 0"]
impl crate::Resettable for CSHR_SPEC {
    const RESET_VALUE: u32 = 0;
}
