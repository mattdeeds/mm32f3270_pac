#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `TX_IEN` reader - Transmit buffer empty interrupt enable bit"]
pub type TX_IEN_R = crate::BitReader;
#[doc = "Field `TX_IEN` writer - Transmit buffer empty interrupt enable bit"]
pub type TX_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IEN` reader - Receive buffer interrupt enable bit"]
pub type RX_IEN_R = crate::BitReader;
#[doc = "Field `RX_IEN` writer - Receive buffer interrupt enable bit"]
pub type RX_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC_IEN` reader - Transmit complete interrupt enable bit"]
pub type TXC_IEN_R = crate::BitReader;
#[doc = "Field `TXC_IEN` writer - Transmit complete interrupt enable bit"]
pub type TXC_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERR_IEN` reader - Receive overflow error interrupt enable bit"]
pub type RXOERR_IEN_R = crate::BitReader;
#[doc = "Field `RXOERR_IEN` writer - Receive overflow error interrupt enable bit"]
pub type RXOERR_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPERR_IEN` reader - Parity error interrupt enable bit"]
pub type RXPERR_IEN_R = crate::BitReader;
#[doc = "Field `RXPERR_IEN` writer - Parity error interrupt enable bit"]
pub type RXPERR_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFERR_IEN` reader - Frame error interrupt enable bit"]
pub type RXFERR_IEN_R = crate::BitReader;
#[doc = "Field `RXFERR_IEN` writer - Frame error interrupt enable bit"]
pub type RXFERR_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK_IEN` reader - Receive frame break interrupt enable bit"]
pub type RXBRK_IEN_R = crate::BitReader;
#[doc = "Field `RXBRK_IEN` writer - Receive frame break interrupt enable bit"]
pub type RXBRK_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBRK_IEN` reader - Transmit Break Frame Interrupt Enable Bit"]
pub type TXBRK_IEN_R = crate::BitReader;
#[doc = "Field `TXBRK_IEN` writer - Transmit Break Frame Interrupt Enable Bit"]
pub type TXBRK_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB8_IEN` reader - Receive Bit 8 Interrupt Enable Bit"]
pub type RXB8_IEN_R = crate::BitReader;
#[doc = "Field `RXB8_IEN` writer - Receive Bit 8 Interrupt Enable Bit"]
pub type RXB8_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIDLE_IEN` reader - Receive frame idle interrupt enable bit"]
pub type RXIDLE_IEN_R = crate::BitReader;
#[doc = "Field `RXIDLE_IEN` writer - Receive frame idle interrupt enable bit"]
pub type RXIDLE_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREND_IEN` reader - Automatic baud rate end interrupt enable"]
pub type ABREND_IEN_R = crate::BitReader;
#[doc = "Field `ABREND_IEN` writer - Automatic baud rate end interrupt enable"]
pub type ABREND_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRERR_IEN` reader - Automatic baud rate error interrupt enable"]
pub type ABRERR_IEN_R = crate::BitReader;
#[doc = "Field `ABRERR_IEN` writer - Automatic baud rate error interrupt enable"]
pub type ABRERR_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit buffer empty interrupt enable bit"]
    #[inline(always)]
    pub fn tx_ien(&self) -> TX_IEN_R {
        TX_IEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive buffer interrupt enable bit"]
    #[inline(always)]
    pub fn rx_ien(&self) -> RX_IEN_R {
        RX_IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit complete interrupt enable bit"]
    #[inline(always)]
    pub fn txc_ien(&self) -> TXC_IEN_R {
        TXC_IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt enable bit"]
    #[inline(always)]
    pub fn rxoerr_ien(&self) -> RXOERR_IEN_R {
        RXOERR_IEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error interrupt enable bit"]
    #[inline(always)]
    pub fn rxperr_ien(&self) -> RXPERR_IEN_R {
        RXPERR_IEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame error interrupt enable bit"]
    #[inline(always)]
    pub fn rxferr_ien(&self) -> RXFERR_IEN_R {
        RXFERR_IEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive frame break interrupt enable bit"]
    #[inline(always)]
    pub fn rxbrk_ien(&self) -> RXBRK_IEN_R {
        RXBRK_IEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt Enable Bit"]
    #[inline(always)]
    pub fn txbrk_ien(&self) -> TXBRK_IEN_R {
        TXBRK_IEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxb8_ien(&self) -> RXB8_IEN_R {
        RXB8_IEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive frame idle interrupt enable bit"]
    #[inline(always)]
    pub fn rxidle_ien(&self) -> RXIDLE_IEN_R {
        RXIDLE_IEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic baud rate end interrupt enable"]
    #[inline(always)]
    pub fn abrend_ien(&self) -> ABREND_IEN_R {
        ABREND_IEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Automatic baud rate error interrupt enable"]
    #[inline(always)]
    pub fn abrerr_ien(&self) -> ABRERR_IEN_R {
        ABRERR_IEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit buffer empty interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ien(&mut self) -> TX_IEN_W<IER_SPEC> {
        TX_IEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive buffer interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ien(&mut self) -> RX_IEN_W<IER_SPEC> {
        RX_IEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit complete interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn txc_ien(&mut self) -> TXC_IEN_W<IER_SPEC> {
        TXC_IEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxoerr_ien(&mut self) -> RXOERR_IEN_W<IER_SPEC> {
        RXOERR_IEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxperr_ien(&mut self) -> RXPERR_IEN_W<IER_SPEC> {
        RXPERR_IEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Frame error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxferr_ien(&mut self) -> RXFERR_IEN_W<IER_SPEC> {
        RXFERR_IEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive frame break interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk_ien(&mut self) -> RXBRK_IEN_W<IER_SPEC> {
        RXBRK_IEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn txbrk_ien(&mut self) -> TXBRK_IEN_W<IER_SPEC> {
        TXBRK_IEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxb8_ien(&mut self) -> RXB8_IEN_W<IER_SPEC> {
        RXB8_IEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive frame idle interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxidle_ien(&mut self) -> RXIDLE_IEN_W<IER_SPEC> {
        RXIDLE_IEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Automatic baud rate end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn abrend_ien(&mut self) -> ABREND_IEN_W<IER_SPEC> {
        ABREND_IEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Automatic baud rate error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn abrerr_ien(&mut self) -> ABRERR_IEN_W<IER_SPEC> {
        ABRERR_IEN_W::new(self, 11)
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: u32 = 0;
}
