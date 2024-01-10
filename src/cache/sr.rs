#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `CS` reader - Cache status"]
pub type CS_R = crate::FieldReader;
#[doc = "Field `INV_STAT` reader - Invalid status"]
pub type INV_STAT_R = crate::BitReader;
#[doc = "Field `POW_STAT` reader - SRAM power response"]
pub type POW_STAT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Cache status"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Invalid status"]
    #[inline(always)]
    pub fn inv_stat(&self) -> INV_STAT_R {
        INV_STAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM power response"]
    #[inline(always)]
    pub fn pow_stat(&self) -> POW_STAT_R {
        POW_STAT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
