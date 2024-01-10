#[doc = "Register `TX_OVER` reader"]
pub type R = crate::R<TX_OVER_SPEC>;
#[doc = "Field `TX_OVER` reader - Read this register to clear the RX_UNDER interrupt(bit 3)of the RAWISR register"]
pub type TX_OVER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 3)of the RAWISR register"]
    #[inline(always)]
    pub fn tx_over(&self) -> TX_OVER_R {
        TX_OVER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_OVER Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_over::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_OVER_SPEC;
impl crate::RegisterSpec for TX_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_over::R`](R) reader structure"]
impl crate::Readable for TX_OVER_SPEC {}
#[doc = "`reset()` method sets TX_OVER to value 0"]
impl crate::Resettable for TX_OVER_SPEC {
    const RESET_VALUE: u32 = 0;
}
