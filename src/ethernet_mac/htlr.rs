#[doc = "Register `HTLR` reader"]
pub type R = crate::R<HTLR_SPEC>;
#[doc = "Register `HTLR` writer"]
pub type W = crate::W<HTLR_SPEC>;
#[doc = "Field `HTABL` reader - Hash Table Low"]
pub type HTABL_R = crate::FieldReader<u32>;
#[doc = "Field `HTABL` writer - Hash Table Low"]
pub type HTABL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn htabl(&self) -> HTABL_R {
        HTABL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    #[must_use]
    pub fn htabl(&mut self) -> HTABL_W<HTLR_SPEC> {
        HTABL_W::new(self, 0)
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
#[doc = "Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTLR_SPEC;
impl crate::RegisterSpec for HTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htlr::R`](R) reader structure"]
impl crate::Readable for HTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`htlr::W`](W) writer structure"]
impl crate::Writable for HTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTLR to value 0"]
impl crate::Resettable for HTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
