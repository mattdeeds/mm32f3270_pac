#[doc = "Register `CMPR` reader"]
pub type R = crate::R<CMPR_SPEC>;
#[doc = "Register `CMPR` writer"]
pub type W = crate::W<CMPR_SPEC>;
#[doc = "Field `CMPLDATA` reader - ADC 12bit window compare DOWN LEVEL DATA"]
pub type CMPLDATA_R = crate::FieldReader<u16>;
#[doc = "Field `CMPLDATA` writer - ADC 12bit window compare DOWN LEVEL DATA"]
pub type CMPLDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CMPHDATA` reader - ADC 12bit window compare UP LEVEL DATA"]
pub type CMPHDATA_R = crate::FieldReader<u16>;
#[doc = "Field `CMPHDATA` writer - ADC 12bit window compare UP LEVEL DATA"]
pub type CMPHDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ADC 12bit window compare DOWN LEVEL DATA"]
    #[inline(always)]
    pub fn cmpldata(&self) -> CMPLDATA_R {
        CMPLDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC 12bit window compare UP LEVEL DATA"]
    #[inline(always)]
    pub fn cmphdata(&self) -> CMPHDATA_R {
        CMPHDATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC 12bit window compare DOWN LEVEL DATA"]
    #[inline(always)]
    #[must_use]
    pub fn cmpldata(&mut self) -> CMPLDATA_W<CMPR_SPEC> {
        CMPLDATA_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - ADC 12bit window compare UP LEVEL DATA"]
    #[inline(always)]
    #[must_use]
    pub fn cmphdata(&mut self) -> CMPHDATA_W<CMPR_SPEC> {
        CMPHDATA_W::new(self, 16)
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
#[doc = "Compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPR_SPEC;
impl crate::RegisterSpec for CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr::R`](R) reader structure"]
impl crate::Readable for CMPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpr::W`](W) writer structure"]
impl crate::Writable for CMPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CMPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
