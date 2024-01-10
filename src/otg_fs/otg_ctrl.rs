#[doc = "Register `OTG_CTRL` reader"]
pub type R = crate::R<OTG_CTRL_SPEC>;
#[doc = "Register `OTG_CTRL` writer"]
pub type W = crate::W<OTG_CTRL_SPEC>;
#[doc = "Field `VBUS_DSCHG` reader - When set, this releases the VBUS signal through the resistance"]
pub type VBUS_DSCHG_R = crate::BitReader;
#[doc = "Field `VBUS_DSCHG` writer - When set, this releases the VBUS signal through the resistance"]
pub type VBUS_DSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_CHG` reader - When set, this will release the VBUS signal through the resistance. When set, it will send the VBUS signal through the resistance"]
pub type VBUS_CHG_R = crate::BitReader;
#[doc = "Field `VBUS_CHG` writer - When set, this will release the VBUS signal through the resistance. When set, it will send the VBUS signal through the resistance"]
pub type VBUS_CHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_EN` reader - When set, the pull_up and pull_down controls in this register can be used"]
pub type OTG_EN_R = crate::BitReader;
#[doc = "Field `OTG_EN` writer - When set, the pull_up and pull_down controls in this register can be used"]
pub type OTG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_ON` reader - When set, the VBUS power signal will be turned on"]
pub type VBUS_ON_R = crate::BitReader;
#[doc = "Field `VBUS_ON` writer - When set, the VBUS power signal will be turned on"]
pub type VBUS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_LOW` reader - When set, a pull_down resistor is enabled on the d_reduce_data line"]
pub type DM_LOW_R = crate::BitReader;
#[doc = "Field `DM_LOW` writer - When set, a pull_down resistor is enabled on the d_reduce_data line"]
pub type DM_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_LOW` reader - When set, a pull_down resistor is enabled on the d_plus_data line"]
pub type DP_LOW_R = crate::BitReader;
#[doc = "Field `DP_LOW` writer - When set, a pull_down resistor is enabled on the d_plus_data line"]
pub type DP_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_HIGH` reader - When set, a pull_up resistor is enabled on the d_reduce_data line"]
pub type DM_HIGH_R = crate::BitReader;
#[doc = "Field `DM_HIGH` writer - When set, a pull_up resistor is enabled on the d_reduce_data line"]
pub type DM_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_HIGH` reader - When set, a pull_up resistor is enabled on the d_plus_data line"]
pub type DP_HIGH_R = crate::BitReader;
#[doc = "Field `DP_HIGH` writer - When set, a pull_up resistor is enabled on the d_plus_data line"]
pub type DP_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, this releases the VBUS signal through the resistance"]
    #[inline(always)]
    pub fn vbus_dschg(&self) -> VBUS_DSCHG_R {
        VBUS_DSCHG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, this will release the VBUS signal through the resistance. When set, it will send the VBUS signal through the resistance"]
    #[inline(always)]
    pub fn vbus_chg(&self) -> VBUS_CHG_R {
        VBUS_CHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set, the pull_up and pull_down controls in this register can be used"]
    #[inline(always)]
    pub fn otg_en(&self) -> OTG_EN_R {
        OTG_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set, the VBUS power signal will be turned on"]
    #[inline(always)]
    pub fn vbus_on(&self) -> VBUS_ON_R {
        VBUS_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set, a pull_down resistor is enabled on the d_reduce_data line"]
    #[inline(always)]
    pub fn dm_low(&self) -> DM_LOW_R {
        DM_LOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set, a pull_down resistor is enabled on the d_plus_data line"]
    #[inline(always)]
    pub fn dp_low(&self) -> DP_LOW_R {
        DP_LOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set, a pull_up resistor is enabled on the d_reduce_data line"]
    #[inline(always)]
    pub fn dm_high(&self) -> DM_HIGH_R {
        DM_HIGH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set, a pull_up resistor is enabled on the d_plus_data line"]
    #[inline(always)]
    pub fn dp_high(&self) -> DP_HIGH_R {
        DP_HIGH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, this releases the VBUS signal through the resistance"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_dschg(&mut self) -> VBUS_DSCHG_W<OTG_CTRL_SPEC> {
        VBUS_DSCHG_W::new(self, 0)
    }
    #[doc = "Bit 1 - When set, this will release the VBUS signal through the resistance. When set, it will send the VBUS signal through the resistance"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_chg(&mut self) -> VBUS_CHG_W<OTG_CTRL_SPEC> {
        VBUS_CHG_W::new(self, 1)
    }
    #[doc = "Bit 2 - When set, the pull_up and pull_down controls in this register can be used"]
    #[inline(always)]
    #[must_use]
    pub fn otg_en(&mut self) -> OTG_EN_W<OTG_CTRL_SPEC> {
        OTG_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - When set, the VBUS power signal will be turned on"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_on(&mut self) -> VBUS_ON_W<OTG_CTRL_SPEC> {
        VBUS_ON_W::new(self, 3)
    }
    #[doc = "Bit 4 - When set, a pull_down resistor is enabled on the d_reduce_data line"]
    #[inline(always)]
    #[must_use]
    pub fn dm_low(&mut self) -> DM_LOW_W<OTG_CTRL_SPEC> {
        DM_LOW_W::new(self, 4)
    }
    #[doc = "Bit 5 - When set, a pull_down resistor is enabled on the d_plus_data line"]
    #[inline(always)]
    #[must_use]
    pub fn dp_low(&mut self) -> DP_LOW_W<OTG_CTRL_SPEC> {
        DP_LOW_W::new(self, 5)
    }
    #[doc = "Bit 6 - When set, a pull_up resistor is enabled on the d_reduce_data line"]
    #[inline(always)]
    #[must_use]
    pub fn dm_high(&mut self) -> DM_HIGH_W<OTG_CTRL_SPEC> {
        DM_HIGH_W::new(self, 6)
    }
    #[doc = "Bit 7 - When set, a pull_up resistor is enabled on the d_plus_data line"]
    #[inline(always)]
    #[must_use]
    pub fn dp_high(&mut self) -> DP_HIGH_W<OTG_CTRL_SPEC> {
        DP_HIGH_W::new(self, 7)
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
#[doc = "OTG Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_CTRL_SPEC;
impl crate::RegisterSpec for OTG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_ctrl::R`](R) reader structure"]
impl crate::Readable for OTG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_ctrl::W`](W) writer structure"]
impl crate::Writable for OTG_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_CTRL to value 0"]
impl crate::Resettable for OTG_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
