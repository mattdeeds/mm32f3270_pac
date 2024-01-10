#[doc = "Register `TXDR4_B` reader"]
pub type R = crate::R<TXDR4_B_SPEC>;
#[doc = "Register `TXDR4_B` writer"]
pub type W = crate::W<TXDR4_B_SPEC>;
#[doc = "Field `TXDR4` reader - Transmit data register"]
pub type TXDR4_R = crate::FieldReader<u32>;
#[doc = "Field `TXDR4` writer - Transmit data register"]
pub type TXDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr4(&self) -> TXDR4_R {
        TXDR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdr4(&mut self) -> TXDR4_W<TXDR4_B_SPEC> {
        TXDR4_W::new(self, 0)
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
#[doc = "Basic TX Data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr4_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr4_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDR4_B_SPEC;
impl crate::RegisterSpec for TXDR4_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr4_b::R`](R) reader structure"]
impl crate::Readable for TXDR4_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdr4_b::W`](W) writer structure"]
impl crate::Writable for TXDR4_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR4_B to value 0"]
impl crate::Resettable for TXDR4_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
