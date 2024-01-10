#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    tar: TAR,
    sar: SAR,
    _reserved3: [u8; 0x04],
    dr: DR,
    sshr: SSHR,
    sslr: SSLR,
    fshr: FSHR,
    fslr: FSLR,
    _reserved8: [u8; 0x08],
    isr: ISR,
    imr: IMR,
    rawisr: RAWISR,
    rxtlr: RXTLR,
    txtlr: TXTLR,
    icr: ICR,
    rx_under: RX_UNDER,
    rx_over: RX_OVER,
    tx_over: TX_OVER,
    rd_req: RD_REQ,
    tx_abrt: TX_ABRT,
    rx_done: RX_DONE,
    activ: ACTIV,
    stop: STOP,
    start: START,
    gc: GC,
    enr: ENR,
    sr: SR,
    txflr: TXFLR,
    rxflr: RXFLR,
    hold: HOLD,
    _reserved29: [u8; 0x08],
    dma: DMA,
    _reserved30: [u8; 0x08],
    setup: SETUP,
    gcr: GCR,
    _reserved32: [u8; 0x14],
    slvmask: SLVMASK,
    slvrcvaddr: SLVRCVADDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Target Register"]
    #[inline(always)]
    pub const fn tar(&self) -> &TAR {
        &self.tar
    }
    #[doc = "0x08 - Slave Address Register"]
    #[inline(always)]
    pub const fn sar(&self) -> &SAR {
        &self.sar
    }
    #[doc = "0x10 - Data Command Register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x14 - SCL High Period Count for Std. Speed Register"]
    #[inline(always)]
    pub const fn sshr(&self) -> &SSHR {
        &self.sshr
    }
    #[doc = "0x18 - SCL Low Period Count for Std. Speed Register"]
    #[inline(always)]
    pub const fn sslr(&self) -> &SSLR {
        &self.sslr
    }
    #[doc = "0x1c - SCL High Period Count for Fast Speed Register"]
    #[inline(always)]
    pub const fn fshr(&self) -> &FSHR {
        &self.fshr
    }
    #[doc = "0x20 - SCL Low Period Count for Fast Speed Register"]
    #[inline(always)]
    pub const fn fslr(&self) -> &FSLR {
        &self.fslr
    }
    #[doc = "0x2c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x30 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x34 - RAW Interrupt Status Register"]
    #[inline(always)]
    pub const fn rawisr(&self) -> &RAWISR {
        &self.rawisr
    }
    #[doc = "0x38 - Receive FIFO Threshold Level Register"]
    #[inline(always)]
    pub const fn rxtlr(&self) -> &RXTLR {
        &self.rxtlr
    }
    #[doc = "0x3c - Transmit FIFO Threshold Level Register"]
    #[inline(always)]
    pub const fn txtlr(&self) -> &TXTLR {
        &self.txtlr
    }
    #[doc = "0x40 - Clear All Interrupt Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    #[inline(always)]
    pub const fn rx_under(&self) -> &RX_UNDER {
        &self.rx_under
    }
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn rx_over(&self) -> &RX_OVER {
        &self.rx_over
    }
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn tx_over(&self) -> &TX_OVER {
        &self.tx_over
    }
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    #[inline(always)]
    pub const fn rd_req(&self) -> &RD_REQ {
        &self.rd_req
    }
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    #[inline(always)]
    pub const fn tx_abrt(&self) -> &TX_ABRT {
        &self.tx_abrt
    }
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    #[inline(always)]
    pub const fn rx_done(&self) -> &RX_DONE {
        &self.rx_done
    }
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    #[inline(always)]
    pub const fn activ(&self) -> &ACTIV {
        &self.activ
    }
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    #[inline(always)]
    pub const fn stop(&self) -> &STOP {
        &self.stop
    }
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x68 - Clear GEN_CALL Interrupt Register"]
    #[inline(always)]
    pub const fn gc(&self) -> &GC {
        &self.gc
    }
    #[doc = "0x6c - Enable Register"]
    #[inline(always)]
    pub const fn enr(&self) -> &ENR {
        &self.enr
    }
    #[doc = "0x70 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x74 - Transmit FIFO Level Register"]
    #[inline(always)]
    pub const fn txflr(&self) -> &TXFLR {
        &self.txflr
    }
    #[doc = "0x78 - Receive FIFO Level Register"]
    #[inline(always)]
    pub const fn rxflr(&self) -> &RXFLR {
        &self.rxflr
    }
    #[doc = "0x7c - SDA Hold Time Register"]
    #[inline(always)]
    pub const fn hold(&self) -> &HOLD {
        &self.hold
    }
    #[doc = "0x88 - DMA Control Register"]
    #[inline(always)]
    pub const fn dma(&self) -> &DMA {
        &self.dma
    }
    #[doc = "0x94 - SDA Setup Time Register"]
    #[inline(always)]
    pub const fn setup(&self) -> &SETUP {
        &self.setup
    }
    #[doc = "0x98 - ACK General Call Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0xb0 - Slave Address Mask Register"]
    #[inline(always)]
    pub const fn slvmask(&self) -> &SLVMASK {
        &self.slvmask
    }
    #[doc = "0xb4 - Receiver Address Register"]
    #[inline(always)]
    pub const fn slvrcvaddr(&self) -> &SLVRCVADDR {
        &self.slvrcvaddr
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "TAR (rw) register accessor: Target Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "Target Register"]
pub mod tar;
#[doc = "SAR (rw) register accessor: Slave Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "Slave Address Register"]
pub mod sar;
#[doc = "DR (rw) register accessor: Data Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data Command Register"]
pub mod dr;
#[doc = "SSHR (rw) register accessor: SCL High Period Count for Std. Speed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sshr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sshr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sshr`]
module"]
pub type SSHR = crate::Reg<sshr::SSHR_SPEC>;
#[doc = "SCL High Period Count for Std. Speed Register"]
pub mod sshr;
#[doc = "SSLR (rw) register accessor: SCL Low Period Count for Std. Speed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sslr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sslr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sslr`]
module"]
pub type SSLR = crate::Reg<sslr::SSLR_SPEC>;
#[doc = "SCL Low Period Count for Std. Speed Register"]
pub mod sslr;
#[doc = "FSHR (rw) register accessor: SCL High Period Count for Fast Speed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fshr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fshr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fshr`]
module"]
pub type FSHR = crate::Reg<fshr::FSHR_SPEC>;
#[doc = "SCL High Period Count for Fast Speed Register"]
pub mod fshr;
#[doc = "FSLR (rw) register accessor: SCL Low Period Count for Fast Speed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fslr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fslr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fslr`]
module"]
pub type FSLR = crate::Reg<fslr::FSLR_SPEC>;
#[doc = "SCL Low Period Count for Fast Speed Register"]
pub mod fslr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IMR (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "RAWISR (r) register accessor: RAW Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawisr`]
module"]
pub type RAWISR = crate::Reg<rawisr::RAWISR_SPEC>;
#[doc = "RAW Interrupt Status Register"]
pub mod rawisr;
#[doc = "RXTLR (rw) register accessor: Receive FIFO Threshold Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtlr`]
module"]
pub type RXTLR = crate::Reg<rxtlr::RXTLR_SPEC>;
#[doc = "Receive FIFO Threshold Level Register"]
pub mod rxtlr;
#[doc = "TXTLR (rw) register accessor: Transmit FIFO Threshold Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtlr`]
module"]
pub type TXTLR = crate::Reg<txtlr::TXTLR_SPEC>;
#[doc = "Transmit FIFO Threshold Level Register"]
pub mod txtlr;
#[doc = "ICR (r) register accessor: Clear All Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Clear All Interrupt Register"]
pub mod icr;
#[doc = "RX_UNDER (r) register accessor: Clear RX_UNDER Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_under::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_under`]
module"]
pub type RX_UNDER = crate::Reg<rx_under::RX_UNDER_SPEC>;
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod rx_under;
#[doc = "RX_OVER (r) register accessor: Clear RX_OVER Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_over::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_over`]
module"]
pub type RX_OVER = crate::Reg<rx_over::RX_OVER_SPEC>;
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod rx_over;
#[doc = "TX_OVER (r) register accessor: Clear TX_OVER Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_over::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_over`]
module"]
pub type TX_OVER = crate::Reg<tx_over::TX_OVER_SPEC>;
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod tx_over;
#[doc = "RD_REQ (r) register accessor: Clear RD_REQ Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_req::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_req`]
module"]
pub type RD_REQ = crate::Reg<rd_req::RD_REQ_SPEC>;
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod rd_req;
#[doc = "TX_ABRT (r) register accessor: Clear TX_ABRT Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_abrt::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_abrt`]
module"]
pub type TX_ABRT = crate::Reg<tx_abrt::TX_ABRT_SPEC>;
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod tx_abrt;
#[doc = "RX_DONE (r) register accessor: Clear RX_DONE Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_done::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_done`]
module"]
pub type RX_DONE = crate::Reg<rx_done::RX_DONE_SPEC>;
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod rx_done;
#[doc = "ACTIV (r) register accessor: Clear ACTIVITY Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`activ::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@activ`]
module"]
pub type ACTIV = crate::Reg<activ::ACTIV_SPEC>;
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod activ;
#[doc = "STOP (r) register accessor: Clear STOP_DET Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stop::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
pub type STOP = crate::Reg<stop::STOP_SPEC>;
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod stop;
#[doc = "START (r) register accessor: Clear START_DET Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Clear START_DET Interrupt Register"]
pub mod start;
#[doc = "GC (r) register accessor: Clear GEN_CALL Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gc::R`]. WARN: The register is **cleared** (set to zero) following a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gc`]
module"]
pub type GC = crate::Reg<gc::GC_SPEC>;
#[doc = "Clear GEN_CALL Interrupt Register"]
pub mod gc;
#[doc = "ENR (rw) register accessor: Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enr`]
module"]
pub type ENR = crate::Reg<enr::ENR_SPEC>;
#[doc = "Enable Register"]
pub mod enr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TXFLR (rw) register accessor: Transmit FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txflr`]
module"]
pub type TXFLR = crate::Reg<txflr::TXFLR_SPEC>;
#[doc = "Transmit FIFO Level Register"]
pub mod txflr;
#[doc = "RXFLR (rw) register accessor: Receive FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxflr`]
module"]
pub type RXFLR = crate::Reg<rxflr::RXFLR_SPEC>;
#[doc = "Receive FIFO Level Register"]
pub mod rxflr;
#[doc = "HOLD (rw) register accessor: SDA Hold Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hold`]
module"]
pub type HOLD = crate::Reg<hold::HOLD_SPEC>;
#[doc = "SDA Hold Time Register"]
pub mod hold;
#[doc = "DMA (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Control Register"]
pub mod dma;
#[doc = "SETUP (rw) register accessor: SDA Setup Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup`]
module"]
pub type SETUP = crate::Reg<setup::SETUP_SPEC>;
#[doc = "SDA Setup Time Register"]
pub mod setup;
#[doc = "GCR (rw) register accessor: ACK General Call Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "ACK General Call Register"]
pub mod gcr;
#[doc = "SLVMASK (rw) register accessor: Slave Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slvmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slvmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvmask`]
module"]
pub type SLVMASK = crate::Reg<slvmask::SLVMASK_SPEC>;
#[doc = "Slave Address Mask Register"]
pub mod slvmask;
#[doc = "SLVRCVADDR (r) register accessor: Receiver Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slvrcvaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvrcvaddr`]
module"]
pub type SLVRCVADDR = crate::Reg<slvrcvaddr::SLVRCVADDR_SPEC>;
#[doc = "Receiver Address Register"]
pub mod slvrcvaddr;
