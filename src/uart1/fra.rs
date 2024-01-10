#[doc = "Register `FRA` reader"]
pub type R = crate::R<FRA_SPEC>;
#[doc = "Register `FRA` writer"]
pub type W = crate::W<FRA_SPEC>;
#[doc = "Field `DIV_Fraction` reader - Fractional part of UARTDIV"]
pub type DIV_FRACTION_R = crate::FieldReader;
#[doc = "Field `DIV_Fraction` writer - Fractional part of UARTDIV"]
pub type DIV_FRACTION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Fractional part of UARTDIV"]
    #[inline(always)]
    pub fn div_fraction(&self) -> DIV_FRACTION_R {
        DIV_FRACTION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Fractional part of UARTDIV"]
    #[inline(always)]
    #[must_use]
    pub fn div_fraction(&mut self) -> DIV_FRACTION_W<FRA_SPEC> {
        DIV_FRACTION_W::new(self, 0)
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
#[doc = "Fractional baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRA_SPEC;
impl crate::RegisterSpec for FRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fra::R`](R) reader structure"]
impl crate::Readable for FRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fra::W`](W) writer structure"]
impl crate::Writable for FRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRA to value 0"]
impl crate::Resettable for FRA_SPEC {
    const RESET_VALUE: u32 = 0;
}
