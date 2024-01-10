#[doc = "Register `JDR0` reader"]
pub type R = crate::R<JDR0_SPEC>;
#[doc = "Field `JDATA` reader - Transfer data"]
pub type JDATA_R = crate::FieldReader<u16>;
#[doc = "Field `JOVERRUN` reader - Overrun flag"]
pub type JOVERRUN_R = crate::BitReader;
#[doc = "Field `JVALID` reader - Valid flag"]
pub type JVALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Transfer data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 21 - Overrun flag"]
    #[inline(always)]
    pub fn joverrun(&self) -> JOVERRUN_R {
        JOVERRUN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Valid flag"]
    #[inline(always)]
    pub fn jvalid(&self) -> JVALID_R {
        JVALID_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Injection data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDR0_SPEC;
impl crate::RegisterSpec for JDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr0::R`](R) reader structure"]
impl crate::Readable for JDR0_SPEC {}
#[doc = "`reset()` method sets JDR0 to value 0"]
impl crate::Resettable for JDR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
