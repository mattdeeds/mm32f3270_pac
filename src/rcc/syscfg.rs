#[doc = "Register `SYSCFG` reader"]
pub type R = crate::R<SYSCFG_SPEC>;
#[doc = "Register `SYSCFG` writer"]
pub type W = crate::W<SYSCFG_SPEC>;
#[doc = "Field `PROG_CHECK_EN` reader - Whether to check if the data in Flash is 0xFF when writing Flash"]
pub type PROG_CHECK_EN_R = crate::BitReader;
#[doc = "Field `PROG_CHECK_EN` writer - Whether to check if the data in Flash is 0xFF when writing Flash"]
pub type PROG_CHECK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_1K_CFG` reader - The size of flash page erase"]
pub type SECTOR_1K_CFG_R = crate::BitReader;
#[doc = "Field `SECTOR_1K_CFG` writer - The size of flash page erase"]
pub type SECTOR_1K_CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFT_NRST_RMP` reader - Software mapping NRST"]
pub type SFT_NRST_RMP_R = crate::BitReader;
#[doc = "Field `SFT_NRST_RMP` writer - Software mapping NRST"]
pub type SFT_NRST_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_OSC_TRIM` reader - Calibration of external crystal oscillator"]
pub type PAD_OSC_TRIM_R = crate::FieldReader;
#[doc = "Field `PAD_OSC_TRIM` writer - Calibration of external crystal oscillator"]
pub type PAD_OSC_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSC_LPF_EN` reader - External crystal low-pass filter enable"]
pub type OSC_LPF_EN_R = crate::BitReader;
#[doc = "Field `OSC_LPF_EN` writer - External crystal low-pass filter enable"]
pub type OSC_LPF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Whether to check if the data in Flash is 0xFF when writing Flash"]
    #[inline(always)]
    pub fn prog_check_en(&self) -> PROG_CHECK_EN_R {
        PROG_CHECK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The size of flash page erase"]
    #[inline(always)]
    pub fn sector_1k_cfg(&self) -> SECTOR_1K_CFG_R {
        SECTOR_1K_CFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software mapping NRST"]
    #[inline(always)]
    pub fn sft_nrst_rmp(&self) -> SFT_NRST_RMP_R {
        SFT_NRST_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Calibration of external crystal oscillator"]
    #[inline(always)]
    pub fn pad_osc_trim(&self) -> PAD_OSC_TRIM_R {
        PAD_OSC_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - External crystal low-pass filter enable"]
    #[inline(always)]
    pub fn osc_lpf_en(&self) -> OSC_LPF_EN_R {
        OSC_LPF_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Whether to check if the data in Flash is 0xFF when writing Flash"]
    #[inline(always)]
    #[must_use]
    pub fn prog_check_en(&mut self) -> PROG_CHECK_EN_W<SYSCFG_SPEC> {
        PROG_CHECK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - The size of flash page erase"]
    #[inline(always)]
    #[must_use]
    pub fn sector_1k_cfg(&mut self) -> SECTOR_1K_CFG_W<SYSCFG_SPEC> {
        SECTOR_1K_CFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software mapping NRST"]
    #[inline(always)]
    #[must_use]
    pub fn sft_nrst_rmp(&mut self) -> SFT_NRST_RMP_W<SYSCFG_SPEC> {
        SFT_NRST_RMP_W::new(self, 2)
    }
    #[doc = "Bits 8:12 - Calibration of external crystal oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn pad_osc_trim(&mut self) -> PAD_OSC_TRIM_W<SYSCFG_SPEC> {
        PAD_OSC_TRIM_W::new(self, 8)
    }
    #[doc = "Bit 14 - External crystal low-pass filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc_lpf_en(&mut self) -> OSC_LPF_EN_W<SYSCFG_SPEC> {
        OSC_LPF_EN_W::new(self, 14)
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
#[doc = "System Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_SPEC;
impl crate::RegisterSpec for SYSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg::R`](R) reader structure"]
impl crate::Readable for SYSCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscfg::W`](W) writer structure"]
impl crate::Writable for SYSCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG to value 0x0101"]
impl crate::Resettable for SYSCFG_SPEC {
    const RESET_VALUE: u32 = 0x0101;
}
