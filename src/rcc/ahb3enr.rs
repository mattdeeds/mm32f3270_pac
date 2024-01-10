#[doc = "Register `AHB3ENR` reader"]
pub type R = crate::R<AHB3ENR_SPEC>;
#[doc = "Register `AHB3ENR` writer"]
pub type W = crate::W<AHB3ENR_SPEC>;
#[doc = "Field `FSMC` reader - *D0"]
pub type FSMC_R = crate::BitReader;
#[doc = "Field `FSMC` writer - *D0"]
pub type FSMC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - *D0"]
    #[inline(always)]
    pub fn fsmc(&self) -> FSMC_R {
        FSMC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - *D0"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc(&mut self) -> FSMC_W<AHB3ENR_SPEC> {
        FSMC_W::new(self, 0)
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
#[doc = "Advanced High Performance Bus 3 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3ENR_SPEC;
impl crate::RegisterSpec for AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3enr::R`](R) reader structure"]
impl crate::Readable for AHB3ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure"]
impl crate::Writable for AHB3ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for AHB3ENR_SPEC {
    const RESET_VALUE: u32 = 0;
}
