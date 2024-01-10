#[doc = "Register `START` reader"]
pub type R = crate::R<START_SPEC>;
#[doc = "Field `START` reader - Read this register to clear the START_DET interrupt(bit 10)of the RAWISR register"]
pub type START_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the START_DET interrupt(bit 10)of the RAWISR register"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear START_DET Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for START_SPEC {}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    const RESET_VALUE: u32 = 0;
}
