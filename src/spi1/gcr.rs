#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCR_SPEC>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCR_SPEC>;
#[doc = "Field `SPIEN` reader - SPI select bit"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI select bit"]
pub type SPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEN` reader - SPI interrupt enable bit"]
pub type IEN_R = crate::BitReader;
#[doc = "Field `IEN` writer - SPI interrupt enable bit"]
pub type IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Master mode bit"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Master mode bit"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Transmit enable bit"]
pub type TXEN_R = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmit enable bit"]
pub type TXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receive enable bit"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXEN` writer - Receive enable bit"]
pub type RXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTLF` reader - RX FIFO trigger level bit"]
pub type RXTLF_R = crate::FieldReader;
#[doc = "Field `RXTLF` writer - RX FIFO trigger level bit"]
pub type RXTLF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXTLF` reader - TX FIFO trigger level bit"]
pub type TXTLF_R = crate::FieldReader;
#[doc = "Field `TXTLF` writer - TX FIFO trigger level bit"]
pub type TXTLF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMAEN` reader - DMA access mode enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA access mode enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSS` reader - NSS select signal that from software and hardware"]
pub type NSS_R = crate::BitReader;
#[doc = "Field `NSS` writer - NSS select signal that from software and hardware"]
pub type NSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWSEL` reader - Valid byte or double-word data select signal"]
pub type DWSEL_R = crate::BitReader;
#[doc = "Field `DWSEL` writer - Valid byte or double-word data select signal"]
pub type DWSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSTOG` reader - NSS selection signal is automatically flipped"]
pub type NSSTOG_R = crate::BitReader;
#[doc = "Field `NSSTOG` writer - NSS selection signal is automatically flipped"]
pub type NSSTOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_SEL` reader - Bus mapping transformation."]
pub type PAD_SEL_R = crate::FieldReader;
#[doc = "Field `PAD_SEL` writer - Bus mapping transformation."]
pub type PAD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - SPI select bit"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI interrupt enable bit"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master mode bit"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit enable bit"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive enable bit"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - RX FIFO trigger level bit"]
    #[inline(always)]
    pub fn rxtlf(&self) -> RXTLF_R {
        RXTLF_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - TX FIFO trigger level bit"]
    #[inline(always)]
    pub fn txtlf(&self) -> TXTLF_R {
        TXTLF_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - DMA access mode enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NSS select signal that from software and hardware"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Valid byte or double-word data select signal"]
    #[inline(always)]
    pub fn dwsel(&self) -> DWSEL_R {
        DWSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NSS selection signal is automatically flipped"]
    #[inline(always)]
    pub fn nsstog(&self) -> NSSTOG_R {
        NSSTOG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - Bus mapping transformation."]
    #[inline(always)]
    pub fn pad_sel(&self) -> PAD_SEL_R {
        PAD_SEL_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI select bit"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<GCR_SPEC> {
        SPIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<GCR_SPEC> {
        IEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<GCR_SPEC> {
        MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<GCR_SPEC> {
        TXEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<GCR_SPEC> {
        RXEN_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - RX FIFO trigger level bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxtlf(&mut self) -> RXTLF_W<GCR_SPEC> {
        RXTLF_W::new(self, 5)
    }
    #[doc = "Bits 7:8 - TX FIFO trigger level bit"]
    #[inline(always)]
    #[must_use]
    pub fn txtlf(&mut self) -> TXTLF_W<GCR_SPEC> {
        TXTLF_W::new(self, 7)
    }
    #[doc = "Bit 9 - DMA access mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<GCR_SPEC> {
        DMAEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - NSS select signal that from software and hardware"]
    #[inline(always)]
    #[must_use]
    pub fn nss(&mut self) -> NSS_W<GCR_SPEC> {
        NSS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Valid byte or double-word data select signal"]
    #[inline(always)]
    #[must_use]
    pub fn dwsel(&mut self) -> DWSEL_W<GCR_SPEC> {
        DWSEL_W::new(self, 11)
    }
    #[doc = "Bit 12 - NSS selection signal is automatically flipped"]
    #[inline(always)]
    #[must_use]
    pub fn nsstog(&mut self) -> NSSTOG_W<GCR_SPEC> {
        NSSTOG_W::new(self, 12)
    }
    #[doc = "Bits 13:17 - Bus mapping transformation."]
    #[inline(always)]
    #[must_use]
    pub fn pad_sel(&mut self) -> PAD_SEL_W<GCR_SPEC> {
        PAD_SEL_W::new(self, 13)
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
#[doc = "Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0x04"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
