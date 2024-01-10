#[doc = "Register `GC` reader"]
pub type R = crate::R<GC_SPEC>;
#[doc = "Field `GC` reader - Read this register to clear the GEN_CALL interrupt(bit 11)of the RAWISR register"]
pub type GC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the GEN_CALL interrupt(bit 11)of the RAWISR register"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear GEN_CALL Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gc::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GC_SPEC;
impl crate::RegisterSpec for GC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gc::R`](R) reader structure"]
impl crate::Readable for GC_SPEC {}
#[doc = "`reset()` method sets GC to value 0"]
impl crate::Resettable for GC_SPEC {
    const RESET_VALUE: u32 = 0;
}
