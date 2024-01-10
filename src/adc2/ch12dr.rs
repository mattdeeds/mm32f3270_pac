#[doc = "Register `CH12DR` reader"]
pub type R = crate::R<CH12DR_SPEC>;
#[doc = "Field `DATA` reader - Transfer data"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OVERRUN_R = crate::BitReader;
#[doc = "Field `VALID` reader - Valid flag"]
pub type VALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Transfer data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 20 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Channel 12 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH12DR_SPEC;
impl crate::RegisterSpec for CH12DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch12dr::R`](R) reader structure"]
impl crate::Readable for CH12DR_SPEC {}
#[doc = "`reset()` method sets CH12DR to value 0"]
impl crate::Resettable for CH12DR_SPEC {
    const RESET_VALUE: u32 = 0;
}