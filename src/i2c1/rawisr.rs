#[doc = "Register `RAWISR` reader"]
pub type R = crate::R<RAWISR_SPEC>;
#[doc = "Field `RX_UNDER` reader - Receive buffer under"]
pub type RX_UNDER_R = crate::BitReader;
#[doc = "Field `RX_OVER` reader - Receive buffer over"]
pub type RX_OVER_R = crate::BitReader;
#[doc = "Field `RX_FULL` reader - Receive buffer not empty"]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `TX_OVER` reader - Transmit buffer over"]
pub type TX_OVER_R = crate::BitReader;
#[doc = "Field `TX_EMPTY` reader - Transmit buffer empty"]
pub type TX_EMPTY_R = crate::BitReader;
#[doc = "Field `RD_REQ` reader - Read request"]
pub type RD_REQ_R = crate::BitReader;
#[doc = "Field `TX_ABRT` reader - Transmit abort"]
pub type TX_ABRT_R = crate::BitReader;
#[doc = "Field `RX_DONE` reader - Transmit done"]
pub type RX_DONE_R = crate::BitReader;
#[doc = "Field `ACTIV` reader - This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
pub type ACTIV_R = crate::BitReader;
#[doc = "Field `STOP` reader - Stop condition detection"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `START` reader - Start condition detection"]
pub type START_R = crate::BitReader;
#[doc = "Field `GC` reader - General call"]
pub type GC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive buffer under"]
    #[inline(always)]
    pub fn rx_under(&self) -> RX_UNDER_R {
        RX_UNDER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive buffer over"]
    #[inline(always)]
    pub fn rx_over(&self) -> RX_OVER_R {
        RX_OVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit buffer over"]
    #[inline(always)]
    pub fn tx_over(&self) -> TX_OVER_R {
        TX_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit buffer empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read request"]
    #[inline(always)]
    pub fn rd_req(&self) -> RD_REQ_R {
        RD_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit abort"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TX_ABRT_R {
        TX_ABRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit done"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
    #[inline(always)]
    pub fn activ(&self) -> ACTIV_R {
        ACTIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop condition detection"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start condition detection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - General call"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "RAW Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAWISR_SPEC;
impl crate::RegisterSpec for RAWISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawisr::R`](R) reader structure"]
impl crate::Readable for RAWISR_SPEC {}
#[doc = "`reset()` method sets RAWISR to value 0"]
impl crate::Resettable for RAWISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
