#[doc = "Register `UID3` reader"]
pub type R = crate::R<UID3_SPEC>;
#[doc = "Field `U_ID` reader - 63:32 unique ID bits"]
pub type U_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 63:32 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> U_ID_R {
        U_ID_R::new(self.bits)
    }
}
#[doc = "Unique product indentification register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uid3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UID3_SPEC;
impl crate::RegisterSpec for UID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uid3::R`](R) reader structure"]
impl crate::Readable for UID3_SPEC {}
#[doc = "`reset()` method sets UID3 to value 0"]
impl crate::Resettable for UID3_SPEC {
    const RESET_VALUE: u32 = 0;
}
