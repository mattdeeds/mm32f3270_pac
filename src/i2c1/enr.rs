#[doc = "Register `ENR` reader"]
pub type R = crate::R<ENR_SPEC>;
#[doc = "Register `ENR` writer"]
pub type W = crate::W<ENR_SPEC>;
#[doc = "Field `ENABLE` reader - I2C mode enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - I2C mode enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - I2C transfer abort"]
pub type ABORT_R = crate::BitReader;
#[doc = "Field `ABORT` writer - I2C transfer abort"]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C mode enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C transfer abort"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<ENR_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C transfer abort"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<ENR_SPEC> {
        ABORT_W::new(self, 1)
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
#[doc = "Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENR_SPEC;
impl crate::RegisterSpec for ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enr::R`](R) reader structure"]
impl crate::Readable for ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enr::W`](W) writer structure"]
impl crate::Writable for ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENR to value 0"]
impl crate::Resettable for ENR_SPEC {
    const RESET_VALUE: u32 = 0;
}
