#[doc = "Register `CHANY1` reader"]
pub type R = crate::R<CHANY1_SPEC>;
#[doc = "Register `CHANY1` writer"]
pub type W = crate::W<CHANY1_SPEC>;
#[doc = "Field `CHANY_SEL8` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL8_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL8` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL9` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL9_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL9` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL14` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL14_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL14` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL15` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL15_R = crate::FieldReader;
#[doc = "Field `CHANY_SEL15` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type CHANY_SEL15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel8(&self) -> CHANY_SEL8_R {
        CHANY_SEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel9(&self) -> CHANY_SEL9_R {
        CHANY_SEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel14(&self) -> CHANY_SEL14_R {
        CHANY_SEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel15(&self) -> CHANY_SEL15_R {
        CHANY_SEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel8(&mut self) -> CHANY_SEL8_W<CHANY1_SPEC> {
        CHANY_SEL8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel9(&mut self) -> CHANY_SEL9_W<CHANY1_SPEC> {
        CHANY_SEL9_W::new(self, 4)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel14(&mut self) -> CHANY_SEL14_W<CHANY1_SPEC> {
        CHANY_SEL14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn chany_sel15(&mut self) -> CHANY_SEL15_W<CHANY1_SPEC> {
        CHANY_SEL15_W::new(self, 28)
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
#[doc = "Arbitrary channel channel selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chany1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chany1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANY1_SPEC;
impl crate::RegisterSpec for CHANY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chany1::R`](R) reader structure"]
impl crate::Readable for CHANY1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chany1::W`](W) writer structure"]
impl crate::Writable for CHANY1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHANY1 to value 0"]
impl crate::Resettable for CHANY1_SPEC {
    const RESET_VALUE: u32 = 0;
}
