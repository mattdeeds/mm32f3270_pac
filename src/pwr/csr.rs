#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `WUF` writer - Wakeup flag"]
pub type WUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `SBF` writer - Standby flag"]
pub type SBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDO` reader - PVD output"]
pub type PVDO_R = crate::BitReader;
#[doc = "Field `PVDO` writer - PVD output"]
pub type PVDO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP` reader - Enable WKUP pin"]
pub type EWUP_R = crate::BitReader;
#[doc = "Field `EWUP` writer - Enable WKUP pin"]
pub type EWUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn ewup(&self) -> EWUP_R {
        EWUP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    #[must_use]
    pub fn wuf(&mut self) -> WUF_W<CSR_SPEC> {
        WUF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn sbf(&mut self) -> SBF_W<CSR_SPEC> {
        SBF_W::new(self, 1)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    #[must_use]
    pub fn pvdo(&mut self) -> PVDO_W<CSR_SPEC> {
        PVDO_W::new(self, 2)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    #[must_use]
    pub fn ewup(&mut self) -> EWUP_W<CSR_SPEC> {
        EWUP_W::new(self, 8)
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
#[doc = "Control Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
