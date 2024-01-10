#[doc = "Register `ICSCR` reader"]
pub type R = crate::R<ICSCR_SPEC>;
#[doc = "Register `ICSCR` writer"]
pub type W = crate::W<ICSCR_SPEC>;
#[doc = "Field `TIME_CRS_SEL` reader - CRS module flag"]
pub type TIME_CRS_SEL_R = crate::BitReader;
#[doc = "Field `TIME_CRS_SEL` writer - CRS module flag"]
pub type TIME_CRS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI_CAL_SEL` reader - Initial value of internal high-speed clock calibration"]
pub type HSI_CAL_SEL_R = crate::FieldReader;
#[doc = "Field `HSI_CAL_SEL` writer - Initial value of internal high-speed clock calibration"]
pub type HSI_CAL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HSI_CAL_SFT` reader - Internal high-speed clock calibration"]
pub type HSI_CAL_SFT_R = crate::FieldReader<u16>;
#[doc = "Field `HSI_CAL_SFT` writer - Internal high-speed clock calibration"]
pub type HSI_CAL_SFT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - CRS module flag"]
    #[inline(always)]
    pub fn time_crs_sel(&self) -> TIME_CRS_SEL_R {
        TIME_CRS_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 11:15 - Initial value of internal high-speed clock calibration"]
    #[inline(always)]
    pub fn hsi_cal_sel(&self) -> HSI_CAL_SEL_R {
        HSI_CAL_SEL_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:25 - Internal high-speed clock calibration"]
    #[inline(always)]
    pub fn hsi_cal_sft(&self) -> HSI_CAL_SFT_R {
        HSI_CAL_SFT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CRS module flag"]
    #[inline(always)]
    #[must_use]
    pub fn time_crs_sel(&mut self) -> TIME_CRS_SEL_W<ICSCR_SPEC> {
        TIME_CRS_SEL_W::new(self, 0)
    }
    #[doc = "Bits 11:15 - Initial value of internal high-speed clock calibration"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_cal_sel(&mut self) -> HSI_CAL_SEL_W<ICSCR_SPEC> {
        HSI_CAL_SEL_W::new(self, 11)
    }
    #[doc = "Bits 16:25 - Internal high-speed clock calibration"]
    #[inline(always)]
    #[must_use]
    pub fn hsi_cal_sft(&mut self) -> HSI_CAL_SFT_W<ICSCR_SPEC> {
        HSI_CAL_SFT_W::new(self, 16)
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
#[doc = "Internal clock sourcec Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icscr::R`](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icscr::W`](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0x0200_0000"]
impl crate::Resettable for ICSCR_SPEC {
    const RESET_VALUE: u32 = 0x0200_0000;
}
