#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `ADIE` reader - ADC interrupt enable"]
pub type ADIE_R = crate::BitReader;
#[doc = "Field `ADIE` writer - ADC interrupt enable"]
pub type ADIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADWIE` reader - ADC window comparator interrupt enable"]
pub type ADWIE_R = crate::BitReader;
#[doc = "Field `ADWIE` writer - ADC window comparator interrupt enable"]
pub type ADWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGEN` reader - External trigger enable"]
pub type TRGEN_R = crate::BitReader;
#[doc = "Field `TRGEN` writer - External trigger enable"]
pub type TRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - Direct memory access enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - Direct memory access enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSELL` reader - External trigger selection_l"]
pub type TRGSELL_R = crate::FieldReader;
#[doc = "Field `TRGSELL` writer - External trigger selection_l"]
pub type TRGSELL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADST` reader - ADC start"]
pub type ADST_R = crate::BitReader;
#[doc = "Field `ADST` writer - ADC start"]
pub type ADST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMD` reader - ADC mode"]
pub type ADMD_R = crate::FieldReader;
#[doc = "Field `ADMD` writer - ADC mode"]
pub type ADMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALIGN` reader - Data alignment"]
pub type ALIGN_R = crate::BitReader;
#[doc = "Field `ALIGN` writer - Data alignment"]
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCH` reader - Window comparison channel selection"]
pub type CMPCH_R = crate::FieldReader;
#[doc = "Field `CMPCH` writer - Window comparison channel selection"]
pub type CMPCH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCANDIR` reader - ADC scan direction"]
pub type SCANDIR_R = crate::BitReader;
#[doc = "Field `SCANDIR` writer - ADC scan direction"]
pub type SCANDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSELH` reader - External trigger selection_h"]
pub type TRGSELH_R = crate::FieldReader;
#[doc = "Field `TRGSELH` writer - External trigger selection_h"]
pub type TRGSELH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGSHIFT` reader - External trigger shift sample"]
pub type TRGSHIFT_R = crate::FieldReader;
#[doc = "Field `TRGSHIFT` writer - External trigger shift sample"]
pub type TRGSHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRG_EDGE` reader - Trigger edge selection"]
pub type TRG_EDGE_R = crate::FieldReader;
#[doc = "Field `TRG_EDGE` writer - Trigger edge selection"]
pub type TRG_EDGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOSMPIE` reader - ADC end sampling flag interrupt enable"]
pub type EOSMPIE_R = crate::BitReader;
#[doc = "Field `EOSMPIE` writer - ADC end sampling flag interrupt enable"]
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - ADC end of conversion interrupt enable"]
pub type EOCIE_R = crate::BitReader;
#[doc = "Field `EOCIE` writer - ADC end of conversion interrupt enable"]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC interrupt enable"]
    #[inline(always)]
    pub fn adie(&self) -> ADIE_R {
        ADIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt enable"]
    #[inline(always)]
    pub fn adwie(&self) -> ADWIE_R {
        ADWIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Direct memory access enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - External trigger selection_l"]
    #[inline(always)]
    pub fn trgsell(&self) -> TRGSELL_R {
        TRGSELL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC start"]
    #[inline(always)]
    pub fn adst(&self) -> ADST_R {
        ADST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - ADC mode"]
    #[inline(always)]
    pub fn admd(&self) -> ADMD_R {
        ADMD_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Window comparison channel selection"]
    #[inline(always)]
    pub fn cmpch(&self) -> CMPCH_R {
        CMPCH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ADC scan direction"]
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - External trigger selection_h"]
    #[inline(always)]
    pub fn trgselh(&self) -> TRGSELH_R {
        TRGSELH_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - External trigger shift sample"]
    #[inline(always)]
    pub fn trgshift(&self) -> TRGSHIFT_R {
        TRGSHIFT_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Trigger edge selection"]
    #[inline(always)]
    pub fn trg_edge(&self) -> TRG_EDGE_R {
        TRG_EDGE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - ADC end sampling flag interrupt enable"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC end of conversion interrupt enable"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie(&mut self) -> ADIE_W<CR_SPEC> {
        ADIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adwie(&mut self) -> ADWIE_W<CR_SPEC> {
        ADWIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TRGEN_W<CR_SPEC> {
        TRGEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Direct memory access enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CR_SPEC> {
        DMAEN_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - External trigger selection_l"]
    #[inline(always)]
    #[must_use]
    pub fn trgsell(&mut self) -> TRGSELL_W<CR_SPEC> {
        TRGSELL_W::new(self, 4)
    }
    #[doc = "Bit 8 - ADC start"]
    #[inline(always)]
    #[must_use]
    pub fn adst(&mut self) -> ADST_W<CR_SPEC> {
        ADST_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - ADC mode"]
    #[inline(always)]
    #[must_use]
    pub fn admd(&mut self) -> ADMD_W<CR_SPEC> {
        ADMD_W::new(self, 9)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<CR_SPEC> {
        ALIGN_W::new(self, 11)
    }
    #[doc = "Bits 12:15 - Window comparison channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpch(&mut self) -> CMPCH_W<CR_SPEC> {
        CMPCH_W::new(self, 12)
    }
    #[doc = "Bit 16 - ADC scan direction"]
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> SCANDIR_W<CR_SPEC> {
        SCANDIR_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - External trigger selection_h"]
    #[inline(always)]
    #[must_use]
    pub fn trgselh(&mut self) -> TRGSELH_W<CR_SPEC> {
        TRGSELH_W::new(self, 17)
    }
    #[doc = "Bits 19:21 - External trigger shift sample"]
    #[inline(always)]
    #[must_use]
    pub fn trgshift(&mut self) -> TRGSHIFT_W<CR_SPEC> {
        TRGSHIFT_W::new(self, 19)
    }
    #[doc = "Bits 24:25 - Trigger edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn trg_edge(&mut self) -> TRG_EDGE_W<CR_SPEC> {
        TRG_EDGE_W::new(self, 24)
    }
    #[doc = "Bit 26 - ADC end sampling flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<CR_SPEC> {
        EOSMPIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - ADC end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<CR_SPEC> {
        EOCIE_W::new(self, 27)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
