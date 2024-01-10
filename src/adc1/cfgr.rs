#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Field `ADEN` reader - ADC enable"]
pub type ADEN_R = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC enable"]
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADWEN` reader - ADC window comparison enable"]
pub type ADWEN_R = crate::BitReader;
#[doc = "Field `ADWEN` writer - ADC window comparison enable"]
pub type ADWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - Temperature sensor enable"]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - Temperature sensor enable"]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEN` reader - Voltage Sensor enable"]
pub type VSEN_R = crate::BitReader;
#[doc = "Field `VSEN` writer - Voltage Sensor enable"]
pub type VSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPRE_H` reader - ADC prescaler_h"]
pub type ADCPRE_H_R = crate::FieldReader;
#[doc = "Field `ADCPRE_H` writer - ADC prescaler_h"]
pub type ADCPRE_H_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSLTCTL` reader - Resolution"]
pub type RSLTCTL_R = crate::FieldReader;
#[doc = "Field `RSLTCTL` writer - Resolution"]
pub type RSLTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADCPRE_L` reader - ADC prescaler_l"]
pub type ADCPRE_L_R = crate::BitReader;
#[doc = "Field `ADCPRE_L` writer - ADC prescaler_l"]
pub type ADCPRE_L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADWEN` reader - Inject ADC conversion window comparison enable"]
pub type JADWEN_R = crate::BitReader;
#[doc = "Field `JADWEN` writer - Inject ADC conversion window comparison enable"]
pub type JADWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparison enable"]
    #[inline(always)]
    pub fn adwen(&self) -> ADWEN_R {
        ADWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Sensor enable"]
    #[inline(always)]
    pub fn vsen(&self) -> VSEN_R {
        VSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - ADC prescaler_h"]
    #[inline(always)]
    pub fn adcpre_h(&self) -> ADCPRE_H_R {
        ADCPRE_H_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Resolution"]
    #[inline(always)]
    pub fn rsltctl(&self) -> RSLTCTL_R {
        RSLTCTL_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 14 - ADC prescaler_l"]
    #[inline(always)]
    pub fn adcpre_l(&self) -> ADCPRE_L_R {
        ADCPRE_L_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Inject ADC conversion window comparison enable"]
    #[inline(always)]
    pub fn jadwen(&self) -> JADWEN_R {
        JADWEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<CFGR_SPEC> {
        ADEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC window comparison enable"]
    #[inline(always)]
    #[must_use]
    pub fn adwen(&mut self) -> ADWEN_W<CFGR_SPEC> {
        ADWEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Temperature sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CFGR_SPEC> {
        TSEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Voltage Sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsen(&mut self) -> VSEN_W<CFGR_SPEC> {
        VSEN_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - ADC prescaler_h"]
    #[inline(always)]
    #[must_use]
    pub fn adcpre_h(&mut self) -> ADCPRE_H_W<CFGR_SPEC> {
        ADCPRE_H_W::new(self, 4)
    }
    #[doc = "Bits 7:9 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn rsltctl(&mut self) -> RSLTCTL_W<CFGR_SPEC> {
        RSLTCTL_W::new(self, 7)
    }
    #[doc = "Bit 14 - ADC prescaler_l"]
    #[inline(always)]
    #[must_use]
    pub fn adcpre_l(&mut self) -> ADCPRE_L_W<CFGR_SPEC> {
        ADCPRE_L_W::new(self, 14)
    }
    #[doc = "Bit 16 - Inject ADC conversion window comparison enable"]
    #[inline(always)]
    #[must_use]
    pub fn jadwen(&mut self) -> JADWEN_W<CFGR_SPEC> {
        JADWEN_W::new(self, 16)
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
#[doc = "Configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: u32 = 0;
}
