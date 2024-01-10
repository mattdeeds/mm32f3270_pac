#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `OK` writer - SYNC event OK clear flag"]
pub type OK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` writer - SYNC warning clear flag"]
pub type WARN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRC` writer - Error clear flag"]
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESYNCC` writer - Expected SYNC clear flag"]
pub type ESYNCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - SYNC event OK clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ok(&mut self) -> OK_W<ICR_SPEC> {
        OK_W::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WARN_W<ICR_SPEC> {
        WARN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ERRC_W<ICR_SPEC> {
        ERRC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn esyncc(&mut self) -> ESYNCC_W<ICR_SPEC> {
        ESYNCC_W::new(self, 3)
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
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
