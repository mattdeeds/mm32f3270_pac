#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2_SPEC>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2_SPEC>;
#[doc = "Field `TIMADV_CLKSEL` reader - TIMADV clock selection"]
pub type TIMADV_CLKSEL_R = crate::BitReader;
#[doc = "Field `TIMADV_CLKSEL` writer - TIMADV clock selection"]
pub type TIMADV_CLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMADV_PRE` reader - SYSCLK clock Frequency division coefficient"]
pub type TIMADV_PRE_R = crate::FieldReader;
#[doc = "Field `TIMADV_PRE` writer - SYSCLK clock Frequency division coefficient"]
pub type TIMADV_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FSMC_PRE` reader - *D8"]
pub type FSMC_PRE_R = crate::FieldReader;
#[doc = "Field `FSMC_PRE` writer - *D8"]
pub type FSMC_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `APB1_CLK_HV_PRE` reader - FSMC_PRE clock Frequency division coefficient"]
pub type APB1_CLK_HV_PRE_R = crate::FieldReader;
#[doc = "Field `APB1_CLK_HV_PRE` writer - FSMC_PRE clock Frequency division coefficient"]
pub type APB1_CLK_HV_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - TIMADV clock selection"]
    #[inline(always)]
    pub fn timadv_clksel(&self) -> TIMADV_CLKSEL_R {
        TIMADV_CLKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SYSCLK clock Frequency division coefficient"]
    #[inline(always)]
    pub fn timadv_pre(&self) -> TIMADV_PRE_R {
        TIMADV_PRE_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:12 - *D8"]
    #[inline(always)]
    pub fn fsmc_pre(&self) -> FSMC_PRE_R {
        FSMC_PRE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - FSMC_PRE clock Frequency division coefficient"]
    #[inline(always)]
    pub fn apb1_clk_hv_pre(&self) -> APB1_CLK_HV_PRE_R {
        APB1_CLK_HV_PRE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TIMADV clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn timadv_clksel(&mut self) -> TIMADV_CLKSEL_W<CFGR2_SPEC> {
        TIMADV_CLKSEL_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - SYSCLK clock Frequency division coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn timadv_pre(&mut self) -> TIMADV_PRE_W<CFGR2_SPEC> {
        TIMADV_PRE_W::new(self, 1)
    }
    #[doc = "Bits 8:12 - *D8"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc_pre(&mut self) -> FSMC_PRE_W<CFGR2_SPEC> {
        FSMC_PRE_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - FSMC_PRE clock Frequency division coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn apb1_clk_hv_pre(&mut self) -> APB1_CLK_HV_PRE_W<CFGR2_SPEC> {
        APB1_CLK_HV_PRE_W::new(self, 16)
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
#[doc = "Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0x0003_1f00"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: u32 = 0x0003_1f00;
}
