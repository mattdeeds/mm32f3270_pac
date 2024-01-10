#[doc = "Register `JOFR3` reader"]
pub type R = crate::R<JOFR3_SPEC>;
#[doc = "Register `JOFR3` writer"]
pub type W = crate::W<JOFR3_SPEC>;
#[doc = "Field `JOFR` reader - The A_D conversion result of injection channel 3 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
pub type JOFR_R = crate::FieldReader<u16>;
#[doc = "Field `JOFR` writer - The A_D conversion result of injection channel 3 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
pub type JOFR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The A_D conversion result of injection channel 3 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
    #[inline(always)]
    pub fn jofr(&self) -> JOFR_R {
        JOFR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The A_D conversion result of injection channel 3 is compensated After subtracting the compensation value, the conversion result is obtained and stored in jdata"]
    #[inline(always)]
    #[must_use]
    pub fn jofr(&mut self) -> JOFR_W<JOFR3_SPEC> {
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
#[doc = "Injection channe 3 data compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JOFR3_SPEC;
impl crate::RegisterSpec for JOFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jofr3::R`](R) reader structure"]
impl crate::Readable for JOFR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`jofr3::W`](W) writer structure"]
impl crate::Writable for JOFR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JOFR3 to value 0"]
impl crate::Resettable for JOFR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
