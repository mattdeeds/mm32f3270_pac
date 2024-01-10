#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `LSION` reader - Internal low-speed oscillator enable"]
pub type LSION_R = crate::BitReader;
#[doc = "Field `LSION` writer - Internal low-speed oscillator enable"]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - Internal low-speed oscillator ready"]
pub type LSIRDY_R = crate::BitReader;
#[doc = "Field `LSIOENLV` reader - LSI output enable lower voltage"]
pub type LSIOENLV_R = crate::BitReader;
#[doc = "Field `LSIOENLV` writer - LSI output enable lower voltage"]
pub type LSIOENLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDRSTEN` reader - PVD reset enable"]
pub type PVDRSTEN_R = crate::BitReader;
#[doc = "Field `PVDRSTEN` writer - PVD reset enable"]
pub type PVDRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUPEN` reader - CPU lockup reset enable"]
pub type LOCKUPEN_R = crate::BitReader;
#[doc = "Field `LOCKUPEN` writer - CPU lockup reset enable"]
pub type LOCKUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDTRSTNEN` reader - Voltage detect reset enable"]
pub type VDTRSTNEN_R = crate::BitReader;
#[doc = "Field `VDTRSTNEN` writer - Voltage detect reset enable"]
pub type VDTRSTNEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDTRSTF` reader - Voltage detect reset flag"]
pub type VDTRSTF_R = crate::BitReader;
#[doc = "Field `PVDRSTF` reader - PVD reset flag"]
pub type PVDRSTF_R = crate::BitReader;
#[doc = "Field `LOCKUPF` reader - CPU lockup reset flag"]
pub type LOCKUPF_R = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINRSTF` reader - PIN reset flag"]
pub type PINRSTF_R = crate::BitReader;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PORRSTF_R = crate::BitReader;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader;
#[doc = "Field `IWDGRSTF` reader - Independent watchdog reset flag"]
pub type IWDGRSTF_R = crate::BitReader;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader;
#[doc = "Field `LPWRRSTF` reader - *D31"]
pub type LPWRRSTF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - LSI output enable lower voltage"]
    #[inline(always)]
    pub fn lsioenlv(&self) -> LSIOENLV_R {
        LSIOENLV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PVD reset enable"]
    #[inline(always)]
    pub fn pvdrsten(&self) -> PVDRSTEN_R {
        PVDRSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU lockup reset enable"]
    #[inline(always)]
    pub fn lockupen(&self) -> LOCKUPEN_R {
        LOCKUPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage detect reset enable"]
    #[inline(always)]
    pub fn vdtrstnen(&self) -> VDTRSTNEN_R {
        VDTRSTNEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 21 - Voltage detect reset flag"]
    #[inline(always)]
    pub fn vdtrstf(&self) -> VDTRSTF_R {
        VDTRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PVD reset flag"]
    #[inline(always)]
    pub fn pvdrstf(&self) -> PVDRSTF_R {
        PVDRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU lockup reset flag"]
    #[inline(always)]
    pub fn lockupf(&self) -> LOCKUPF_R {
        LOCKUPF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - *D31"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<CSR_SPEC> {
        LSION_W::new(self, 0)
    }
    #[doc = "Bit 5 - LSI output enable lower voltage"]
    #[inline(always)]
    #[must_use]
    pub fn lsioenlv(&mut self) -> LSIOENLV_W<CSR_SPEC> {
        LSIOENLV_W::new(self, 5)
    }
    #[doc = "Bit 6 - PVD reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvdrsten(&mut self) -> PVDRSTEN_W<CSR_SPEC> {
        PVDRSTEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU lockup reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn lockupen(&mut self) -> LOCKUPEN_W<CSR_SPEC> {
        LOCKUPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Voltage detect reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn vdtrstnen(&mut self) -> VDTRSTNEN_W<CSR_SPEC> {
        VDTRSTNEN_W::new(self, 8)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<CSR_SPEC> {
        RMVF_W::new(self, 24)
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
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
