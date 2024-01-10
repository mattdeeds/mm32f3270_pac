#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDR_SPEC>;
#[doc = "Field `RXREG` reader - Receive data register"]
pub type RXREG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data register"]
    #[inline(always)]
    pub fn rxreg(&self) -> RXREG_R {
        RXREG_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDR_SPEC {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
