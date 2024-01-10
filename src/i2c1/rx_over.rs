#[doc = "Register `RX_OVER` reader"]
pub type R = crate::R<RX_OVER_SPEC>;
#[doc = "Field `RX_OVER` reader - Read this register to clear the RX_UNDER interrupt(bit 1)of the RAWISR register"]
pub type RX_OVER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 1)of the RAWISR register"]
    #[inline(always)]
    pub fn rx_over(&self) -> RX_OVER_R {
        RX_OVER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_OVER Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_over::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_OVER_SPEC;
impl crate::RegisterSpec for RX_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_over::R`](R) reader structure"]
impl crate::Readable for RX_OVER_SPEC {}
#[doc = "`reset()` method sets RX_OVER to value 0"]
impl crate::Resettable for RX_OVER_SPEC {
    const RESET_VALUE: u32 = 0;
}
