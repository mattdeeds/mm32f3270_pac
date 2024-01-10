#[doc = "Register `DAT_CRCH` reader"]
pub type R = crate::R<DAT_CRCH_SPEC>;
#[doc = "Field `VAL` reader - The DAT CRC high register value"]
pub type VAL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The DAT CRC high register value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CRC high data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat_crch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAT_CRCH_SPEC;
impl crate::RegisterSpec for DAT_CRCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat_crch::R`](R) reader structure"]
impl crate::Readable for DAT_CRCH_SPEC {}
#[doc = "`reset()` method sets DAT_CRCH to value 0"]
impl crate::Resettable for DAT_CRCH_SPEC {
    const RESET_VALUE: u32 = 0;
}
