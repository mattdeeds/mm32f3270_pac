#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICR_SPEC>;
#[doc = "Field `ICR` reader - Read this register to clear the combined interrupt"]
pub type ICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the combined interrupt"]
    #[inline(always)]
    pub fn icr(&self) -> ICR_R {
        ICR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear All Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICR_SPEC {}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
