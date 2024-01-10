#[doc = "Register `MOD_P` reader"]
pub type R = crate::R<MOD_P_SPEC>;
#[doc = "Register `MOD_P` writer"]
pub type W = crate::W<MOD_P_SPEC>;
#[doc = "Field `RM` reader - Reset mode"]
pub type RM_R = crate::BitReader;
#[doc = "Field `RM` writer - Reset mode"]
pub type RM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOM` reader - Listen only mode"]
pub type LOM_R = crate::BitReader;
#[doc = "Field `LOM` writer - Listen only mode"]
pub type LOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STM` reader - Self test mode"]
pub type STM_R = crate::BitReader;
#[doc = "Field `STM` writer - Self test mode"]
pub type STM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFM` reader - Acceptance filter mode"]
pub type AFM_R = crate::BitReader;
#[doc = "Field `AFM` writer - Acceptance filter mode"]
pub type AFM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset mode"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Listen only mode"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self test mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm(&self) -> AFM_R {
        AFM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset mode"]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RM_W<MOD_P_SPEC> {
        RM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Listen only mode"]
    #[inline(always)]
    #[must_use]
    pub fn lom(&mut self) -> LOM_W<MOD_P_SPEC> {
        LOM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Self test mode"]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> STM_W<MOD_P_SPEC> {
        STM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Acceptance filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn afm(&mut self) -> AFM_W<MOD_P_SPEC> {
        AFM_W::new(self, 3)
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
#[doc = "Peli Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mod_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mod_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOD_P_SPEC;
impl crate::RegisterSpec for MOD_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod_p::R`](R) reader structure"]
impl crate::Readable for MOD_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mod_p::W`](W) writer structure"]
impl crate::Writable for MOD_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOD_P to value 0x01"]
impl crate::Resettable for MOD_P_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
