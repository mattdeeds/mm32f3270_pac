#[doc = "Register `FRM_NUML` reader"]
pub type R = crate::R<FRM_NUML_SPEC>;
#[doc = "Field `FRM_7_0` reader - These bits represent the lower 8 bits of 11 bit frames"]
pub type FRM_7_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits represent the lower 8 bits of 11 bit frames"]
    #[inline(always)]
    pub fn frm_7_0(&self) -> FRM_7_0_R {
        FRM_7_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_numl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRM_NUML_SPEC;
impl crate::RegisterSpec for FRM_NUML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frm_numl::R`](R) reader structure"]
impl crate::Readable for FRM_NUML_SPEC {}
#[doc = "`reset()` method sets FRM_NUML to value 0"]
impl crate::Resettable for FRM_NUML_SPEC {
    const RESET_VALUE: u32 = 0;
}
