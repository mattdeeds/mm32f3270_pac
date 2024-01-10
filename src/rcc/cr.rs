#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub type HSION_R = crate::BitReader;
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - Internal high-speed clock ready flag"]
pub type HSIRDY_R = crate::BitReader;
#[doc = "Field `HSIRDY` writer - Internal high-speed clock ready flag"]
pub type HSIRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDIV` reader - Internal high-speed clock division factor"]
pub type HSIDIV_R = crate::FieldReader;
#[doc = "Field `HSIDIV` writer - Internal high-speed clock division factor"]
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSEON` reader - External high-speed clock enable"]
pub type HSEON_R = crate::BitReader;
#[doc = "Field `HSEON` writer - External high-speed clock enable"]
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - External high-speed clock ready flag"]
pub type HSERDY_R = crate::BitReader;
#[doc = "Field `HSERDY` writer - External high-speed clock ready flag"]
pub type HSERDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEBYP` reader - External high-speed clock bypass"]
pub type HSEBYP_R = crate::BitReader;
#[doc = "Field `HSEBYP` writer - External high-speed clock bypass"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCSON` reader - lock security system enable"]
pub type CCSON_R = crate::BitReader;
#[doc = "Field `CCSON` writer - lock security system enable"]
pub type CCSON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLON` reader - PLL enable"]
pub type PLLON_R = crate::BitReader;
#[doc = "Field `PLLON` writer - PLL enable"]
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader;
#[doc = "Field `PLLRDY` writer - PLL clock ready flag"]
pub type PLLRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Internal high-speed clock division factor"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 16 - External high-speed clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External high-speed clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External high-speed clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - lock security system enable"]
    #[inline(always)]
    pub fn ccson(&self) -> CCSON_R {
        CCSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CR_SPEC> {
        HSION_W::new(self, 0)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HSIRDY_W<CR_SPEC> {
        HSIRDY_W::new(self, 1)
    }
    #[doc = "Bits 11:13 - Internal high-speed clock division factor"]
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HSIDIV_W<CR_SPEC> {
        HSIDIV_W::new(self, 11)
    }
    #[doc = "Bit 16 - External high-speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CR_SPEC> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 17 - External high-speed clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HSERDY_W<CR_SPEC> {
        HSERDY_W::new(self, 17)
    }
    #[doc = "Bit 18 - External high-speed clock bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CR_SPEC> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - lock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccson(&mut self) -> CCSON_W<CR_SPEC> {
        CCSON_W::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<CR_SPEC> {
        PLLON_W::new(self, 24)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdy(&mut self) -> PLLRDY_W<CR_SPEC> {
        PLLRDY_W::new(self, 25)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x01"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
