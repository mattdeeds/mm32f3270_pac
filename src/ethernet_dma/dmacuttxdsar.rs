#[doc = "Register `DMACUTTXDSAR` reader"]
pub type R = crate::R<DMACUTTXDSAR_SPEC>;
#[doc = "Field `CUTTXDSA` reader - Host Receive Descriptor AddressPointer"]
pub type CUTTXDSA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor AddressPointer"]
    #[inline(always)]
    pub fn cuttxdsa(&self) -> CUTTXDSA_R {
        CUTTXDSA_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacuttxdsar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACUTTXDSAR_SPEC;
impl crate::RegisterSpec for DMACUTTXDSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacuttxdsar::R`](R) reader structure"]
impl crate::Readable for DMACUTTXDSAR_SPEC {}
#[doc = "`reset()` method sets DMACUTTXDSAR to value 0"]
impl crate::Resettable for DMACUTTXDSAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
