#[doc = "Register `RX_DONE` reader"]
pub type R = crate::R<RX_DONE_SPEC>;
#[doc = "Field `RX_DONE` reader - Read this register to clear the RX_UNDER interrupt(bit 7)of the RAWISR register"]
pub type RX_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 7)of the RAWISR register"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_DONE Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_done::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DONE_SPEC;
impl crate::RegisterSpec for RX_DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_done::R`](R) reader structure"]
impl crate::Readable for RX_DONE_SPEC {}
#[doc = "`reset()` method sets RX_DONE to value 0"]
impl crate::Resettable for RX_DONE_SPEC {
    const RESET_VALUE: u32 = 0;
}
