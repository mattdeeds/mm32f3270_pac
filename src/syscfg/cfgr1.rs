#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1_SPEC>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1_SPEC>;
#[doc = "Field `MEM_MODE` reader - Memory selection bit is set and cleared by software"]
pub type MEM_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory selection bit is set and cleared by software"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FC_SYNC_EN` reader - FSMC synchronization enable"]
pub type FC_SYNC_EN_R = crate::BitReader;
#[doc = "Field `FC_SYNC_EN` writer - FSMC synchronization enable"]
pub type FC_SYNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC_ODATA_EN` reader - FSMC address data multiplexing pin can only be used as data"]
pub type FC_ODATA_EN_R = crate::BitReader;
#[doc = "Field `FC_ODATA_EN` writer - FSMC address data multiplexing pin can only be used as data"]
pub type FC_ODATA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODESEL` reader - FSMC mode selection"]
pub type MODESEL_R = crate::FieldReader;
#[doc = "Field `MODESEL` writer - FSMC mode selection"]
pub type MODESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Memory selection bit is set and cleared by software"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 27 - FSMC synchronization enable"]
    #[inline(always)]
    pub fn fc_sync_en(&self) -> FC_SYNC_EN_R {
        FC_SYNC_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FSMC address data multiplexing pin can only be used as data"]
    #[inline(always)]
    pub fn fc_odata_en(&self) -> FC_ODATA_EN_R {
        FC_ODATA_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - FSMC mode selection"]
    #[inline(always)]
    pub fn modesel(&self) -> MODESEL_R {
        MODESEL_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory selection bit is set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1_SPEC> {
        MEM_MODE_W::new(self, 0)
    }
    #[doc = "Bit 27 - FSMC synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc_sync_en(&mut self) -> FC_SYNC_EN_W<CFGR1_SPEC> {
        FC_SYNC_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - FSMC address data multiplexing pin can only be used as data"]
    #[inline(always)]
    #[must_use]
    pub fn fc_odata_en(&mut self) -> FC_ODATA_EN_W<CFGR1_SPEC> {
        FC_ODATA_EN_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - FSMC mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn modesel(&mut self) -> MODESEL_W<CFGR1_SPEC> {
        MODESEL_W::new(self, 29)
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
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
