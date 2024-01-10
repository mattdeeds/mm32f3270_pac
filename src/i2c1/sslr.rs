#[doc = "Register `SSLR` reader"]
pub type R = crate::R<SSLR_SPEC>;
#[doc = "Register `SSLR` writer"]
pub type W = crate::W<SSLR_SPEC>;
#[doc = "Field `CNT` reader - This register sets the SCL clock low period count for standard speed"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock low period count for standard speed"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<SSLR_SPEC> {
        CNT_W::new(self, 0)
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
#[doc = "SCL Low Period Count for Std. Speed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sslr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sslr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSLR_SPEC;
impl crate::RegisterSpec for SSLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sslr::R`](R) reader structure"]
impl crate::Readable for SSLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sslr::W`](W) writer structure"]
impl crate::Writable for SSLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSLR to value 0x01d6"]
impl crate::Resettable for SSLR_SPEC {
    const RESET_VALUE: u32 = 0x01d6;
}
