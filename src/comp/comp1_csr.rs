#[doc = "Register `COMP1_CSR` reader"]
pub type R = crate::R<COMP1_CSR_SPEC>;
#[doc = "Register `COMP1_CSR` writer"]
pub type W = crate::W<COMP1_CSR_SPEC>;
#[doc = "Field `EN` reader - Comparator 1 enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 1 enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Comparator 1 mode"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Comparator 1 mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INM_SEL` reader - Comparator 1 inverting input selection"]
pub type INM_SEL_R = crate::FieldReader;
#[doc = "Field `INM_SEL` writer - Comparator 1 inverting input selection"]
pub type INM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INP_SEL` reader - Comparator 1 normal phase input selection"]
pub type INP_SEL_R = crate::FieldReader;
#[doc = "Field `INP_SEL` writer - Comparator 1 normal phase input selection"]
pub type INP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUT_SEL` reader - Comparator 1 output selection"]
pub type OUT_SEL_R = crate::FieldReader;
#[doc = "Field `OUT_SEL` writer - Comparator 1 output selection"]
pub type OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POL` reader - Comparator 1 output polarity"]
pub type POL_R = crate::BitReader;
#[doc = "Field `POL` writer - Comparator 1 output polarity"]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Comparator 1 hysteresis"]
pub type HYST_R = crate::FieldReader;
#[doc = "Field `HYST` writer - Comparator 1 hysteresis"]
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFLT` reader - Comparator 1 output filter"]
pub type OFLT_R = crate::FieldReader;
#[doc = "Field `OFLT` writer - Comparator 1 output filter"]
pub type OFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT` reader - Comparator 1 output status"]
pub type OUT_R = crate::BitReader;
#[doc = "Field `LOCK` reader - Comparator 1 lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Comparator 1 lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn inm_sel(&self) -> INM_SEL_R {
        INM_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Comparator 1 normal phase input selection"]
    #[inline(always)]
    pub fn inp_sel(&self) -> INP_SEL_R {
        INP_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 1 output filter"]
    #[inline(always)]
    pub fn oflt(&self) -> OFLT_R {
        OFLT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 1 output status"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 1 lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<COMP1_CSR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<COMP1_CSR_SPEC> {
        MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Comparator 1 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn inm_sel(&mut self) -> INM_SEL_W<COMP1_CSR_SPEC> {
        INM_SEL_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - Comparator 1 normal phase input selection"]
    #[inline(always)]
    #[must_use]
    pub fn inp_sel(&mut self) -> INP_SEL_W<COMP1_CSR_SPEC> {
        INP_SEL_W::new(self, 7)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<COMP1_CSR_SPEC> {
        OUT_SEL_W::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 1 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<COMP1_CSR_SPEC> {
        POL_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<COMP1_CSR_SPEC> {
        HYST_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 1 output filter"]
    #[inline(always)]
    #[must_use]
    pub fn oflt(&mut self) -> OFLT_W<COMP1_CSR_SPEC> {
        OFLT_W::new(self, 18)
    }
    #[doc = "Bit 31 - Comparator 1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<COMP1_CSR_SPEC> {
        LOCK_W::new(self, 31)
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
#[doc = "COMP1 Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_csr::R`](R) reader structure"]
impl crate::Readable for COMP1_CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure"]
impl crate::Writable for COMP1_CSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
