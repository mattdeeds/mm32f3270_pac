#[doc = "Register `IER_P` reader"]
pub type R = crate::R<IER_P_SPEC>;
#[doc = "Register `IER_P` writer"]
pub type W = crate::W<IER_P_SPEC>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOIE` reader - Data overrun interrupt enable"]
pub type DOIE_R = crate::BitReader;
#[doc = "Field `DOIE` writer - Data overrun interrupt enable"]
pub type DOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPIE` reader - Error passive interrupt enable"]
pub type EPIE_R = crate::BitReader;
#[doc = "Field `EPIE` writer - Error passive interrupt enable"]
pub type EPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIE` reader - Arbitration lost interrupt enable"]
pub type ALIE_R = crate::BitReader;
#[doc = "Field `ALIE` writer - Arbitration lost interrupt enable"]
pub type ALIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIE` reader - Bus error interrupt enable"]
pub type BEIE_R = crate::BitReader;
#[doc = "Field `BEIE` writer - Bus error interrupt enable"]
pub type BEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data overrun interrupt enable"]
    #[inline(always)]
    pub fn doie(&self) -> DOIE_R {
        DOIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus error interrupt enable"]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<IER_P_SPEC> {
        RIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<IER_P_SPEC> {
        TIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<IER_P_SPEC> {
        EIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn doie(&mut self) -> DOIE_W<IER_P_SPEC> {
        DOIE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Error passive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epie(&mut self) -> EPIE_W<IER_P_SPEC> {
        EPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> ALIE_W<IER_P_SPEC> {
        ALIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn beie(&mut self) -> BEIE_W<IER_P_SPEC> {
        BEIE_W::new(self, 7)
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
#[doc = "Peli Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_P_SPEC;
impl crate::RegisterSpec for IER_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier_p::R`](R) reader structure"]
impl crate::Readable for IER_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier_p::W`](W) writer structure"]
impl crate::Writable for IER_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER_P to value 0"]
impl crate::Resettable for IER_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
