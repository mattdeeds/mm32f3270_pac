#[doc = "Register `RX_UNDER` reader"]
pub type R = crate::R<RX_UNDER_SPEC>;
#[doc = "Field `RX_UNDER` reader - Read this register to clear the RX_UNDER interrupt(bit 0)of the RAWISR register"]
pub type RX_UNDER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 0)of the RAWISR register"]
    #[inline(always)]
    pub fn rx_under(&self) -> RX_UNDER_R {
        RX_UNDER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_UNDER Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_under::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_UNDER_SPEC;
impl crate::RegisterSpec for RX_UNDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_under::R`](R) reader structure"]
impl crate::Readable for RX_UNDER_SPEC {}
#[doc = "`reset()` method sets RX_UNDER to value 0"]
impl crate::Resettable for RX_UNDER_SPEC {
    const RESET_VALUE: u32 = 0;
}
