#[doc = "Register `SMPR1` reader"]
pub type R = crate::R<SMPR1_SPEC>;
#[doc = "Register `SMPR1` writer"]
pub type W = crate::W<SMPR1_SPEC>;
#[doc = "Field `SAMCTL0` reader - Channel 0 Sample time selection"]
pub type SAMCTL0_R = crate::FieldReader;
#[doc = "Field `SAMCTL0` writer - Channel 0 Sample time selection"]
pub type SAMCTL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL1` reader - Channel 1 Sample time selection"]
pub type SAMCTL1_R = crate::FieldReader;
#[doc = "Field `SAMCTL1` writer - Channel 1 Sample time selection"]
pub type SAMCTL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL2` reader - Channel 2 Sample time selection"]
pub type SAMCTL2_R = crate::FieldReader;
#[doc = "Field `SAMCTL2` writer - Channel 2 Sample time selection"]
pub type SAMCTL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL3` reader - Channel 3 Sample time selection"]
pub type SAMCTL3_R = crate::FieldReader;
#[doc = "Field `SAMCTL3` writer - Channel 3 Sample time selection"]
pub type SAMCTL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL4` reader - Channel 4 Sample time selection"]
pub type SAMCTL4_R = crate::FieldReader;
#[doc = "Field `SAMCTL4` writer - Channel 4 Sample time selection"]
pub type SAMCTL4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL5` reader - Channel 5 Sample time selection"]
pub type SAMCTL5_R = crate::FieldReader;
#[doc = "Field `SAMCTL5` writer - Channel 5 Sample time selection"]
pub type SAMCTL5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL6` reader - Channel 6 Sample time selection"]
pub type SAMCTL6_R = crate::FieldReader;
#[doc = "Field `SAMCTL6` writer - Channel 6 Sample time selection"]
pub type SAMCTL6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMCTL7` reader - Channel 7 Sample time selection"]
pub type SAMCTL7_R = crate::FieldReader;
#[doc = "Field `SAMCTL7` writer - Channel 7 Sample time selection"]
pub type SAMCTL7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel 0 Sample time selection"]
    #[inline(always)]
    pub fn samctl0(&self) -> SAMCTL0_R {
        SAMCTL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 Sample time selection"]
    #[inline(always)]
    pub fn samctl1(&self) -> SAMCTL1_R {
        SAMCTL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 Sample time selection"]
    #[inline(always)]
    pub fn samctl2(&self) -> SAMCTL2_R {
        SAMCTL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 Sample time selection"]
    #[inline(always)]
    pub fn samctl3(&self) -> SAMCTL3_R {
        SAMCTL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Channel 4 Sample time selection"]
    #[inline(always)]
    pub fn samctl4(&self) -> SAMCTL4_R {
        SAMCTL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel 5 Sample time selection"]
    #[inline(always)]
    pub fn samctl5(&self) -> SAMCTL5_R {
        SAMCTL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel 6 Sample time selection"]
    #[inline(always)]
    pub fn samctl6(&self) -> SAMCTL6_R {
        SAMCTL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Channel 7 Sample time selection"]
    #[inline(always)]
    pub fn samctl7(&self) -> SAMCTL7_R {
        SAMCTL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 0 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl0(&mut self) -> SAMCTL0_W<SMPR1_SPEC> {
        SAMCTL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel 1 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl1(&mut self) -> SAMCTL1_W<SMPR1_SPEC> {
        SAMCTL1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Channel 2 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl2(&mut self) -> SAMCTL2_W<SMPR1_SPEC> {
        SAMCTL2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Channel 3 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl3(&mut self) -> SAMCTL3_W<SMPR1_SPEC> {
        SAMCTL3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Channel 4 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl4(&mut self) -> SAMCTL4_W<SMPR1_SPEC> {
        SAMCTL4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Channel 5 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl5(&mut self) -> SAMCTL5_W<SMPR1_SPEC> {
        SAMCTL5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Channel 6 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl6(&mut self) -> SAMCTL6_W<SMPR1_SPEC> {
        SAMCTL6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Channel 7 Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl7(&mut self) -> SAMCTL7_W<SMPR1_SPEC> {
        SAMCTL7_W::new(self, 28)
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
#[doc = "Any channel configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr1::R`](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smpr1::W`](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
