#[doc = "Register `ANY_CR` reader"]
pub type R = crate::R<ANY_CR_SPEC>;
#[doc = "Register `ANY_CR` writer"]
pub type W = crate::W<ANY_CR_SPEC>;
#[doc = "Field `CHANY_MDEN` reader - Any channel configuration mode enable bit"]
pub type CHANY_MDEN_R = crate::BitReader;
#[doc = "Field `CHANY_MDEN` writer - Any channel configuration mode enable bit"]
pub type CHANY_MDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JCEN` reader - Injected channel enable"]
pub type JCEN_R = crate::BitReader;
#[doc = "Field `JCEN` writer - Injected channel enable"]
pub type JCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOSMPIE` reader - Interrupt enable the end of sequence conversion for injected channel"]
pub type JEOSMPIE_R = crate::BitReader;
#[doc = "Field `JEOSMPIE` writer - Interrupt enable the end of sequence conversion for injected channel"]
pub type JEOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOCIE` reader - Interrupt enable the end of conversion for injected channel"]
pub type JEOCIE_R = crate::BitReader;
#[doc = "Field `JEOCIE` writer - Interrupt enable the end of conversion for injected channel"]
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOSIE` reader - Interrupt enable the end of sequence conversion for injected channel"]
pub type JEOSIE_R = crate::BitReader;
#[doc = "Field `JEOSIE` writer - Interrupt enable the end of sequence conversion for injected channel"]
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAUTO` reader - Automatic Injected group conversion"]
pub type JAUTO_R = crate::BitReader;
#[doc = "Field `JAUTO` writer - Automatic Injected group conversion"]
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADST` reader - Start conversion of injected channels"]
pub type JADST_R = crate::BitReader;
#[doc = "Field `JADST` writer - Start conversion of injected channels"]
pub type JADST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTRGEN` reader - External trigger conversion mode for injected channels"]
pub type JTRGEN_R = crate::BitReader;
#[doc = "Field `JTRGEN` writer - External trigger conversion mode for injected channels"]
pub type JTRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTRGSEL` reader - External event select for in-jected group"]
pub type JTRGSEL_R = crate::FieldReader;
#[doc = "Field `JTRGSEL` writer - External event select for in-jected group"]
pub type JTRGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JTRGSHIFT` reader - Injection mode external trigger delay sampling"]
pub type JTRGSHIFT_R = crate::FieldReader;
#[doc = "Field `JTRGSHIFT` writer - Injection mode external trigger delay sampling"]
pub type JTRGSHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JTRGEDGE` reader - Injection mode trigger edge selection"]
pub type JTRGEDGE_R = crate::FieldReader;
#[doc = "Field `JTRGEDGE` writer - Injection mode trigger edge selection"]
pub type JTRGEDGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Any channel configuration mode enable bit"]
    #[inline(always)]
    pub fn chany_mden(&self) -> CHANY_MDEN_R {
        CHANY_MDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Injected channel enable"]
    #[inline(always)]
    pub fn jcen(&self) -> JCEN_R {
        JCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable the end of sequence conversion for injected channel"]
    #[inline(always)]
    pub fn jeosmpie(&self) -> JEOSMPIE_R {
        JEOSMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable the end of conversion for injected channel"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable the end of sequence conversion for injected channel"]
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Injected group conversion"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jadst(&self) -> JADST_R {
        JADST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jtrgen(&self) -> JTRGEN_R {
        JTRGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - External event select for in-jected group"]
    #[inline(always)]
    pub fn jtrgsel(&self) -> JTRGSEL_R {
        JTRGSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Injection mode external trigger delay sampling"]
    #[inline(always)]
    pub fn jtrgshift(&self) -> JTRGSHIFT_R {
        JTRGSHIFT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Injection mode trigger edge selection"]
    #[inline(always)]
    pub fn jtrgedge(&self) -> JTRGEDGE_R {
        JTRGEDGE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Any channel configuration mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn chany_mden(&mut self) -> CHANY_MDEN_W<ANY_CR_SPEC> {
        CHANY_MDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Injected channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn jcen(&mut self) -> JCEN_W<ANY_CR_SPEC> {
        JCEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable the end of sequence conversion for injected channel"]
    #[inline(always)]
    #[must_use]
    pub fn jeosmpie(&mut self) -> JEOSMPIE_W<ANY_CR_SPEC> {
        JEOSMPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable the end of conversion for injected channel"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<ANY_CR_SPEC> {
        JEOCIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable the end of sequence conversion for injected channel"]
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JEOSIE_W<ANY_CR_SPEC> {
        JEOSIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic Injected group conversion"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<ANY_CR_SPEC> {
        JAUTO_W::new(self, 5)
    }
    #[doc = "Bit 6 - Start conversion of injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jadst(&mut self) -> JADST_W<ANY_CR_SPEC> {
        JADST_W::new(self, 6)
    }
    #[doc = "Bit 7 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jtrgen(&mut self) -> JTRGEN_W<ANY_CR_SPEC> {
        JTRGEN_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - External event select for in-jected group"]
    #[inline(always)]
    #[must_use]
    pub fn jtrgsel(&mut self) -> JTRGSEL_W<ANY_CR_SPEC> {
        JTRGSEL_W::new(self, 8)
    }
    #[doc = "Bits 13:15 - Injection mode external trigger delay sampling"]
    #[inline(always)]
    #[must_use]
    pub fn jtrgshift(&mut self) -> JTRGSHIFT_W<ANY_CR_SPEC> {
        JTRGSHIFT_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Injection mode trigger edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn jtrgedge(&mut self) -> JTRGEDGE_W<ANY_CR_SPEC> {
        JTRGEDGE_W::new(self, 16)
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
#[doc = "Arbitrary channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`any_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`any_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANY_CR_SPEC;
impl crate::RegisterSpec for ANY_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`any_cr::R`](R) reader structure"]
impl crate::Readable for ANY_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`any_cr::W`](W) writer structure"]
impl crate::Writable for ANY_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANY_CR to value 0"]
impl crate::Resettable for ANY_CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
