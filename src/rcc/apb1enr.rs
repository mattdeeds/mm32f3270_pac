#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<APB1ENR_SPEC>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<APB1ENR_SPEC>;
#[doc = "Field `TIM2` reader - TIM2 clock enable"]
pub type TIM2_R = crate::BitReader;
#[doc = "Field `TIM2` writer - TIM2 clock enable"]
pub type TIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3` reader - TIM3 clock enable"]
pub type TIM3_R = crate::BitReader;
#[doc = "Field `TIM3` writer - TIM3 clock enable"]
pub type TIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4` reader - TIM4 clock enable"]
pub type TIM4_R = crate::BitReader;
#[doc = "Field `TIM4` writer - TIM4 clock enable"]
pub type TIM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5` reader - TIM5 clock enable"]
pub type TIM5_R = crate::BitReader;
#[doc = "Field `TIM5` writer - TIM5 clock enable"]
pub type TIM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6` reader - TIM6 clock enable"]
pub type TIM6_R = crate::BitReader;
#[doc = "Field `TIM6` writer - TIM6 clock enable"]
pub type TIM6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7` reader - TIM7 clock enable"]
pub type TIM7_R = crate::BitReader;
#[doc = "Field `TIM7` writer - TIM7 clock enable"]
pub type TIM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG` reader - Window watchdog clock enable"]
pub type WWDG_R = crate::BitReader;
#[doc = "Field `WWDG` writer - Window watchdog clock enable"]
pub type WWDG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - SPI2 clock enable"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 clock enable"]
pub type SPI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - SPI3 clock enable"]
pub type SPI3_R = crate::BitReader;
#[doc = "Field `SPI3` writer - SPI3 clock enable"]
pub type SPI3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - UART2 clock enable"]
pub type UART2_R = crate::BitReader;
#[doc = "Field `UART2` writer - UART2 clock enable"]
pub type UART2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3` reader - UART3 clock enable"]
pub type UART3_R = crate::BitReader;
#[doc = "Field `UART3` writer - UART3 clock enable"]
pub type UART3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4` reader - UART4 clock enable"]
pub type UART4_R = crate::BitReader;
#[doc = "Field `UART4` writer - UART4 clock enable"]
pub type UART4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5` reader - UART5 clock enable"]
pub type UART5_R = crate::BitReader;
#[doc = "Field `UART5` writer - UART5 clock enable"]
pub type UART5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 clock enable"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 clock enable"]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C2 clock enable"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 clock enable"]
pub type I2C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRS` reader - CRS clock enable"]
pub type CRS_R = crate::BitReader;
#[doc = "Field `CRS` writer - CRS clock enable"]
pub type CRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN` reader - CAN clock enable"]
pub type CAN_R = crate::BitReader;
#[doc = "Field `CAN` writer - CAN clock enable"]
pub type CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Backup interface clock enable"]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - Backup interface clock enable"]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR` reader - Power interface clock enable"]
pub type PWR_R = crate::BitReader;
#[doc = "Field `PWR` writer - Power interface clock enable"]
pub type PWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC` reader - DAC clock enable"]
pub type DAC_R = crate::BitReader;
#[doc = "Field `DAC` writer - DAC clock enable"]
pub type DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7` reader - UART7 clock enable"]
pub type UART7_R = crate::BitReader;
#[doc = "Field `UART7` writer - UART7 clock enable"]
pub type UART7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8` reader - UART8 clock enable"]
pub type UART8_R = crate::BitReader;
#[doc = "Field `UART8` writer - UART8 clock enable"]
pub type UART8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable"]
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline(always)]
    pub fn tim5(&self) -> TIM5_R {
        TIM5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable"]
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 clock enable"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART3 clock enable"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    pub fn uart5(&self) -> UART5_R {
        UART5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN clock enable"]
    #[inline(always)]
    pub fn can(&self) -> CAN_R {
        CAN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 clock enable"]
    #[inline(always)]
    pub fn uart7(&self) -> UART7_R {
        UART7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 clock enable"]
    #[inline(always)]
    pub fn uart8(&self) -> UART8_R {
        UART8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2(&mut self) -> TIM2_W<APB1ENR_SPEC> {
        TIM2_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3(&mut self) -> TIM3_W<APB1ENR_SPEC> {
        TIM3_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4(&mut self) -> TIM4_W<APB1ENR_SPEC> {
        TIM4_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim5(&mut self) -> TIM5_W<APB1ENR_SPEC> {
        TIM5_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6(&mut self) -> TIM6_W<APB1ENR_SPEC> {
        TIM6_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7(&mut self) -> TIM7_W<APB1ENR_SPEC> {
        TIM7_W::new(self, 5)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg(&mut self) -> WWDG_W<APB1ENR_SPEC> {
        WWDG_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<APB1ENR_SPEC> {
        SPI2_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<APB1ENR_SPEC> {
        SPI3_W::new(self, 15)
    }
    #[doc = "Bit 17 - UART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<APB1ENR_SPEC> {
        UART2_W::new(self, 17)
    }
    #[doc = "Bit 18 - UART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<APB1ENR_SPEC> {
        UART3_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4(&mut self) -> UART4_W<APB1ENR_SPEC> {
        UART4_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5(&mut self) -> UART5_W<APB1ENR_SPEC> {
        UART5_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<APB1ENR_SPEC> {
        I2C1_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<APB1ENR_SPEC> {
        I2C2_W::new(self, 22)
    }
    #[doc = "Bit 24 - CRS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CRS_W<APB1ENR_SPEC> {
        CRS_W::new(self, 24)
    }
    #[doc = "Bit 25 - CAN clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can(&mut self) -> CAN_W<APB1ENR_SPEC> {
        CAN_W::new(self, 25)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<APB1ENR_SPEC> {
        BKP_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwr(&mut self) -> PWR_W<APB1ENR_SPEC> {
        PWR_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<APB1ENR_SPEC> {
        DAC_W::new(self, 29)
    }
    #[doc = "Bit 30 - UART7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart7(&mut self) -> UART7_W<APB1ENR_SPEC> {
        UART7_W::new(self, 30)
    }
    #[doc = "Bit 31 - UART8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart8(&mut self) -> UART8_W<APB1ENR_SPEC> {
        UART8_W::new(self, 31)
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
#[doc = "Advanced Peripheral Bus 1 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1ENR_SPEC;
impl crate::RegisterSpec for APB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr::R`](R) reader structure"]
impl crate::Readable for APB1ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure"]
impl crate::Writable for APB1ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1ENR to value 0"]
impl crate::Resettable for APB1ENR_SPEC {
    const RESET_VALUE: u32 = 0;
}
