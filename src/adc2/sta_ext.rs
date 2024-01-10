#[doc = "Register `STA_EXT` reader"]
pub type R = crate::R<STA_EXT_SPEC>;
#[doc = "Field `VALID` reader - Channel data Valid flag"]
pub type VALID_R = crate::FieldReader;
#[doc = "Field `OVERRUN` reader - Channel data Overrun flag"]
pub type OVERRUN_R = crate::FieldReader;
#[doc = "Field `EOSMPIF` reader - End of sampling interrupt flag"]
pub type EOSMPIF_R = crate::BitReader;
#[doc = "Field `EOCIF` reader - End of conversion interrupt flag"]
pub type EOCIF_R = crate::BitReader;
#[doc = "Field `JEOSMPIF` reader - Injected channel end of sampling interrupt flag"]
pub type JEOSMPIF_R = crate::BitReader;
#[doc = "Field `JEOCIF` reader - Injected channel end of conversion interrupt flag"]
pub type JEOCIF_R = crate::BitReader;
#[doc = "Field `JEOSIF` reader - Injected channel end of sequential conversion interrupt flag"]
pub type JEOSIF_R = crate::BitReader;
#[doc = "Field `JBUSY` reader - Injection mode"]
pub type JBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 2:3 - Channel data Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Channel data Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - End of sampling interrupt flag"]
    #[inline(always)]
    pub fn eosmpif(&self) -> EOSMPIF_R {
        EOSMPIF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - End of conversion interrupt flag"]
    #[inline(always)]
    pub fn eocif(&self) -> EOCIF_R {
        EOCIF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Injected channel end of sampling interrupt flag"]
    #[inline(always)]
    pub fn jeosmpif(&self) -> JEOSMPIF_R {
        JEOSMPIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Injected channel end of conversion interrupt flag"]
    #[inline(always)]
    pub fn jeocif(&self) -> JEOCIF_R {
        JEOCIF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Injected channel end of sequential conversion interrupt flag"]
    #[inline(always)]
    pub fn jeosif(&self) -> JEOSIF_R {
        JEOSIF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Injection mode"]
    #[inline(always)]
    pub fn jbusy(&self) -> JBUSY_R {
        JBUSY_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Extend state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta_ext::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STA_EXT_SPEC;
impl crate::RegisterSpec for STA_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta_ext::R`](R) reader structure"]
impl crate::Readable for STA_EXT_SPEC {}
#[doc = "`reset()` method sets STA_EXT to value 0"]
impl crate::Resettable for STA_EXT_SPEC {
    const RESET_VALUE: u32 = 0;
}
