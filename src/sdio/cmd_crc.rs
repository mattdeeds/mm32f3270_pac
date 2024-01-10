#[doc = "Register `CMD_CRC` reader"]
pub type R = crate::R<CMD_CRC_SPEC>;
#[doc = "Field `VAL` reader - CMD CRC register value"]
pub type VAL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - CMD CRC register value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "CMD_CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_crc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_CRC_SPEC;
impl crate::RegisterSpec for CMD_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_crc::R`](R) reader structure"]
impl crate::Readable for CMD_CRC_SPEC {}
#[doc = "`reset()` method sets CMD_CRC to value 0"]
impl crate::Resettable for CMD_CRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
