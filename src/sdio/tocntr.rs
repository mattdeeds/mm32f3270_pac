#[doc = "Register `TOCNTR` reader"]
pub type R = crate::R<TOCNTR_SPEC>;
#[doc = "Register `TOCNTR` writer"]
pub type W = crate::W<TOCNTR_SPEC>;
#[doc = "Field `CNT` reader - Data transfer timeout count"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - Data transfer timeout count"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data transfer timeout count"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data transfer timeout count"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<TOCNTR_SPEC> {
        CNT_W::new(self, 0)
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
#[doc = "Data transfer timeout count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCNTR_SPEC;
impl crate::RegisterSpec for TOCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocntr::R`](R) reader structure"]
impl crate::Readable for TOCNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tocntr::W`](W) writer structure"]
impl crate::Writable for TOCNTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCNTR to value 0x40"]
impl crate::Resettable for TOCNTR_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
