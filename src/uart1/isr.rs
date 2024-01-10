#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `TX_INTF` reader - Transmit buffer empty interrupt flag bit"]
pub type TX_INTF_R = crate::BitReader;
#[doc = "Field `RX_INTF` reader - Receive valid data interrupt flag bit"]
pub type RX_INTF_R = crate::BitReader;
#[doc = "Field `TXC_INTF` reader - UART Transmit Complete Interrupt Flag bit"]
pub type TXC_INTF_R = crate::BitReader;
#[doc = "Field `RXOERR_INTF` reader - Receive overflow error interrupt flag bit"]
pub type RXOERR_INTF_R = crate::BitReader;
#[doc = "Field `RXPERR_INTF` reader - Parity error interrupt flag bit"]
pub type RXPERR_INTF_R = crate::BitReader;
#[doc = "Field `RXFERR_INTF` reader - Frame error interrupt flag bit"]
pub type RXFERR_INTF_R = crate::BitReader;
#[doc = "Field `RXBRK_INTF` reader - Receive frame break interrupt flag bit"]
pub type RXBRK_INTF_R = crate::BitReader;
#[doc = "Field `TXBRK_INTF` reader - Transmit Break Frame Interrupt Flag Bit"]
pub type TXBRK_INTF_R = crate::BitReader;
#[doc = "Field `RXB8_INTF` reader - Receive Bit 8 Interrupt Flag Bit"]
pub type RXB8_INTF_R = crate::BitReader;
#[doc = "Field `RXIDLE_INTF` reader - Receive frame idle interrupt flag bit"]
pub type RXIDLE_INTF_R = crate::BitReader;
#[doc = "Field `ABREND_INTF` reader - Automatic baud rate end interrupt flag bit"]
pub type ABREND_INTF_R = crate::BitReader;
#[doc = "Field `ABRERR_INTF` reader - Automatic baud rate error interrupt flag bit"]
pub type ABRERR_INTF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit buffer empty interrupt flag bit"]
    #[inline(always)]
    pub fn tx_intf(&self) -> TX_INTF_R {
        TX_INTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive valid data interrupt flag bit"]
    #[inline(always)]
    pub fn rx_intf(&self) -> RX_INTF_R {
        RX_INTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Transmit Complete Interrupt Flag bit"]
    #[inline(always)]
    pub fn txc_intf(&self) -> TXC_INTF_R {
        TXC_INTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt flag bit"]
    #[inline(always)]
    pub fn rxoerr_intf(&self) -> RXOERR_INTF_R {
        RXOERR_INTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error interrupt flag bit"]
    #[inline(always)]
    pub fn rxperr_intf(&self) -> RXPERR_INTF_R {
        RXPERR_INTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame error interrupt flag bit"]
    #[inline(always)]
    pub fn rxferr_intf(&self) -> RXFERR_INTF_R {
        RXFERR_INTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive frame break interrupt flag bit"]
    #[inline(always)]
    pub fn rxbrk_intf(&self) -> RXBRK_INTF_R {
        RXBRK_INTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt Flag Bit"]
    #[inline(always)]
    pub fn txbrk_intf(&self) -> TXBRK_INTF_R {
        TXBRK_INTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt Flag Bit"]
    #[inline(always)]
    pub fn rxb8_intf(&self) -> RXB8_INTF_R {
        RXB8_INTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive frame idle interrupt flag bit"]
    #[inline(always)]
    pub fn rxidle_intf(&self) -> RXIDLE_INTF_R {
        RXIDLE_INTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic baud rate end interrupt flag bit"]
    #[inline(always)]
    pub fn abrend_intf(&self) -> ABREND_INTF_R {
        ABREND_INTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Automatic baud rate error interrupt flag bit"]
    #[inline(always)]
    pub fn abrerr_intf(&self) -> ABRERR_INTF_R {
        ABRERR_INTF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
