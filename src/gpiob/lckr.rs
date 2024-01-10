#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LCKR_SPEC>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LCKR_SPEC>;
#[doc = "Field `LCK` reader - Port x Lock bit y"]
pub type LCK_R = crate::FieldReader<u16>;
#[doc = "Field `LCK` writer - Port x Lock bit y"]
pub type LCK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LCKK` reader - Lock key"]
pub type LCKK_R = crate::BitReader;
#[doc = "Field `LCKK` writer - Lock key"]
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Port x Lock bit y"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Lock key"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port x Lock bit y"]
    #[inline(always)]
    #[must_use]
    pub fn lck(&mut self) -> LCK_W<LCKR_SPEC> {
        LCK_W::new(self, 0)
    }
    #[doc = "Bit 16 - Lock key"]
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<LCKR_SPEC> {
        LCKK_W::new(self, 16)
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
#[doc = "Port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LCKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LCKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LCKR_SPEC {
    const RESET_VALUE: u32 = 0;
}
