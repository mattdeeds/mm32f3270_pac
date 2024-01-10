#[doc = "Register `DMATXDSAR` reader"]
pub type R = crate::R<DMATXDSAR_SPEC>;
#[doc = "Register `DMATXDSAR` writer"]
pub type W = crate::W<DMATXDSAR_SPEC>;
#[doc = "Field `TXDSA` reader - Start of receive list"]
pub type TXDSA_R = crate::FieldReader<u32>;
#[doc = "Field `TXDSA` writer - Start of receive list"]
pub type TXDSA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    pub fn txdsa(&self) -> TXDSA_R {
        TXDSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    #[must_use]
    pub fn txdsa(&mut self) -> TXDSA_W<DMATXDSAR_SPEC> {
        TXDSA_W::new(self, 0)
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
#[doc = "Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxdsar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatxdsar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATXDSAR_SPEC;
impl crate::RegisterSpec for DMATXDSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxdsar::R`](R) reader structure"]
impl crate::Readable for DMATXDSAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatxdsar::W`](W) writer structure"]
impl crate::Writable for DMATXDSAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATXDSAR to value 0"]
impl crate::Resettable for DMATXDSAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
