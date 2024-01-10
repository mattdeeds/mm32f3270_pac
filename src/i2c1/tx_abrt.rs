#[doc = "Register `TX_ABRT` reader"]
pub type R = crate::R<TX_ABRT_SPEC>;
#[doc = "Field `TX_ABRT` reader - Read this register to clear the RX_UNDER interrupt(bit 6)of the RAWISR register"]
pub type TX_ABRT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 6)of the RAWISR register"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TX_ABRT_R {
        TX_ABRT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_ABRT Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_abrt::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ABRT_SPEC;
impl crate::RegisterSpec for TX_ABRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_abrt::R`](R) reader structure"]
impl crate::Readable for TX_ABRT_SPEC {}
#[doc = "`reset()` method sets TX_ABRT to value 0"]
impl crate::Resettable for TX_ABRT_SPEC {
    const RESET_VALUE: u32 = 0;
}
