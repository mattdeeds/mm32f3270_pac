#[doc = "Register `UID1` reader"]
pub type R = crate::R<UID1_SPEC>;
#[doc = "Field `U_ID` reader - 15:0 unique ID bits"]
pub type U_ID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> U_ID_R {
        U_ID_R::new(self.bits)
    }
}
#[doc = "Unique product indentification register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UID1_SPEC;
impl crate::RegisterSpec for UID1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uid1::R`](R) reader structure"]
impl crate::Readable for UID1_SPEC {}
#[doc = "`reset()` method sets UID1 to value 0"]
impl crate::Resettable for UID1_SPEC {
    const RESET_VALUE: u16 = 0;
}
