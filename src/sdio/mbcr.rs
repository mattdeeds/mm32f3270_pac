#[doc = "Register `MBCR` reader"]
pub type R = crate::R<MBCR_SPEC>;
#[doc = "Register `MBCR` writer"]
pub type W = crate::W<MBCR_SPEC>;
#[doc = "Field `PMBDTREN` reader - Set SD/MMC/SDIO port auto multiple block data transfer"]
pub type PMBDTREN_R = crate::BitReader;
#[doc = "Field `PMBDTREN` writer - Set SD/MMC/SDIO port auto multiple block data transfer"]
pub type PMBDTREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBDDIR` reader - Select multiple block data transfer direction"]
pub type MBDDIR_R = crate::BitReader;
#[doc = "Field `MBDDIR` writer - Select multiple block data transfer direction"]
pub type MBDDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUTOTR` reader - Set SD/MMC/SDIO port full auto cmd and multiple block data transferring"]
pub type PAUTOTR_R = crate::BitReader;
#[doc = "Field `PAUTOTR` writer - Set SD/MMC/SDIO port full auto cmd and multiple block data transferring"]
pub type PAUTOTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLKP` reader - SD/MMC/SDIO port CLK line polarity"]
pub type PCLKP_R = crate::BitReader;
#[doc = "Field `PCLKP` writer - SD/MMC/SDIO port CLK line polarity"]
pub type PCLKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTSSEL` reader - SD/MMC/SDIO Busy timeout scale selection"]
pub type BTSSEL_R = crate::FieldReader;
#[doc = "Field `BTSSEL` writer - SD/MMC/SDIO Busy timeout scale selection"]
pub type BTSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NTSSEL` reader - SD/MMC/SDIO NAC timeout scale selection"]
pub type NTSSEL_R = crate::FieldReader;
#[doc = "Field `NTSSEL` writer - SD/MMC/SDIO NAC timeout scale selection"]
pub type NTSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Set SD/MMC/SDIO port auto multiple block data transfer"]
    #[inline(always)]
    pub fn pmbdtren(&self) -> PMBDTREN_R {
        PMBDTREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select multiple block data transfer direction"]
    #[inline(always)]
    pub fn mbddir(&self) -> MBDDIR_R {
        MBDDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set SD/MMC/SDIO port full auto cmd and multiple block data transferring"]
    #[inline(always)]
    pub fn pautotr(&self) -> PAUTOTR_R {
        PAUTOTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD/MMC/SDIO port CLK line polarity"]
    #[inline(always)]
    pub fn pclkp(&self) -> PCLKP_R {
        PCLKP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SD/MMC/SDIO Busy timeout scale selection"]
    #[inline(always)]
    pub fn btssel(&self) -> BTSSEL_R {
        BTSSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SD/MMC/SDIO NAC timeout scale selection"]
    #[inline(always)]
    pub fn ntssel(&self) -> NTSSEL_R {
        NTSSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set SD/MMC/SDIO port auto multiple block data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn pmbdtren(&mut self) -> PMBDTREN_W<MBCR_SPEC> {
        PMBDTREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select multiple block data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn mbddir(&mut self) -> MBDDIR_W<MBCR_SPEC> {
        MBDDIR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set SD/MMC/SDIO port full auto cmd and multiple block data transferring"]
    #[inline(always)]
    #[must_use]
    pub fn pautotr(&mut self) -> PAUTOTR_W<MBCR_SPEC> {
        PAUTOTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - SD/MMC/SDIO port CLK line polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pclkp(&mut self) -> PCLKP_W<MBCR_SPEC> {
        PCLKP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - SD/MMC/SDIO Busy timeout scale selection"]
    #[inline(always)]
    #[must_use]
    pub fn btssel(&mut self) -> BTSSEL_W<MBCR_SPEC> {
        BTSSEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - SD/MMC/SDIO NAC timeout scale selection"]
    #[inline(always)]
    #[must_use]
    pub fn ntssel(&mut self) -> NTSSEL_W<MBCR_SPEC> {
        NTSSEL_W::new(self, 6)
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
#[doc = "Multi-block data transmission register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MBCR_SPEC;
impl crate::RegisterSpec for MBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbcr::R`](R) reader structure"]
impl crate::Readable for MBCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mbcr::W`](W) writer structure"]
impl crate::Writable for MBCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MBCR to value 0x10"]
impl crate::Resettable for MBCR_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
