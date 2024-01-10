#[doc = "Register `TXERR_P` reader"]
pub type R = crate::R<TXERR_P_SPEC>;
#[doc = "Register `TXERR_P` writer"]
pub type W = crate::W<TXERR_P_SPEC>;
#[doc = "Field `TXERR` reader - TX error counter register"]
pub type TXERR_R = crate::FieldReader;
#[doc = "Field `TXERR` writer - TX error counter register"]
pub type TXERR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TX error counter register"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX error counter register"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<TXERR_P_SPEC> {
        TXERR_W::new(self, 0)
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
#[doc = "Peli TX Error Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txerr_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txerr_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXERR_P_SPEC;
impl crate::RegisterSpec for TXERR_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txerr_p::R`](R) reader structure"]
impl crate::Readable for TXERR_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txerr_p::W`](W) writer structure"]
impl crate::Writable for TXERR_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXERR_P to value 0"]
impl crate::Resettable for TXERR_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
