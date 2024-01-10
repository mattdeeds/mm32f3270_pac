#[doc = "Register `FGA1` reader"]
pub type R = crate::R<FGA1_SPEC>;
#[doc = "Register `FGA1` writer"]
pub type W = crate::W<FGA1_SPEC>;
#[doc = "Field `FGA_15_8` reader - Filter group enable"]
pub type FGA_15_8_R = crate::FieldReader;
#[doc = "Field `FGA_15_8` writer - Filter group enable"]
pub type FGA_15_8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Filter group enable"]
    #[inline(always)]
    pub fn fga_15_8(&self) -> FGA_15_8_R {
        FGA_15_8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter group enable"]
    #[inline(always)]
    #[must_use]
    pub fn fga_15_8(&mut self) -> FGA_15_8_W<FGA1_SPEC> {
        FGA_15_8_W::new(self, 0)
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
#[doc = "Filter Group Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fga1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fga1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FGA1_SPEC;
impl crate::RegisterSpec for FGA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fga1::R`](R) reader structure"]
impl crate::Readable for FGA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fga1::W`](W) writer structure"]
impl crate::Writable for FGA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FGA1 to value 0"]
impl crate::Resettable for FGA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
