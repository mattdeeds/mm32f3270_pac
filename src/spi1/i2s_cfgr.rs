#[doc = "Register `I2S_CFGR` reader"]
pub type R = crate::R<I2S_CFGR_SPEC>;
#[doc = "Register `I2S_CFGR` writer"]
pub type W = crate::W<I2S_CFGR_SPEC>;
#[doc = "Field `CHLEN` reader - Channel length"]
pub type CHLEN_R = crate::BitReader;
#[doc = "Field `CHLEN` writer - Channel length"]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATLEN` reader - Audio data width"]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Audio data width"]
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2SSTD_R = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization mode"]
pub type PCMSYNC_R = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization mode"]
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_I2S` reader - SPI/I2S module function selection"]
pub type SPI_I2S_R = crate::BitReader;
#[doc = "Field `SPI_I2S` writer - SPI/I2S module function selection"]
pub type SPI_I2S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOE` reader - I2S master clock output enable"]
pub type MCKOE_R = crate::BitReader;
#[doc = "Field `MCKOE` writer - I2S master clock output enable"]
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SDIV` reader - Frequency division coefficient of I2S prescaler, configuration is valid in main mode"]
pub type I2SDIV_R = crate::FieldReader<u16>;
#[doc = "Field `I2SDIV` writer - Frequency division coefficient of I2S prescaler, configuration is valid in main mode"]
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - Channel length"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Audio data width"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - PCM frame synchronization mode"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI/I2S module function selection"]
    #[inline(always)]
    pub fn spi_i2s(&self) -> SPI_I2S_R {
        SPI_I2S_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Frequency division coefficient of I2S prescaler, configuration is valid in main mode"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<I2S_CFGR_SPEC> {
        CHLEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Audio data width"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<I2S_CFGR_SPEC> {
        DATLEN_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<I2S_CFGR_SPEC> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 6 - PCM frame synchronization mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<I2S_CFGR_SPEC> {
        PCMSYNC_W::new(self, 6)
    }
    #[doc = "Bit 10 - SPI/I2S module function selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi_i2s(&mut self) -> SPI_I2S_W<I2S_CFGR_SPEC> {
        SPI_I2S_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2S master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<I2S_CFGR_SPEC> {
        MCKOE_W::new(self, 11)
    }
    #[doc = "Bits 16:24 - Frequency division coefficient of I2S prescaler, configuration is valid in main mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<I2S_CFGR_SPEC> {
        I2SDIV_W::new(self, 16)
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
#[doc = "I2S Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_CFGR_SPEC;
impl crate::RegisterSpec for I2S_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_cfgr::R`](R) reader structure"]
impl crate::Readable for I2S_CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_cfgr::W`](W) writer structure"]
impl crate::Writable for I2S_CFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S_CFGR to value 0x01"]
impl crate::Resettable for I2S_CFGR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
