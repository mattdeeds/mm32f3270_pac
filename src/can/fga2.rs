#[doc = "Register `FGA2` reader"]
pub type R = crate::R<FGA2_SPEC>;
#[doc = "Register `FGA2` writer"]
pub type W = crate::W<FGA2_SPEC>;
#[doc = "Field `FGA_19_16` reader - Filter group enable"]
pub type FGA_19_16_R = crate::FieldReader;
#[doc = "Field `FGA_19_16` writer - Filter group enable"]
pub type FGA_19_16_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Filter group enable"]
    #[inline(always)]
    pub fn fga_19_16(&self) -> FGA_19_16_R {
        FGA_19_16_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter group enable"]
    #[inline(always)]
    #[must_use]
    pub fn fga_19_16(&mut self) -> FGA_19_16_W<FGA2_SPEC> {
        FGA_19_16_W::new(self, 0)
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
#[doc = "Filter Group Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fga2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fga2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FGA2_SPEC;
impl crate::RegisterSpec for FGA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fga2::R`](R) reader structure"]
impl crate::Readable for FGA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fga2::W`](W) writer structure"]
impl crate::Writable for FGA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FGA2 to value 0"]
impl crate::Resettable for FGA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
