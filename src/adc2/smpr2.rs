#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<SMPR2_SPEC>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<SMPR2_SPEC>;
#[doc = "Field `SAMCTL8` reader - Channel 8 Sample time selection"]
pub type SAMCTL8_R = crate::FieldReader;
#[doc = "Field `SAMCTL8` writer - Channel 8 Sample time selection"]
pub type SAMCTL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL9` reader - Channel 9 Sample time selection"]
pub type SAMCTL9_R = crate::FieldReader;
#[doc = "Field `SAMCTL9` writer - Channel 9 Sample time selection"]
pub type SAMCTL9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL10` reader - Channel 10 Sample time selection"]
pub type SAMCTL10_R = crate::FieldReader;
#[doc = "Field `SAMCTL10` writer - Channel 10 Sample time selection"]
pub type SAMCTL10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL11` reader - Channel 11 Sample time selection"]
pub type SAMCTL11_R = crate::FieldReader;
#[doc = "Field `SAMCTL11` writer - Channel 11 Sample time selection"]
pub type SAMCTL11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL12` reader - Channel 12 Sample time selection"]
pub type SAMCTL12_R = crate::FieldReader;
#[doc = "Field `SAMCTL12` writer - Channel 12 Sample time selection"]
pub type SAMCTL12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL13` reader - Channel 13 Sample time selection"]
pub type SAMCTL13_R = crate::FieldReader;
#[doc = "Field `SAMCTL13` writer - Channel 13 Sample time selection"]
pub type SAMCTL13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL14` reader - Channel 14 Sample time selection"]
pub type SAMCTL14_R = crate::FieldReader;
#[doc = "Field `SAMCTL14` writer - Channel 14 Sample time selection"]
pub type SAMCTL14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL15` reader - Channel 15 Sample time selection"]
pub type SAMCTL15_R = crate::FieldReader;
#[doc = "Field `SAMCTL15` writer - Channel 15 Sample time selection"]
pub type SAMCTL15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel 8 Sample time selection"]
    #[inline(always)]
    pub fn samctl8(&self) -> SAMCTL8_R {
        SAMCTL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 9 Sample time selection"]
    #[inline(always)]
    pub fn samctl9(&self) -> SAMCTL9_R {
        SAMCTL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 10 Sample time selection"]
    #[inline(always)]
    pub fn samctl10(&self) -> SAMCTL10_R {
        SAMCTL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 11 Sample time selection"]
    #[inline(always)]
    pub fn samctl11(&self) -> SAMCTL11_R {
        SAMCTL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Channel 12 Sample time selection"]
    #[inline(always)]
    pub fn samctl12(&self) -> SAMCTL12_R {
        SAMCTL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel 13 Sample time selection"]
    #[inline(always)]
    pub fn samctl13(&self) -> SAMCTL13_R {
        SAMCTL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel 14 Sample time selection"]
    #[inline(always)]
    pub fn samctl14(&self) -> SAMCTL14_R {
        SAMCTL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Channel 15 Sample time selection"]
    #[inline(always)]
    pub fn samctl15(&self) -> SAMCTL15_R {
        SAMCTL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 8 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl8(&mut self) -> SAMCTL8_W<SMPR2_SPEC> {
        SAMCTL8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel 9 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl9(&mut self) -> SAMCTL9_W<SMPR2_SPEC> {
        SAMCTL9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Channel 10 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl10(&mut self) -> SAMCTL10_W<SMPR2_SPEC> {
        SAMCTL10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Channel 11 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl11(&mut self) -> SAMCTL11_W<SMPR2_SPEC> {
        SAMCTL11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Channel 12 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl12(&mut self) -> SAMCTL12_W<SMPR2_SPEC> {
        SAMCTL12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Channel 13 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl13(&mut self) -> SAMCTL13_W<SMPR2_SPEC> {
        SAMCTL13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Channel 14 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl14(&mut self) -> SAMCTL14_W<SMPR2_SPEC> {
        SAMCTL14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Channel 15 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl15(&mut self) -> SAMCTL15_W<SMPR2_SPEC> {
        SAMCTL15_W::new(self, 28)
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
#[doc = "Any channel configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
