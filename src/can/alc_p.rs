#[doc = "Register `ALC_P` reader"]
pub type R = crate::R<ALC_P_SPEC>;
#[doc = "Register `ALC_P` writer"]
pub type W = crate::W<ALC_P_SPEC>;
#[doc = "Field `BITNO` reader - Bit number"]
pub type BITNO_R = crate::FieldReader;
#[doc = "Field `BITNO` writer - Bit number"]
pub type BITNO_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Bit number"]
    #[inline(always)]
    pub fn bitno(&self) -> BITNO_R {
        BITNO_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit number"]
    #[inline(always)]
    #[must_use]
    pub fn bitno(&mut self) -> BITNO_W<ALC_P_SPEC> {
        BITNO_W::new(self, 0)
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
#[doc = "Peli Arbitration Lost Capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alc_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alc_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALC_P_SPEC;
impl crate::RegisterSpec for ALC_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alc_p::R`](R) reader structure"]
impl crate::Readable for ALC_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alc_p::W`](W) writer structure"]
impl crate::Writable for ALC_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALC_P to value 0"]
impl crate::Resettable for ALC_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
