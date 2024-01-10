#[doc = "Register `JSQR` reader"]
pub type R = crate::R<JSQR_SPEC>;
#[doc = "Register `JSQR` writer"]
pub type W = crate::W<JSQR_SPEC>;
#[doc = "Field `JSQ0` reader - The first conversion channel in the injection sequence"]
pub type JSQ0_R = crate::FieldReader;
#[doc = "Field `JSQ0` writer - The first conversion channel in the injection sequence"]
pub type JSQ0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ1` reader - The second conversion channel in the injection sequence"]
pub type JSQ1_R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - The second conversion channel in the injection sequence"]
pub type JSQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ2` reader - The third conversion channel number in the injection sequence"]
pub type JSQ2_R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - The third conversion channel number in the injection sequence"]
pub type JSQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ3` reader - The fourth conversion channel number in the injection sequence"]
pub type JSQ3_R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - The fourth conversion channel number in the injection sequence"]
pub type JSQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JNUMM` reader - Injection channel sequence length"]
pub type JNUMM_R = crate::FieldReader;
#[doc = "Field `JNUMM` writer - Injection channel sequence length"]
pub type JNUMM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - The first conversion channel in the injection sequence"]
    #[inline(always)]
    pub fn jsq0(&self) -> JSQ0_R {
        JSQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The second conversion channel in the injection sequence"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - The third conversion channel number in the injection sequence"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - The fourth conversion channel number in the injection sequence"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - Injection channel sequence length"]
    #[inline(always)]
    pub fn jnumm(&self) -> JNUMM_R {
        JNUMM_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - The first conversion channel in the injection sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq0(&mut self) -> JSQ0_W<JSQR_SPEC> {
        JSQ0_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - The second conversion channel in the injection sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<JSQR_SPEC> {
        JSQ1_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - The third conversion channel number in the injection sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<JSQR_SPEC> {
        JSQ2_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - The fourth conversion channel number in the injection sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<JSQR_SPEC> {
        JSQ3_W::new(self, 15)
    }
    #[doc = "Bits 20:21 - Injection channel sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn jnumm(&mut self) -> JNUMM_W<JSQR_SPEC> {
        JNUMM_W::new(self, 20)
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
#[doc = "Injection sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jsqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JSQR_SPEC;
impl crate::RegisterSpec for JSQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jsqr::R`](R) reader structure"]
impl crate::Readable for JSQR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`jsqr::W`](W) writer structure"]
impl crate::Writable for JSQR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JSQR_SPEC {
    const RESET_VALUE: u32 = 0;
}
