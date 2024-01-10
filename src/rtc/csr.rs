#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `SECF` reader - Second flag"]
pub type SECF_R = crate::BitReader;
#[doc = "Field `SECF` writer - Second flag"]
pub type SECF_W<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `ALRF` reader - Alarm flag"]
pub type ALRF_R = crate::BitReader;
#[doc = "Field `ALRF` writer - Alarm flag"]
pub type ALRF_W<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `OWF` reader - Overflow flag"]
pub type OWF_R = crate::BitReader;
#[doc = "Field `OWF` writer - Overflow flag"]
pub type OWF_W<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `RSF` reader - Registers synchronized flag"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - Registers synchronized flag"]
pub type RSF_W<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `CNF` reader - Configuration flag"]
pub type CNF_R = crate::BitReader;
#[doc = "Field `CNF` writer - Configuration flag"]
pub type CNF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOFF` reader - RTC operation OFF"]
pub type RTOFF_R = crate::BitReader;
#[doc = "Field `ALPEN` reader - RTC alarm loop enable"]
pub type ALPEN_R = crate::BitReader;
#[doc = "Field `ALPEN` writer - RTC alarm loop enable"]
pub type ALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Second flag"]
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm flag"]
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow flag"]
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration flag"]
    #[inline(always)]
    pub fn cnf(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn rtoff(&self) -> RTOFF_R {
        RTOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC alarm loop enable"]
    #[inline(always)]
    pub fn alpen(&self) -> ALPEN_R {
        ALPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second flag"]
    #[inline(always)]
    #[must_use]
    pub fn secf(&mut self) -> SECF_W<CSR_SPEC> {
        SECF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrf(&mut self) -> ALRF_W<CSR_SPEC> {
        ALRF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn owf(&mut self) -> OWF_W<CSR_SPEC> {
        OWF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<CSR_SPEC> {
        RSF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration flag"]
    #[inline(always)]
    #[must_use]
    pub fn cnf(&mut self) -> CNF_W<CSR_SPEC> {
        CNF_W::new(self, 4)
    }
    #[doc = "Bit 6 - RTC alarm loop enable"]
    #[inline(always)]
    #[must_use]
    pub fn alpen(&mut self) -> ALPEN_W<CSR_SPEC> {
        ALPEN_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x0f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x20"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: u16 = 0x20;
}
