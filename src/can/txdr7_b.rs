#[doc = "Register `TXDR7_B` reader"]
pub type R = crate::R<TXDR7_B_SPEC>;
#[doc = "Register `TXDR7_B` writer"]
pub type W = crate::W<TXDR7_B_SPEC>;
#[doc = "Field `TXDR7` reader - Transmit data register"]
pub type TXDR7_R = crate::FieldReader<u32>;
#[doc = "Field `TXDR7` writer - Transmit data register"]
pub type TXDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr7(&self) -> TXDR7_R {
        TXDR7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdr7(&mut self) -> TXDR7_W<TXDR7_B_SPEC> {
        TXDR7_W::new(self, 0)
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
#[doc = "Basic TX Data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr7_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr7_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDR7_B_SPEC;
impl crate::RegisterSpec for TXDR7_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr7_b::R`](R) reader structure"]
impl crate::Readable for TXDR7_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdr7_b::W`](W) writer structure"]
impl crate::Writable for TXDR7_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR7_B to value 0"]
impl crate::Resettable for TXDR7_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
