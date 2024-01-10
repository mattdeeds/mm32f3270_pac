#[doc = "Register `DMARXPDR` reader"]
pub type R = crate::R<DMARXPDR_SPEC>;
#[doc = "Register `DMARXPDR` writer"]
pub type W = crate::W<DMARXPDR_SPEC>;
#[doc = "Field `RXPD` reader - Receive poll demand"]
pub type RXPD_R = crate::FieldReader<u32>;
#[doc = "Field `RXPD` writer - Receive poll demand"]
pub type RXPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rxpd(&self) -> RXPD_R {
        RXPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn rxpd(&mut self) -> RXPD_W<DMARXPDR_SPEC> {
        RXPD_W::new(self, 0)
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
#[doc = "EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarxpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARXPDR_SPEC;
impl crate::RegisterSpec for DMARXPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxpdr::R`](R) reader structure"]
impl crate::Readable for DMARXPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmarxpdr::W`](W) writer structure"]
impl crate::Writable for DMARXPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARXPDR to value 0"]
impl crate::Resettable for DMARXPDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
