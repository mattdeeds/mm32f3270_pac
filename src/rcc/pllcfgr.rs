#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PLLCFGR_SPEC>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PLLCFGR_SPEC>;
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PLLSRC_R = crate::BitReader;
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PLLXTPRE_R = crate::BitReader;
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PLLXTPRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_ICTRL` reader - PLL CP current control signals"]
pub type PLL_ICTRL_R = crate::FieldReader;
#[doc = "Field `PLL_ICTRL` writer - PLL CP current control signals"]
pub type PLL_ICTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_LDS` reader - PLL lock detector accuracy select"]
pub type PLL_LDS_R = crate::FieldReader;
#[doc = "Field `PLL_LDS` writer - PLL lock detector accuracy select"]
pub type PLL_LDS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLDIV` reader - PLL divide factor"]
pub type PLLDIV_R = crate::FieldReader;
#[doc = "Field `PLLDIV` writer - PLL divide factor"]
pub type PLLDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLMUL` reader - PLL multiplication factor"]
pub type PLLMUL_R = crate::FieldReader;
#[doc = "Field `PLLMUL` writer - PLL multiplication factor"]
pub type PLLMUL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PLL CP current control signals"]
    #[inline(always)]
    pub fn pll_ictrl(&self) -> PLL_ICTRL_R {
        PLL_ICTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PLL lock detector accuracy select"]
    #[inline(always)]
    pub fn pll_lds(&self) -> PLL_LDS_R {
        PLL_LDS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - PLL divide factor"]
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:22 - PLL multiplication factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLCFGR_SPEC> {
        PLLSRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSE divider for PLL entry"]
    #[inline(always)]
    #[must_use]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W<PLLCFGR_SPEC> {
        PLLXTPRE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - PLL CP current control signals"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ictrl(&mut self) -> PLL_ICTRL_W<PLLCFGR_SPEC> {
        PLL_ICTRL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - PLL lock detector accuracy select"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lds(&mut self) -> PLL_LDS_W<PLLCFGR_SPEC> {
        PLL_LDS_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - PLL divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn plldiv(&mut self) -> PLLDIV_W<PLLCFGR_SPEC> {
        PLLDIV_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - PLL multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<PLLCFGR_SPEC> {
        PLLMUL_W::new(self, 16)
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
#[doc = "PLL Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x0018_031c"]
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: u32 = 0x0018_031c;
}
