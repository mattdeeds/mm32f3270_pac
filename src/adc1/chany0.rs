#[doc = "Register `CHANY0` reader"]
pub type R = crate::R<CHANY0_SPEC>;
#[doc = "Register `CHANY0` writer"]
pub type W = crate::W<CHANY0_SPEC>;
#[doc = "Field `CHANY_SEL0` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL0_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL0` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL1` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL1_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL1` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL2` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL2_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL2` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL3` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL3_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL3` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL4` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL4_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL4` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL5` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL5_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL5` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL6` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL6_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL6` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL7` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL7_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL7` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel0(&self) -> CHANY_SEL0_R {
        CHANY_SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel1(&self) -> CHANY_SEL1_R {
        CHANY_SEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel2(&self) -> CHANY_SEL2_R {
        CHANY_SEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel3(&self) -> CHANY_SEL3_R {
        CHANY_SEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel4(&self) -> CHANY_SEL4_R {
        CHANY_SEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel5(&self) -> CHANY_SEL5_R {
        CHANY_SEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel6(&self) -> CHANY_SEL6_R {
        CHANY_SEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel7(&self) -> CHANY_SEL7_R {
        CHANY_SEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel0(&mut self) -> CHANY_SEL0_W<CHANY0_SPEC> {
        CHANY_SEL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel1(&mut self) -> CHANY_SEL1_W<CHANY0_SPEC> {
        CHANY_SEL1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel2(&mut self) -> CHANY_SEL2_W<CHANY0_SPEC> {
        CHANY_SEL2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel3(&mut self) -> CHANY_SEL3_W<CHANY0_SPEC> {
        CHANY_SEL3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel4(&mut self) -> CHANY_SEL4_W<CHANY0_SPEC> {
        CHANY_SEL4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel5(&mut self) -> CHANY_SEL5_W<CHANY0_SPEC> {
        CHANY_SEL5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel6(&mut self) -> CHANY_SEL6_W<CHANY0_SPEC> {
        CHANY_SEL6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel7(&mut self) -> CHANY_SEL7_W<CHANY0_SPEC> {
        CHANY_SEL7_W::new(self, 28)
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
#[doc = "Arbitrary channel channel selection register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chany0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chany0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANY0_SPEC;
impl crate::RegisterSpec for CHANY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chany0::R`](R) reader structure"]
impl crate::Readable for CHANY0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chany0::W`](W) writer structure"]
impl crate::Writable for CHANY0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHANY0 to value 0"]
impl crate::Resettable for CHANY0_SPEC {
    const RESET_VALUE: u32 = 0;
}
