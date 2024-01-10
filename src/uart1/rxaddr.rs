#[doc = "Register `RXADDR` reader"]
pub type R = crate::R<RXADDR_SPEC>;
#[doc = "Register `RXADDR` writer"]
pub type W = crate::W<RXADDR_SPEC>;
#[doc = "Field `RXADDR` reader - Synchronous frame match address"]
pub type RXADDR_R = crate::FieldReader;
#[doc = "Field `RXADDR` writer - Synchronous frame match address"]
pub type RXADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Synchronous frame match address"]
    #[inline(always)]
    pub fn rxaddr(&self) -> RXADDR_R {
        RXADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous frame match address"]
    #[inline(always)]
    #[must_use]
    pub fn rxaddr(&mut self) -> RXADDR_W<RXADDR_SPEC> {
        RXADDR_W::new(self, 0)
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
#[doc = "Receive Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXADDR_SPEC;
impl crate::RegisterSpec for RXADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxaddr::R`](R) reader structure"]
impl crate::Readable for RXADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxaddr::W`](W) writer structure"]
impl crate::Writable for RXADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXADDR to value 0"]
impl crate::Resettable for RXADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
