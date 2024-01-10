#[doc = "Register `SLVRCVADDR` reader"]
pub type R = crate::R<SLVRCVADDR_SPEC>;
#[doc = "Field `ADDR` reader - Slave Address"]
pub type ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Slave Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receiver Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slvrcvaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLVRCVADDR_SPEC;
impl crate::RegisterSpec for SLVRCVADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvrcvaddr::R`](R) reader structure"]
impl crate::Readable for SLVRCVADDR_SPEC {}
#[doc = "`reset()` method sets SLVRCVADDR to value 0"]
impl crate::Resettable for SLVRCVADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
