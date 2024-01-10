#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `TX` reader - Transmit FIFO empty interrupt enable bit"]
pub type TX_R = crate::BitReader;
#[doc = "Field `TX` writer - Transmit FIFO empty interrupt enable bit"]
pub type TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX` reader - Receive FIFO interrupt enable bit"]
pub type RX_R = crate::BitReader;
#[doc = "Field `RX` writer - Receive FIFO interrupt enable bit"]
pub type RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN` reader - Transmitter underrun interrupt enable bit(SPI slave mode only)"]
pub type UNDERRUN_R = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - Transmitter underrun interrupt enable bit(SPI slave mode only)"]
pub type UNDERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERR` reader - Overrun error interrupt enable bit"]
pub type RXOERR_R = crate::BitReader;
#[doc = "Field `RXOERR` writer - Overrun error interrupt enable bit"]
pub type RXOERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMATCH` reader - Receive data complete interrupt enable bit"]
pub type RXMATCH_R = crate::BitReader;
#[doc = "Field `RXMATCH` writer - Receive data complete interrupt enable bit"]
pub type RXMATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - Receive FIFO full interrupt enable bit"]
pub type RXFULL_R = crate::BitReader;
#[doc = "Field `RXFULL` writer - Receive FIFO full interrupt enable bit"]
pub type RXFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEPT` reader - Transmit empty interrupt enable bit"]
pub type TXEPT_R = crate::BitReader;
#[doc = "Field `TXEPT` writer - Transmit empty interrupt enable bit"]
pub type TXEPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRE` reader - I2S frame transmission error interrupt enable bit"]
pub type FRE_R = crate::BitReader;
#[doc = "Field `FRE` writer - I2S frame transmission error interrupt enable bit"]
pub type FRE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt enable bit"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO interrupt enable bit"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter underrun interrupt enable bit(SPI slave mode only)"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error interrupt enable bit"]
    #[inline(always)]
    pub fn rxoerr(&self) -> RXOERR_R {
        RXOERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive data complete interrupt enable bit"]
    #[inline(always)]
    pub fn rxmatch(&self) -> RXMATCH_R {
        RXMATCH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO full interrupt enable bit"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit empty interrupt enable bit"]
    #[inline(always)]
    pub fn txept(&self) -> TXEPT_R {
        TXEPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2S frame transmission error interrupt enable bit"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<IER_SPEC> {
        TX_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<IER_SPEC> {
        RX_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter underrun interrupt enable bit(SPI slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn underrun(&mut self) -> UNDERRUN_W<IER_SPEC> {
        UNDERRUN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxoerr(&mut self) -> RXOERR_W<IER_SPEC> {
        RXOERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive data complete interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxmatch(&mut self) -> RXMATCH_W<IER_SPEC> {
        RXMATCH_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO full interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<IER_SPEC> {
        RXFULL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit empty interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn txept(&mut self) -> TXEPT_W<IER_SPEC> {
        TXEPT_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2S frame transmission error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fre(&mut self) -> FRE_W<IER_SPEC> {
        FRE_W::new(self, 7)
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
