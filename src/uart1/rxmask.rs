#[doc = "Register `RXMASK` reader"]
pub type R = crate::R<RXMASK_SPEC>;
#[doc = "Register `RXMASK` writer"]
pub type W = crate::W<RXMASK_SPEC>;
#[doc = "Field `RXMASK` reader - Synchronous frame match address mask"]
pub type RXMASK_R = crate::FieldReader;
#[doc = "Field `RXMASK` writer - Synchronous frame match address mask"]
pub type RXMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Synchronous frame match address mask"]
    #[inline(always)]
    pub fn rxmask(&self) -> RXMASK_R {
        RXMASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous frame match address mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxmask(&mut self) -> RXMASK_W<RXMASK_SPEC> {
        RXMASK_W::new(self, 0)
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
#[doc = "Receive Mask Registe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMASK_SPEC;
impl crate::RegisterSpec for RXMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmask::R`](R) reader structure"]
impl crate::Readable for RXMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxmask::W`](W) writer structure"]
impl crate::Writable for RXMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXMASK to value 0xff"]
impl crate::Resettable for RXMASK_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
