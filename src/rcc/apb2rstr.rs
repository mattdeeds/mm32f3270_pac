#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTR_SPEC>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTR_SPEC>;
#[doc = "Field `TIM1` reader - TIM1 timer reset"]
pub type TIM1_R = crate::BitReader;
#[doc = "Field `TIM1` writer - TIM1 timer reset"]
pub type TIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8` reader - TIM8 timer reset"]
pub type TIM8_R = crate::BitReader;
#[doc = "Field `TIM8` writer - TIM8 timer reset"]
pub type TIM8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - UART1 reset"]
pub type UART1_R = crate::BitReader;
#[doc = "Field `UART1` writer - UART1 reset"]
pub type UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6` reader - UART6 reset"]
pub type UART6_R = crate::BitReader;
#[doc = "Field `UART6` writer - UART6 reset"]
pub type UART6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC1 reset"]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 reset"]
pub type ADC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2` reader - ADC2 reset"]
pub type ADC2_R = crate::BitReader;
#[doc = "Field `ADC2` writer - ADC2 reset"]
pub type ADC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3` reader - ADC3 reset"]
pub type ADC3_R = crate::BitReader;
#[doc = "Field `ADC3` writer - ADC3 reset"]
pub type ADC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 reset"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 reset"]
pub type SPI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG` reader - SYSCFG reset"]
pub type SYSCFG_R = crate::BitReader;
#[doc = "Field `SYSCFG` writer - SYSCFG reset"]
pub type SYSCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - COMP reset"]
pub type COMP_R = crate::BitReader;
#[doc = "Field `COMP` writer - COMP reset"]
pub type COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1(&self) -> TIM1_R {
        TIM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8(&self) -> TIM8_R {
        TIM8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - UART1 reset"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 reset"]
    #[inline(always)]
    pub fn uart6(&self) -> UART6_R {
        UART6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 reset"]
    #[inline(always)]
    pub fn adc2(&self) -> ADC2_R {
        ADC2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC3 reset"]
    #[inline(always)]
    pub fn adc3(&self) -> ADC3_R {
        ADC3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SYSCFG reset"]
    #[inline(always)]
    pub fn syscfg(&self) -> SYSCFG_R {
        SYSCFG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - COMP reset"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1(&mut self) -> TIM1_W<APB2RSTR_SPEC> {
        TIM1_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim8(&mut self) -> TIM8_W<APB2RSTR_SPEC> {
        TIM8_W::new(self, 1)
    }
    #[doc = "Bit 4 - UART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<APB2RSTR_SPEC> {
        UART1_W::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart6(&mut self) -> UART6_W<APB2RSTR_SPEC> {
        UART6_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<APB2RSTR_SPEC> {
        ADC1_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc2(&mut self) -> ADC2_W<APB2RSTR_SPEC> {
        ADC2_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc3(&mut self) -> ADC3_W<APB2RSTR_SPEC> {
        ADC3_W::new(self, 10)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<APB2RSTR_SPEC> {
        SPI1_W::new(self, 12)
    }
    #[doc = "Bit 14 - SYSCFG reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfg(&mut self) -> SYSCFG_W<APB2RSTR_SPEC> {
        SYSCFG_W::new(self, 14)
    }
    #[doc = "Bit 15 - COMP reset"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<APB2RSTR_SPEC> {
        COMP_W::new(self, 15)
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
#[doc = "Advanced Peripheral Bus 2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
