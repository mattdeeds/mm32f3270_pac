#[doc = "Register `TXFLR` reader"]
pub type R = crate::R<TXFLR_SPEC>;
#[doc = "Register `TXFLR` writer"]
pub type W = crate::W<TXFLR_SPEC>;
#[doc = "Field `CNT` reader - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<TXFLR_SPEC> {
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
#[doc = "Transmit FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFLR_SPEC;
impl crate::RegisterSpec for TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TXFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txflr::W`](W) writer structure"]
impl crate::Writable for TXFLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TXFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
