#[doc = "Register `BLKCNTR` reader"]
pub type R = crate::R<BLKCNTR_SPEC>;
#[doc = "Register `BLKCNTR` writer"]
pub type W = crate::W<BLKCNTR_SPEC>;
#[doc = "Field `CNT` reader - Data block number"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Data block number"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data block number"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data block number"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<BLKCNTR_SPEC> {
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
#[doc = "Data block count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkcntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkcntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLKCNTR_SPEC;
impl crate::RegisterSpec for BLKCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blkcntr::R`](R) reader structure"]
impl crate::Readable for BLKCNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blkcntr::W`](W) writer structure"]
impl crate::Writable for BLKCNTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLKCNTR to value 0x01"]
impl crate::Resettable for BLKCNTR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
