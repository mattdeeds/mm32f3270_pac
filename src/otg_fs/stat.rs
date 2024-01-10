#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `ODD` reader - This bit indicates that the last buffer descriptor updated is in the odd set of BDT"]
pub type ODD_R = crate::BitReader;
#[doc = "Field `TX` reader - This bit indicates that the last updated BDT was used for transmit transmission OR receive data transmission"]
pub type TX_R = crate::BitReader;
#[doc = "Field `ENDP` reader - These four bits encode the address of the endpoint that received or sent the previous token."]
pub type ENDP_R = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - This bit indicates that the last buffer descriptor updated is in the odd set of BDT"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the last updated BDT was used for transmit transmission OR receive data transmission"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - These four bits encode the address of the endpoint that received or sent the previous token."]
    #[inline(always)]
    pub fn endp(&self) -> ENDP_R {
        ENDP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
