#[doc = "Register `TXDR6_B` reader"]
pub type R = crate::R<TXDR6_B_SPEC>;
#[doc = "Register `TXDR6_B` writer"]
pub type W = crate::W<TXDR6_B_SPEC>;
#[doc = "Field `TXDR6` reader - Transmit data register"]
pub type TXDR6_R = crate::FieldReader<u32>;
#[doc = "Field `TXDR6` writer - Transmit data register"]
pub type TXDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr6(&self) -> TXDR6_R {
        TXDR6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdr6(&mut self) -> TXDR6_W<TXDR6_B_SPEC> {
        TXDR6_W::new(self, 0)
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
#[doc = "Basic TX Data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr6_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr6_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDR6_B_SPEC;
impl crate::RegisterSpec for TXDR6_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr6_b::R`](R) reader structure"]
impl crate::Readable for TXDR6_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdr6_b::W`](W) writer structure"]
impl crate::Writable for TXDR6_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR6_B to value 0"]
impl crate::Resettable for TXDR6_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
