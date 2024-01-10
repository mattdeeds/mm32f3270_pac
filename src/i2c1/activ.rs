#[doc = "Register `ACTIV` reader"]
pub type R = crate::R<ACTIV_SPEC>;
#[doc = "Field `ACTIV` reader - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore"]
pub type ACTIV_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore"]
    #[inline(always)]
    pub fn activ(&self) -> ACTIV_R {
        ACTIV_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear ACTIVITY Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`activ::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTIV_SPEC;
impl crate::RegisterSpec for ACTIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`activ::R`](R) reader structure"]
impl crate::Readable for ACTIV_SPEC {}
#[doc = "`reset()` method sets ACTIV to value 0"]
impl crate::Resettable for ACTIV_SPEC {
    const RESET_VALUE: u32 = 0;
}
