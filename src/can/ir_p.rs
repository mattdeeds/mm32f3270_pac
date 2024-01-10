#[doc = "Register `IR_P` reader"]
pub type R = crate::R<IR_P_SPEC>;
#[doc = "Field `RI` reader - Receive interrupt"]
pub type RI_R = crate::BitReader;
#[doc = "Field `TI` reader - Transmit interrupt"]
pub type TI_R = crate::BitReader;
#[doc = "Field `EI` reader - Error interrupt"]
pub type EI_R = crate::BitReader;
#[doc = "Field `DOI` reader - Data overrun interrupt"]
pub type DOI_R = crate::BitReader;
#[doc = "Field `EPI` reader - Error passive interrupt"]
pub type EPI_R = crate::BitReader;
#[doc = "Field `ALI` reader - Arbitration lost interrupt"]
pub type ALI_R = crate::BitReader;
#[doc = "Field `BEI` reader - Bus error interrupt"]
pub type BEI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt"]
    #[inline(always)]
    pub fn ei(&self) -> EI_R {
        EI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data overrun interrupt"]
    #[inline(always)]
    pub fn doi(&self) -> DOI_R {
        DOI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Error passive interrupt"]
    #[inline(always)]
    pub fn epi(&self) -> EPI_R {
        EPI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt"]
    #[inline(always)]
    pub fn ali(&self) -> ALI_R {
        ALI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus error interrupt"]
    #[inline(always)]
    pub fn bei(&self) -> BEI_R {
        BEI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir_p::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IR_P_SPEC;
impl crate::RegisterSpec for IR_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_p::R`](R) reader structure"]
impl crate::Readable for IR_P_SPEC {}
#[doc = "`reset()` method sets IR_P to value 0"]
impl crate::Resettable for IR_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
