#[doc = "Register `TXDR2_B` reader"]
pub type R = crate::R<TXDR2_B_SPEC>;
#[doc = "Register `TXDR2_B` writer"]
pub type W = crate::W<TXDR2_B_SPEC>;
#[doc = "Field `TXDR2` reader - Transmit data register"]
pub type TXDR2_R = crate::FieldReader<u32>;
#[doc = "Field `TXDR2` writer - Transmit data register"]
pub type TXDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr2(&self) -> TXDR2_R {
        TXDR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdr2(&mut self) -> TXDR2_W<TXDR2_B_SPEC> {
        TXDR2_W::new(self, 0)
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
#[doc = "Basic Send Data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr2_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr2_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDR2_B_SPEC;
impl crate::RegisterSpec for TXDR2_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr2_b::R`](R) reader structure"]
impl crate::Readable for TXDR2_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdr2_b::W`](W) writer structure"]
impl crate::Writable for TXDR2_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR2_B to value 0"]
impl crate::Resettable for TXDR2_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
