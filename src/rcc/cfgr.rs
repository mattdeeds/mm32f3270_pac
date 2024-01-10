#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SW_R = crate::FieldReader;
#[doc = "Field `SW` writer - System clock switch"]
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SWS_R = crate::FieldReader;
#[doc = "Field `SWS` writer - System clock switch status"]
pub type SWS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HPRE` reader - AHB Prescaler"]
pub type HPRE_R = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB Prescaler"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE1` reader - APB low-speed prescaler(APB1)"]
pub type PPRE1_R = crate::FieldReader;
#[doc = "Field `PPRE1` writer - APB low-speed prescaler(APB1)"]
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE2` reader - APB low-speed prescaler(APB2)"]
pub type PPRE2_R = crate::FieldReader;
#[doc = "Field `PPRE2` writer - APB low-speed prescaler(APB2)"]
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USEPRE` reader - USB prescaler"]
pub type USEPRE_R = crate::FieldReader;
#[doc = "Field `USEPRE` writer - USB prescaler"]
pub type USEPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub type MCO_R = crate::FieldReader;
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub type MCO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB Prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB low-speed prescaler(APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB low-speed prescaler(APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 22:23 - USB prescaler"]
    #[inline(always)]
    pub fn usepre(&self) -> USEPRE_R {
        USEPRE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<CFGR_SPEC> {
        SW_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<CFGR_SPEC> {
        SWS_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - AHB Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<CFGR_SPEC> {
        HPRE_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB low-speed prescaler(APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGR_SPEC> {
        PPRE1_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB low-speed prescaler(APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<CFGR_SPEC> {
        PPRE2_W::new(self, 11)
    }
    #[doc = "Bits 22:23 - USB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn usepre(&mut self) -> USEPRE_W<CFGR_SPEC> {
        USEPRE_W::new(self, 22)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    #[must_use]
    pub fn mco(&mut self) -> MCO_W<CFGR_SPEC> {
        MCO_W::new(self, 24)
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
#[doc = "Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
