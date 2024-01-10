#[doc = "Register `PS` reader"]
pub type R = crate::R<PS_SPEC>;
#[doc = "Field `PS` reader - Watchdog prescaler counter value"]
pub type PS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog prescaler counter value"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "prescaler Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ps::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PS_SPEC;
impl crate::RegisterSpec for PS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ps::R`](R) reader structure"]
impl crate::Readable for PS_SPEC {}
#[doc = "`reset()` method sets PS to value 0x01"]
impl crate::Resettable for PS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
