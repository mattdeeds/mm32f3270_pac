#[doc = "Register `PLLDLY` reader"]
pub type R = crate::R<PLLDLY_SPEC>;
#[doc = "Register `PLLDLY` writer"]
pub type W = crate::W<PLLDLY_SPEC>;
#[doc = "Field `PLL_EQU_CNT` reader - *D0"]
pub type PLL_EQU_CNT_R = crate::FieldReader;
#[doc = "Field `PLL_EQU_CNT` writer - *D0"]
pub type PLL_EQU_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *D0"]
    #[inline(always)]
    pub fn pll_equ_cnt(&self) -> PLL_EQU_CNT_R {
        PLL_EQU_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - *D0"]
    #[inline(always)]
    #[must_use]
    pub fn pll_equ_cnt(&mut self) -> PLL_EQU_CNT_W<PLLDLY_SPEC> {
        PLL_EQU_CNT_W::new(self, 0)
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
#[doc = "PLL Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plldly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plldly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLDLY_SPEC;
impl crate::RegisterSpec for PLLDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldly::R`](R) reader structure"]
impl crate::Readable for PLLDLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plldly::W`](W) writer structure"]
impl crate::Writable for PLLDLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDLY to value 0x03eb"]
impl crate::Resettable for PLLDLY_SPEC {
    const RESET_VALUE: u32 = 0x03eb;
}
