#[doc = "Register `CCR5` reader"]
pub type R = crate::R<CCR5_SPEC>;
#[doc = "Register `CCR5` writer"]
pub type W = crate::W<CCR5_SPEC>;
#[doc = "Field `CCR5` reader - Capture/Compare 5 value"]
pub type CCR5_R = crate::FieldReader<u16>;
#[doc = "Field `CCR5` writer - Capture/Compare 5 value"]
pub type CCR5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 5 value"]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 5 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> CCR5_W<CCR5_SPEC> {
        CCR5_W::new(self, 0)
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
#[doc = "capture/compare register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR5_SPEC;
impl crate::RegisterSpec for CCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5::R`](R) reader structure"]
impl crate::Readable for CCR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr5::W`](W) writer structure"]
impl crate::Writable for CCR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for CCR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
