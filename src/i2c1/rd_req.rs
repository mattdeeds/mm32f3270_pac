#[doc = "Register `RD_REQ` reader"]
pub type R = crate::R<RD_REQ_SPEC>;
#[doc = "Field `RD_REQ` reader - Read this register to clear the RX_UNDER interrupt(bit 5)of the RAWISR register"]
pub type RD_REQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 5)of the RAWISR register"]
    #[inline(always)]
    pub fn rd_req(&self) -> RD_REQ_R {
        RD_REQ_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RD_REQ Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_req::R`](R). WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REQ_SPEC;
impl crate::RegisterSpec for RD_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_req::R`](R) reader structure"]
impl crate::Readable for RD_REQ_SPEC {}
#[doc = "`reset()` method sets RD_REQ to value 0"]
impl crate::Resettable for RD_REQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
