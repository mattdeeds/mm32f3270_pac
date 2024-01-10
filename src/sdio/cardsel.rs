#[doc = "Register `CARDSEL` reader"]
pub type R = crate::R<CARDSEL_SPEC>;
#[doc = "Register `CARDSEL` writer"]
pub type W = crate::W<CARDSEL_SPEC>;
#[doc = "Field `TSCALE` reader - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
pub type TSCALE_R = crate::FieldReader;
#[doc = "Field `TSCALE` writer - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
pub type TSCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PCLKEN` reader - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
pub type PCLKEN_R = crate::BitReader;
#[doc = "Field `PCLKEN` writer - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
pub type PCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTREN` reader - SD/MMC/SDIO controller enable"]
pub type CTREN_R = crate::BitReader;
#[doc = "Field `CTREN` writer - SD/MMC/SDIO controller enable"]
pub type CTREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
    #[inline(always)]
    pub fn tscale(&self) -> TSCALE_R {
        TSCALE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
    #[inline(always)]
    pub fn pclken(&self) -> PCLKEN_R {
        PCLKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO controller enable"]
    #[inline(always)]
    pub fn ctren(&self) -> CTREN_R {
        CTREN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn tscale(&mut self) -> TSCALE_W<CARDSEL_SPEC> {
        TSCALE_W::new(self, 0)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO Time scale base(1Mhz) coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn pclken(&mut self) -> PCLKEN_W<CARDSEL_SPEC> {
        PCLKEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO controller enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctren(&mut self) -> CTREN_W<CARDSEL_SPEC> {
        CTREN_W::new(self, 7)
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
#[doc = "card sell\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cardsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cardsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARDSEL_SPEC;
impl crate::RegisterSpec for CARDSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cardsel::R`](R) reader structure"]
impl crate::Readable for CARDSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cardsel::W`](W) writer structure"]
impl crate::Writable for CARDSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CARDSEL to value 0x40"]
impl crate::Resettable for CARDSEL_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
