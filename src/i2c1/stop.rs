#[doc = "Register `STOP` reader"]
pub type R = crate::R<STOP_SPEC>;
#[doc = "Field `STOP` reader - Read this register to clear the STOP_DET interrupt(bit 9)of the RAWISR register"]
pub type STOP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the STOP_DET interrupt(bit 9)of the RAWISR register"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear STOP_DET Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stop::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STOP_SPEC;
impl crate::RegisterSpec for STOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stop::R`](R) reader structure"]
impl crate::Readable for STOP_SPEC {}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for STOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
