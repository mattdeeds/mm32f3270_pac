#[doc = "Register `FRM_NUMH` reader"]
pub type R = crate::R<FRM_NUMH_SPEC>;
#[doc = "Field `FRM_10_8` reader - These bits represent the higher 8 bits of 11 bit frames"]
pub type FRM_10_8_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - These bits represent the higher 8 bits of 11 bit frames"]
    #[inline(always)]
    pub fn frm_10_8(&self) -> FRM_10_8_R {
        FRM_10_8_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_numh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRM_NUMH_SPEC;
impl crate::RegisterSpec for FRM_NUMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frm_numh::R`](R) reader structure"]
impl crate::Readable for FRM_NUMH_SPEC {}
#[doc = "`reset()` method sets FRM_NUMH to value 0"]
impl crate::Resettable for FRM_NUMH_SPEC {
    const RESET_VALUE: u32 = 0;
}
