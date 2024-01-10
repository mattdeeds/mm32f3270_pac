#[doc = "Register `JOFR0` reader"]
pub type R = crate::R<JOFR0_SPEC>;
#[doc = "Register `JOFR0` writer"]
pub type W = crate::W<JOFR0_SPEC>;
#[doc = "Field `JOFR` reader - The A_D conversion result of injection channel 0 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
pub type JOFR_R = crate::FieldReader<u16>;
#[doc = "Field `JOFR` writer - The A_D conversion result of injection channel 0 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
pub type JOFR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The A_D conversion result of injection channel 0 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
    #[inline(always)]
    pub fn jofr(&self) -> JOFR_R {
        JOFR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The A_D conversion result of injection channel 0 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
    #[inline(always)]
    #[must_use]
    pub fn jofr(&mut self) -> JOFR_W<JOFR0_SPEC> {
        JOFR_W::new(self, 0)
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
#[doc = "Injection channe 0 data compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JOFR0_SPEC;
impl crate::RegisterSpec for JOFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jofr0::R`](R) reader structure"]
impl crate::Readable for JOFR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`jofr0::W`](W) writer structure"]
impl crate::Writable for JOFR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JOFR0 to value 0"]
impl crate::Resettable for JOFR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
