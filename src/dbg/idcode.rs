#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IDCODE_SPEC>;
#[doc = "Field `DEV_ID` reader - Device identifier"]
pub type DEV_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device identifier"]
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new(self.bits)
    }
}
#[doc = "Device ID code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDCODE_SPEC;
impl crate::RegisterSpec for IDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IDCODE_SPEC {}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IDCODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
