#[doc = "Register `DOR1` reader"]
pub type R = crate::R<DOR1_SPEC>;
#[doc = "Register `DOR1` writer"]
pub type W = crate::W<DOR1_SPEC>;
#[doc = "Field `DACC1DOR` reader - DAC channel1 data output"]
pub type DACC1DOR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DOR` writer - DAC channel1 data output"]
pub type DACC1DOR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 data output"]
    #[inline(always)]
    pub fn dacc1dor(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 data output"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dor(&mut self) -> DACC1DOR_W<DOR1_SPEC> {
        DACC1DOR_W::new(self, 0)
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
#[doc = "Channel1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dor1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOR1_SPEC;
impl crate::RegisterSpec for DOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor1::R`](R) reader structure"]
impl crate::Readable for DOR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dor1::W`](W) writer structure"]
impl crate::Writable for DOR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOR1 to value 0"]
impl crate::Resettable for DOR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
