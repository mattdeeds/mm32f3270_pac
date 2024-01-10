#[doc = "Register `TXDATA6_P` reader"]
pub type R = crate::R<TXDATA6_P_SPEC>;
#[doc = "Register `TXDATA6_P` writer"]
pub type W = crate::W<TXDATA6_P_SPEC>;
#[doc = "Field `DATA6` reader - DATA6"]
pub type DATA6_R = crate::FieldReader;
#[doc = "Field `DATA6` writer - DATA6"]
pub type DATA6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA6"]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<TXDATA6_P_SPEC> {
        DATA6_W::new(self, 0)
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
#[doc = "Peli TX Data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata6_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata6_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATA6_P_SPEC;
impl crate::RegisterSpec for TXDATA6_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata6_p::R`](R) reader structure"]
impl crate::Readable for TXDATA6_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdata6_p::W`](W) writer structure"]
impl crate::Writable for TXDATA6_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA6_P to value 0"]
impl crate::Resettable for TXDATA6_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
