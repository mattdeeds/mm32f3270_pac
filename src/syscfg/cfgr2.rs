#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2_SPEC>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2_SPEC>;
#[doc = "Field `I2C1_MODE_SEL` reader - I2C1 port mode selection bit"]
pub type I2C1_MODE_SEL_R = crate::BitReader;
#[doc = "Field `I2C1_MODE_SEL` writer - I2C1 port mode selection bit"]
pub type I2C1_MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_MODE_SEL` reader - I2C2 port mode selection bit"]
pub type I2C2_MODE_SEL_R = crate::BitReader;
#[doc = "Field `I2C2_MODE_SEL` writer - I2C2 port mode selection bit"]
pub type I2C2_MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MII_RMII_SEL` reader - Ethernet PHY interface selection"]
pub type MII_RMII_SEL_R = crate::BitReader;
#[doc = "Field `MII_RMII_SEL` writer - Ethernet PHY interface selection"]
pub type MII_RMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAC_SPD_SEL` reader - MAC simplifies media independent interface speed selection"]
pub type MAC_SPD_SEL_R = crate::BitReader;
#[doc = "Field `MAC_SPD_SEL` writer - MAC simplifies media independent interface speed selection"]
pub type MAC_SPD_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - I2C1 port mode selection bit"]
    #[inline(always)]
    pub fn i2c1_mode_sel(&self) -> I2C1_MODE_SEL_R {
        I2C1_MODE_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C2 port mode selection bit"]
    #[inline(always)]
    pub fn i2c2_mode_sel(&self) -> I2C2_MODE_SEL_R {
        I2C2_MODE_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MAC simplifies media independent interface speed selection"]
    #[inline(always)]
    pub fn mac_spd_sel(&self) -> MAC_SPD_SEL_R {
        MAC_SPD_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - I2C1 port mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_mode_sel(&mut self) -> I2C1_MODE_SEL_W<CFGR2_SPEC> {
        I2C1_MODE_SEL_W::new(self, 16)
    }
    #[doc = "Bit 17 - I2C2 port mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_mode_sel(&mut self) -> I2C2_MODE_SEL_W<CFGR2_SPEC> {
        I2C2_MODE_SEL_W::new(self, 17)
    }
    #[doc = "Bit 20 - Ethernet PHY interface selection"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<CFGR2_SPEC> {
        MII_RMII_SEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - MAC simplifies media independent interface speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn mac_spd_sel(&mut self) -> MAC_SPD_SEL_W<CFGR2_SPEC> {
        MAC_SPD_SEL_W::new(self, 21)
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
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
