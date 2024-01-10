#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<CCMR1_OUTPUT_SPEC>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<CCMR1_OUTPUT_SPEC>;
#[doc = "Field `CC1S` reader - Capture/Compare 2 output enable"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 2 output enable"]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - Output compare 1 fast enable"]
pub type OC1FE_R = crate::BitReader;
#[doc = "Field `OC1FE` writer - Output compare 1 fast enable"]
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - Output compare 1 preload enable"]
pub type OC1PE_R = crate::BitReader;
#[doc = "Field `OC1PE` writer - Output compare 1 preload enable"]
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M` reader - Output compare 1 mode"]
pub type OC1M_R = crate::FieldReader;
#[doc = "Field `OC1M` writer - Output compare 1 mode"]
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1CE` reader - Output compare 1 clear enable"]
pub type OC1CE_R = crate::BitReader;
#[doc = "Field `OC1CE` writer - Output compare 1 clear enable"]
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2S` reader - Capure/Compare 2 selection"]
pub type CC2S_R = crate::FieldReader;
#[doc = "Field `CC2S` writer - Capure/Compare 2 selection"]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC2FE` reader - Output compare 4 fast enable"]
pub type OC2FE_R = crate::BitReader;
#[doc = "Field `OC2FE` writer - Output compare 4 fast enable"]
pub type OC2FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - Output compare 2 preload enable"]
pub type OC2PE_R = crate::BitReader;
#[doc = "Field `OC2PE` writer - Output compare 2 preload enable"]
pub type OC2PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M` reader - Output compare 2 mode"]
pub type OC2M_R = crate::FieldReader;
#[doc = "Field `OC2M` writer - Output compare 2 mode"]
pub type OC2M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC2CE` reader - Output compare 2 clear enable"]
pub type OC2CE_R = crate::BitReader;
#[doc = "Field `OC2CE` writer - Output compare 2 clear enable"]
pub type OC2CE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capure/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 2 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_OUTPUT_SPEC> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<CCMR1_OUTPUT_SPEC> {
        OC1FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<CCMR1_OUTPUT_SPEC> {
        OC1PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<CCMR1_OUTPUT_SPEC> {
        OC1M_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<CCMR1_OUTPUT_SPEC> {
        OC1CE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capure/Compare 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<CCMR1_OUTPUT_SPEC> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<CCMR1_OUTPUT_SPEC> {
        OC2FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<CCMR1_OUTPUT_SPEC> {
        OC2PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OC2M_W<CCMR1_OUTPUT_SPEC> {
        OC2M_W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2ce(&mut self) -> OC2CE_W<CCMR1_OUTPUT_SPEC> {
        OC2CE_W::new(self, 15)
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
#[doc = "capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for CCMR1_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for CCMR1_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for CCMR1_OUTPUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
