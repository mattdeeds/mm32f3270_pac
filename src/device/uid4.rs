#[doc = "Register `UID4` reader"]
pub type R = crate::R<UID4_SPEC>;
#[doc = "Field `U_ID` reader - 95:64 unique ID bits"]
pub type U_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 95:64 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> U_ID_R {
        U_ID_R::new(self.bits)
    }
}
#[doc = "Unique product indentification register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uid4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UID4_SPEC;
impl crate::RegisterSpec for UID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uid4::R`](R) reader structure"]
impl crate::Readable for UID4_SPEC {}
#[doc = "`reset()` method sets UID4 to value 0"]
impl crate::Resettable for UID4_SPEC {
    const RESET_VALUE: u32 = 0;
}
