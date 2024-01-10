#[doc = "Register `PDETCSR` reader"]
pub type R = crate::R<PDETCSR_SPEC>;
#[doc = "Register `PDETCSR` writer"]
pub type W = crate::W<PDETCSR_SPEC>;
#[doc = "Field `PVDE` reader - PVD enable"]
pub type PVDE_R = crate::BitReader;
#[doc = "Field `PVDE` writer - PVD enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PVD threshold selection"]
pub type PLS_R = crate::FieldReader;
#[doc = "Field `PLS` writer - PVD threshold selection"]
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PVDO` reader - PVD output status"]
pub type PVDO_R = crate::BitReader;
#[doc = "Field `PVDO` writer - PVD output status"]
pub type PVDO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDTO` reader - VDT output status"]
pub type VDTO_R = crate::BitReader;
#[doc = "Field `VDTO` writer - VDT output status"]
pub type VDTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDTE` reader - VDT enable"]
pub type VDTE_R = crate::BitReader;
#[doc = "Field `VDTE` writer - VDT enable"]
pub type VDTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDTLS` reader - VDT detection threshold selection"]
pub type VDTLS_R = crate::FieldReader;
#[doc = "Field `VDTLS` writer - VDT detection threshold selection"]
pub type VDTLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VBAT_DIV3` reader - ADC detection VBat_DIV3 partial pressure value enable"]
pub type VBAT_DIV3_R = crate::BitReader;
#[doc = "Field `VBAT_DIV3` writer - ADC detection VBat_DIV3 partial pressure value enable"]
pub type VBAT_DIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PVD enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - PVD threshold selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - PVD output status"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VDT output status"]
    #[inline(always)]
    pub fn vdto(&self) -> VDTO_R {
        VDTO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - VDT enable"]
    #[inline(always)]
    pub fn vdte(&self) -> VDTE_R {
        VDTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - VDT detection threshold selection"]
    #[inline(always)]
    pub fn vdtls(&self) -> VDTLS_R {
        VDTLS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - ADC detection VBat_DIV3 partial pressure value enable"]
    #[inline(always)]
    pub fn vbat_div3(&self) -> VBAT_DIV3_R {
        VBAT_DIV3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PVD enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<PDETCSR_SPEC> {
        PVDE_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - PVD threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<PDETCSR_SPEC> {
        PLS_W::new(self, 1)
    }
    #[doc = "Bit 5 - PVD output status"]
    #[inline(always)]
    #[must_use]
    pub fn pvdo(&mut self) -> PVDO_W<PDETCSR_SPEC> {
        PVDO_W::new(self, 5)
    }
    #[doc = "Bit 6 - VDT output status"]
    #[inline(always)]
    #[must_use]
    pub fn vdto(&mut self) -> VDTO_W<PDETCSR_SPEC> {
        VDTO_W::new(self, 6)
    }
    #[doc = "Bit 8 - VDT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vdte(&mut self) -> VDTE_W<PDETCSR_SPEC> {
        VDTE_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - VDT detection threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn vdtls(&mut self) -> VDTLS_W<PDETCSR_SPEC> {
        VDTLS_W::new(self, 9)
    }
    #[doc = "Bit 11 - ADC detection VBat_DIV3 partial pressure value enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbat_div3(&mut self) -> VBAT_DIV3_W<PDETCSR_SPEC> {
        VBAT_DIV3_W::new(self, 11)
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
#[doc = "Power detection configuration status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdetcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdetcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDETCSR_SPEC;
impl crate::RegisterSpec for PDETCSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdetcsr::R`](R) reader structure"]
impl crate::Readable for PDETCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdetcsr::W`](W) writer structure"]
impl crate::Writable for PDETCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDETCSR to value 0"]
impl crate::Resettable for PDETCSR_SPEC {
    const RESET_VALUE: u16 = 0;
}
