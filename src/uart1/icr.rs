#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `TXICLR` reader - Transmit buffer empty interrupt clear bit"]
pub type TXICLR_R = crate::BitReader;
#[doc = "Field `TXICLR` writer - Transmit buffer empty interrupt clear bit"]
pub type TXICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICLR` reader - Receive interrupt clear bit"]
pub type RXICLR_R = crate::BitReader;
#[doc = "Field `RXICLR` writer - Receive interrupt clear bit"]
pub type RXICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC_ICLR` reader - Transmit complete interrupt clear bit"]
pub type TXC_ICLR_R = crate::BitReader;
#[doc = "Field `TXC_ICLR` writer - Transmit complete interrupt clear bit"]
pub type TXC_ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERR_ICLR` reader - Receive overflow error interrupt clear bit"]
pub type RXOERR_ICLR_R = crate::BitReader;
#[doc = "Field `RXOERR_ICLR` writer - Receive overflow error interrupt clear bit"]
pub type RXOERR_ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPERR_ICLR` reader - Parity error interrupt clear bit"]
pub type RXPERR_ICLR_R = crate::BitReader;
#[doc = "Field `RXPERR_ICLR` writer - Parity error interrupt clear bit"]
pub type RXPERR_ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFERR_ICLR` reader - Frame error interrupt clear bit"]
pub type RXFERR_ICLR_R = crate::BitReader;
#[doc = "Field `RXFERR_ICLR` writer - Frame error interrupt clear bit"]
pub type RXFERR_ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK_ICLR` reader - Receive frame break interrupt clear bit"]
pub type RXBRK_ICLR_R = crate::BitReader;
#[doc = "Field `RXBRK_ICLR` writer - Receive frame break interrupt clear bit"]
pub type RXBRK_ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBRK_ICLR` reader - Transmit Break Frame Interrupt clear Bit"]
pub type TXBRK_ICLR_R = crate::BitReader;
#[doc = "Field `TXBRK_ICLR` writer - Transmit Break Frame Interrupt clear Bit"]
pub type TXBRK_ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB8_ICLR` reader - Receive Bit 8 Interrupt clear Bit"]
pub type RXB8_ICLR_R = crate::BitReader;
#[doc = "Field `RXB8_ICLR` writer - Receive Bit 8 Interrupt clear Bit"]
pub type RXB8_ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIDLE_ICLR` reader - Receive Bit 8 Interrupt clear Bit"]
pub type RXIDLE_ICLR_R = crate::BitReader;
#[doc = "Field `RXIDLE_ICLR` writer - Receive Bit 8 Interrupt clear Bit"]
pub type RXIDLE_ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREND__ICLR` reader - Auto baud rate end interrupt clear bit"]
pub type ABREND__ICLR_R = crate::BitReader;
#[doc = "Field `ABREND__ICLR` writer - Auto baud rate end interrupt clear bit"]
pub type ABREND__ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRERR__ICLR` reader - Auto baud rate error interrupt clear bit"]
pub type ABRERR__ICLR_R = crate::BitReader;
#[doc = "Field `ABRERR__ICLR` writer - Auto baud rate error interrupt clear bit"]
pub type ABRERR__ICLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit buffer empty interrupt clear bit"]
    #[inline(always)]
    pub fn txiclr(&self) -> TXICLR_R {
        TXICLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    pub fn rxiclr(&self) -> RXICLR_R {
        RXICLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit complete interrupt clear bit"]
    #[inline(always)]
    pub fn txc_iclr(&self) -> TXC_ICLR_R {
        TXC_ICLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt clear bit"]
    #[inline(always)]
    pub fn rxoerr_iclr(&self) -> RXOERR_ICLR_R {
        RXOERR_ICLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error interrupt clear bit"]
    #[inline(always)]
    pub fn rxperr_iclr(&self) -> RXPERR_ICLR_R {
        RXPERR_ICLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame error interrupt clear bit"]
    #[inline(always)]
    pub fn rxferr_iclr(&self) -> RXFERR_ICLR_R {
        RXFERR_ICLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive frame break interrupt clear bit"]
    #[inline(always)]
    pub fn rxbrk_iclr(&self) -> RXBRK_ICLR_R {
        RXBRK_ICLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt clear Bit"]
    #[inline(always)]
    pub fn txbrk_iclr(&self) -> TXBRK_ICLR_R {
        TXBRK_ICLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    pub fn rxb8_iclr(&self) -> RXB8_ICLR_R {
        RXB8_ICLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    pub fn rxidle_iclr(&self) -> RXIDLE_ICLR_R {
        RXIDLE_ICLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Auto baud rate end interrupt clear bit"]
    #[inline(always)]
    pub fn abrend__iclr(&self) -> ABREND__ICLR_R {
        ABREND__ICLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Auto baud rate error interrupt clear bit"]
    #[inline(always)]
    pub fn abrerr__iclr(&self) -> ABRERR__ICLR_R {
        ABRERR__ICLR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit buffer empty interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txiclr(&mut self) -> TXICLR_W<ICR_SPEC> {
        TXICLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxiclr(&mut self) -> RXICLR_W<ICR_SPEC> {
        RXICLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit complete interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txc_iclr(&mut self) -> TXC_ICLR_W<ICR_SPEC> {
        TXC_ICLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxoerr_iclr(&mut self) -> RXOERR_ICLR_W<ICR_SPEC> {
        RXOERR_ICLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxperr_iclr(&mut self) -> RXPERR_ICLR_W<ICR_SPEC> {
        RXPERR_ICLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Frame error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxferr_iclr(&mut self) -> RXFERR_ICLR_W<ICR_SPEC> {
        RXFERR_ICLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive frame break interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk_iclr(&mut self) -> RXBRK_ICLR_W<ICR_SPEC> {
        RXBRK_ICLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn txbrk_iclr(&mut self) -> TXBRK_ICLR_W<ICR_SPEC> {
        TXBRK_ICLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxb8_iclr(&mut self) -> RXB8_ICLR_W<ICR_SPEC> {
        RXB8_ICLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxidle_iclr(&mut self) -> RXIDLE_ICLR_W<ICR_SPEC> {
        RXIDLE_ICLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Auto baud rate end interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn abrend__iclr(&mut self) -> ABREND__ICLR_W<ICR_SPEC> {
        ABREND__ICLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Auto baud rate error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn abrerr__iclr(&mut self) -> ABRERR__ICLR_W<ICR_SPEC> {
        ABRERR__ICLR_W::new(self, 11)
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
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
