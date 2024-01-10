#[doc = "Register `RXFLR` reader"]
pub type R = crate::R<RXFLR_SPEC>;
#[doc = "Register `RXFLR` writer"]
pub type W = crate::W<RXFLR_SPEC>;
#[doc = "Field `CNT` reader - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<RXFLR_SPEC> {
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
#[doc = "Receive FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFLR_SPEC;
impl crate::RegisterSpec for RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RXFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxflr::W`](W) writer structure"]
impl crate::Writable for RXFLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RXFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
