#[doc = "Register `DMATXPDR` reader"]
pub type R = crate::R<DMATXPDR_SPEC>;
#[doc = "Register `DMATXPDR` writer"]
pub type W = crate::W<DMATXPDR_SPEC>;
#[doc = "Field `TXPD` reader - Transmit poll demand"]
pub type TXPD_R = crate::FieldReader<u32>;
#[doc = "Field `TXPD` writer - Transmit poll demand"]
pub type TXPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn txpd(&self) -> TXPD_R {
        TXPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn txpd(&mut self) -> TXPD_W<DMATXPDR_SPEC> {
        TXPD_W::new(self, 0)
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
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatxpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATXPDR_SPEC;
impl crate::RegisterSpec for DMATXPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxpdr::R`](R) reader structure"]
impl crate::Readable for DMATXPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatxpdr::W`](W) writer structure"]
impl crate::Writable for DMATXPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATXPDR to value 0"]
impl crate::Resettable for DMATXPDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
