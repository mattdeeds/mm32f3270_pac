#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<AHB1ENR_SPEC>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<AHB1ENR_SPEC>;
#[doc = "Field `GPIOA` reader - FSMC clock enable"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - FSMC clock enable"]
pub type GPIOA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - GPIOb clock enable"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - GPIOb clock enable"]
pub type GPIOB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - GPIOC clock enable"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - GPIOC clock enable"]
pub type GPIOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - GPIOD clock enable"]
pub type GPIOD_R = crate::BitReader;
#[doc = "Field `GPIOD` writer - GPIOD clock enable"]
pub type GPIOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOE` reader - GPIOE clock enable"]
pub type GPIOE_R = crate::BitReader;
#[doc = "Field `GPIOE` writer - GPIOE clock enable"]
pub type GPIOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOF` reader - GPIOF clock enable"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - GPIOF clock enable"]
pub type GPIOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOG` reader - GPIOG clock enable"]
pub type GPIOG_R = crate::BitReader;
#[doc = "Field `GPIOG` writer - GPIOG clock enable"]
pub type GPIOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOH` reader - USBOTGFS clock enable"]
pub type GPIOH_R = crate::BitReader;
#[doc = "Field `GPIOH` writer - USBOTGFS clock enable"]
pub type GPIOH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO` reader - SDIO clock enable"]
pub type SDIO_R = crate::BitReader;
#[doc = "Field `SDIO` writer - SDIO clock enable"]
pub type SDIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - *12"]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - *12"]
pub type CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH` reader - *13"]
pub type FLASH_R = crate::BitReader;
#[doc = "Field `FLASH` writer - *13"]
pub type FLASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM` reader - *14"]
pub type SRAM_R = crate::BitReader;
#[doc = "Field `SRAM` writer - *14"]
pub type SRAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1` reader - DMA1 clock enable"]
pub type DMA1_R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 clock enable"]
pub type DMA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2` reader - DMA2 clock enable"]
pub type DMA2_R = crate::BitReader;
#[doc = "Field `DMA2` writer - DMA2 clock enable"]
pub type DMA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHMAC` reader - ETHMAC clock enable"]
pub type ETHMAC_R = crate::BitReader;
#[doc = "Field `ETHMAC` writer - ETHMAC clock enable"]
pub type ETHMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FSMC clock enable"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOb clock enable"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC clock enable"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD clock enable"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE clock enable"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF clock enable"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOG clock enable"]
    #[inline(always)]
    pub fn gpiog(&self) -> GPIOG_R {
        GPIOG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USBOTGFS clock enable"]
    #[inline(always)]
    pub fn gpioh(&self) -> GPIOH_R {
        GPIOH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - *12"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - *13"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - *14"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2(&self) -> DMA2_R {
        DMA2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - ETHMAC clock enable"]
    #[inline(always)]
    pub fn ethmac(&self) -> ETHMAC_R {
        ETHMAC_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FSMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHB1ENR_SPEC> {
        GPIOA_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOb clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHB1ENR_SPEC> {
        GPIOB_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHB1ENR_SPEC> {
        GPIOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<AHB1ENR_SPEC> {
        GPIOD_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOE clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioe(&mut self) -> GPIOE_W<AHB1ENR_SPEC> {
        GPIOE_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHB1ENR_SPEC> {
        GPIOF_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiog(&mut self) -> GPIOG_W<AHB1ENR_SPEC> {
        GPIOG_W::new(self, 6)
    }
    #[doc = "Bit 7 - USBOTGFS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioh(&mut self) -> GPIOH_W<AHB1ENR_SPEC> {
        GPIOH_W::new(self, 7)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SDIO_W<AHB1ENR_SPEC> {
        SDIO_W::new(self, 10)
    }
    #[doc = "Bit 12 - *12"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<AHB1ENR_SPEC> {
        CRC_W::new(self, 12)
    }
    #[doc = "Bit 13 - *13"]
    #[inline(always)]
    #[must_use]
    pub fn flash(&mut self) -> FLASH_W<AHB1ENR_SPEC> {
        FLASH_W::new(self, 13)
    }
    #[doc = "Bit 14 - *14"]
    #[inline(always)]
    #[must_use]
    pub fn sram(&mut self) -> SRAM_W<AHB1ENR_SPEC> {
        SRAM_W::new(self, 14)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<AHB1ENR_SPEC> {
        DMA1_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2(&mut self) -> DMA2_W<AHB1ENR_SPEC> {
        DMA2_W::new(self, 22)
    }
    #[doc = "Bit 25 - ETHMAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethmac(&mut self) -> ETHMAC_W<AHB1ENR_SPEC> {
        ETHMAC_W::new(self, 25)
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
#[doc = "Advanced High Performance Bus 1 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for AHB1ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for AHB1ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x6000"]
impl crate::Resettable for AHB1ENR_SPEC {
    const RESET_VALUE: u32 = 0x6000;
}
