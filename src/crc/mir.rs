#[doc = "Register `MIR` reader"]
pub type R = crate::R<MIR_SPEC>;
#[doc = "Register `MIR` writer"]
pub type W = crate::W<MIR_SPEC>;
#[doc = "Field `MIR` reader - Middle data register"]
pub type MIR_R = crate::FieldReader<u32>;
#[doc = "Field `MIR` writer - Middle data register"]
pub type MIR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Middle data register"]
    #[inline(always)]
    pub fn mir(&self) -> MIR_R {
        MIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Middle data register"]
    #[inline(always)]
    #[must_use]
    pub fn mir(&mut self) -> MIR_W<MIR_SPEC> {
        MIR_W::new(self, 0)
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
#[doc = "Middle data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR_SPEC;
impl crate::RegisterSpec for MIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir::R`](R) reader structure"]
impl crate::Readable for MIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir::W`](W) writer structure"]
impl crate::Writable for MIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIR to value 0xffff_ffff"]
impl crate::Resettable for MIR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
