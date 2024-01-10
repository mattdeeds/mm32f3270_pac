#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AFRH_SPEC>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AFRH_SPEC>;
#[doc = "Field `AFR8` reader - Multiplexing function selection for bit 8 of portx"]
pub type AFR8_R = crate::FieldReader;
#[doc = "Field `AFR8` writer - Multiplexing function selection for bit 8 of portx"]
pub type AFR8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR9` reader - Multiplexing function selection for bit 9 of portx"]
pub type AFR9_R = crate::FieldReader;
#[doc = "Field `AFR9` writer - Multiplexing function selection for bit 9 of portx"]
pub type AFR9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR10` reader - Multiplexing function selection for bit 10 of portx"]
pub type AFR10_R = crate::FieldReader;
#[doc = "Field `AFR10` writer - Multiplexing function selection for bit 10 of portx"]
pub type AFR10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR11` reader - Multiplexing function selection for bit 11 of portx"]
pub type AFR11_R = crate::FieldReader;
#[doc = "Field `AFR11` writer - Multiplexing function selection for bit 11 of portx"]
pub type AFR11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR12` reader - Multiplexing function selection for bit 12 of portx"]
pub type AFR12_R = crate::FieldReader;
#[doc = "Field `AFR12` writer - Multiplexing function selection for bit 12 of portx"]
pub type AFR12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR13` reader - Multiplexing function selection for bit 13 of portx"]
pub type AFR13_R = crate::FieldReader;
#[doc = "Field `AFR13` writer - Multiplexing function selection for bit 13 of portx"]
pub type AFR13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR14` reader - Multiplexing function selection for bit 14 of portx"]
pub type AFR14_R = crate::FieldReader;
#[doc = "Field `AFR14` writer - Multiplexing function selection for bit 14 of portx"]
pub type AFR14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR15` reader - Multiplexing function selection for bit 15 of portx"]
pub type AFR15_R = crate::FieldReader;
#[doc = "Field `AFR15` writer - Multiplexing function selection for bit 15 of portx"]
pub type AFR15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Multiplexing function selection for bit 8 of portx"]
    #[inline(always)]
    pub fn afr8(&self) -> AFR8_R {
        AFR8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Multiplexing function selection for bit 9 of portx"]
    #[inline(always)]
    pub fn afr9(&self) -> AFR9_R {
        AFR9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Multiplexing function selection for bit 10 of portx"]
    #[inline(always)]
    pub fn afr10(&self) -> AFR10_R {
        AFR10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Multiplexing function selection for bit 11 of portx"]
    #[inline(always)]
    pub fn afr11(&self) -> AFR11_R {
        AFR11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Multiplexing function selection for bit 12 of portx"]
    #[inline(always)]
    pub fn afr12(&self) -> AFR12_R {
        AFR12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Multiplexing function selection for bit 13 of portx"]
    #[inline(always)]
    pub fn afr13(&self) -> AFR13_R {
        AFR13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Multiplexing function selection for bit 14 of portx"]
    #[inline(always)]
    pub fn afr14(&self) -> AFR14_R {
        AFR14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Multiplexing function selection for bit 15 of portx"]
    #[inline(always)]
    pub fn afr15(&self) -> AFR15_R {
        AFR15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Multiplexing function selection for bit 8 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr8(&mut self) -> AFR8_W<AFRH_SPEC> {
        AFR8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Multiplexing function selection for bit 9 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr9(&mut self) -> AFR9_W<AFRH_SPEC> {
        AFR9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Multiplexing function selection for bit 10 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr10(&mut self) -> AFR10_W<AFRH_SPEC> {
        AFR10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Multiplexing function selection for bit 11 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr11(&mut self) -> AFR11_W<AFRH_SPEC> {
        AFR11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Multiplexing function selection for bit 12 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr12(&mut self) -> AFR12_W<AFRH_SPEC> {
        AFR12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Multiplexing function selection for bit 13 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr13(&mut self) -> AFR13_W<AFRH_SPEC> {
        AFR13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Multiplexing function selection for bit 14 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr14(&mut self) -> AFR14_W<AFRH_SPEC> {
        AFR14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Multiplexing function selection for bit 15 of portx"]
    #[inline(always)]
    #[must_use]
    pub fn afr15(&mut self) -> AFR15_W<AFRH_SPEC> {
        AFR15_W::new(self, 28)
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
#[doc = "Port Multiplexing Function High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AFRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AFRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRH to value 0xf00f_ffff"]
impl crate::Resettable for AFRH_SPEC {
    const RESET_VALUE: u32 = 0xf00f_ffff;
}
