#[doc = "Register `TXDATA1_P` reader"]
pub type R = crate::R<TXDATA1_P_SPEC>;
#[doc = "Register `TXDATA1_P` writer"]
pub type W = crate::W<TXDATA1_P_SPEC>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA1` writer - DATA1"]
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<TXDATA1_P_SPEC> {
        DATA1_W::new(self, 0)
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
#[doc = "Peli TX Data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata1_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata1_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATA1_P_SPEC;
impl crate::RegisterSpec for TXDATA1_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata1_p::R`](R) reader structure"]
impl crate::Readable for TXDATA1_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdata1_p::W`](W) writer structure"]
impl crate::Writable for TXDATA1_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA1_P to value 0"]
impl crate::Resettable for TXDATA1_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
