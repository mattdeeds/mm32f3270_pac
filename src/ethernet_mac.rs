#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    ffr: FFR,
    hthr: HTHR,
    htlr: HTLR,
    miiar: MIIAR,
    miidr: MIIDR,
    fcr: FCR,
    vlantr: VLANTR,
    _reserved8: [u8; 0x04],
    sr: SR,
    _reserved9: [u8; 0x18],
    maca0hr: MACA0HR,
    maca0lr: MACA0LR,
    maca1hr: MACA1HR,
    maca1lr: MACA1LR,
    maca2hr: MACA2HR,
    maca2lr: MACA2LR,
    maca3hr: MACA3HR,
    maca3lr: MACA3LR,
    maca4hr: MACA4HR,
    maca4lr: MACA4LR,
    maca5hr: MACA5HR,
    maca5lr: MACA5LR,
    maca6hr: MACA6HR,
    maca6lr: MACA6LR,
    maca7hr: MACA7HR,
    maca7lr: MACA7LR,
    maca8hr: MACA8HR,
    maca8lr: MACA8LR,
    maca9hr: MACA9HR,
    maca9lr: MACA9LR,
    maca10hr: MACA10HR,
    maca10lr: MACA10LR,
    maca11hr: MACA11HR,
    maca11lr: MACA11LR,
    maca12hr: MACA12HR,
    maca12lr: MACA12LR,
    maca13hr: MACA13HR,
    maca13lr: MACA13LR,
    maca14hr: MACA14HR,
    maca14lr: MACA14LR,
    maca15hr: MACA15HR,
    maca15lr: MACA15LR,
    _reserved41: [u8; 0x0740],
    maca16hr: MACA16HR,
    maca16lr: MACA16LR,
    maca17hr: MACA17HR,
    maca17lr: MACA17LR,
    maca18hr: MACA18HR,
    maca18lr: MACA18LR,
    maca19hr: MACA19HR,
    maca19lr: MACA19LR,
    maca20hr: MACA20HR,
    maca20lr: MACA20LR,
    maca21hr: MACA21HR,
    maca21lr: MACA21LR,
    maca22hr: MACA22HR,
    maca22lr: MACA22LR,
    maca23hr: MACA23HR,
    maca23lr: MACA23LR,
    maca24hr: MACA24HR,
    maca24lr: MACA24LR,
    maca25hr: MACA25HR,
    maca25lr: MACA25LR,
    maca26hr: MACA26HR,
    maca26lr: MACA26LR,
    maca27hr: MACA27HR,
    maca27lr: MACA27LR,
    maca28hr: MACA28HR,
    maca28lr: MACA28LR,
    maca29hr: MACA29HR,
    maca29lr: MACA29LR,
    maca30hr: MACA30HR,
    maca30lr: MACA30LR,
    maca31hr: MACA31HR,
    maca31lr: MACA31LR,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Ethernet MAC frame filter register"]
    #[inline(always)]
    pub const fn ffr(&self) -> &FFR {
        &self.ffr
    }
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    #[inline(always)]
    pub const fn hthr(&self) -> &HTHR {
        &self.hthr
    }
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    #[inline(always)]
    pub const fn htlr(&self) -> &HTLR {
        &self.htlr
    }
    #[doc = "0x10 - Ethernet MAC MII address register"]
    #[inline(always)]
    pub const fn miiar(&self) -> &MIIAR {
        &self.miiar
    }
    #[doc = "0x14 - Ethernet MAC MII data register"]
    #[inline(always)]
    pub const fn miidr(&self) -> &MIIDR {
        &self.miidr
    }
    #[doc = "0x18 - Ethernet MAC flow control register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x1c - Ethernet MAC VLAN tag register"]
    #[inline(always)]
    pub const fn vlantr(&self) -> &VLANTR {
        &self.vlantr
    }
    #[doc = "0x24 - Ethernet MAC VLAN tag register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x40 - Ethernet MAC address 0 high register"]
    #[inline(always)]
    pub const fn maca0hr(&self) -> &MACA0HR {
        &self.maca0hr
    }
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    #[inline(always)]
    pub const fn maca0lr(&self) -> &MACA0LR {
        &self.maca0lr
    }
    #[doc = "0x48 - Ethernet MAC address 1 high register"]
    #[inline(always)]
    pub const fn maca1hr(&self) -> &MACA1HR {
        &self.maca1hr
    }
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    #[inline(always)]
    pub const fn maca1lr(&self) -> &MACA1LR {
        &self.maca1lr
    }
    #[doc = "0x50 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    pub const fn maca2hr(&self) -> &MACA2HR {
        &self.maca2hr
    }
    #[doc = "0x54 - Ethernet MAC address1 low register"]
    #[inline(always)]
    pub const fn maca2lr(&self) -> &MACA2LR {
        &self.maca2lr
    }
    #[doc = "0x58 - Ethernet MAC address 3 high register"]
    #[inline(always)]
    pub const fn maca3hr(&self) -> &MACA3HR {
        &self.maca3hr
    }
    #[doc = "0x5c - Ethernet MAC address3 low register"]
    #[inline(always)]
    pub const fn maca3lr(&self) -> &MACA3LR {
        &self.maca3lr
    }
    #[doc = "0x60 - Ethernet MAC address 4 high register"]
    #[inline(always)]
    pub const fn maca4hr(&self) -> &MACA4HR {
        &self.maca4hr
    }
    #[doc = "0x64 - Ethernet MAC address4 low register"]
    #[inline(always)]
    pub const fn maca4lr(&self) -> &MACA4LR {
        &self.maca4lr
    }
    #[doc = "0x68 - Ethernet MAC address 5 high register"]
    #[inline(always)]
    pub const fn maca5hr(&self) -> &MACA5HR {
        &self.maca5hr
    }
    #[doc = "0x6c - Ethernet MAC address5 low register"]
    #[inline(always)]
    pub const fn maca5lr(&self) -> &MACA5LR {
        &self.maca5lr
    }
    #[doc = "0x70 - Ethernet MAC address 6 high register"]
    #[inline(always)]
    pub const fn maca6hr(&self) -> &MACA6HR {
        &self.maca6hr
    }
    #[doc = "0x74 - Ethernet MAC address6 low register"]
    #[inline(always)]
    pub const fn maca6lr(&self) -> &MACA6LR {
        &self.maca6lr
    }
    #[doc = "0x78 - Ethernet MAC address 7 high register"]
    #[inline(always)]
    pub const fn maca7hr(&self) -> &MACA7HR {
        &self.maca7hr
    }
    #[doc = "0x7c - Ethernet MAC address7 low register"]
    #[inline(always)]
    pub const fn maca7lr(&self) -> &MACA7LR {
        &self.maca7lr
    }
    #[doc = "0x80 - Ethernet MAC address 8 high register"]
    #[inline(always)]
    pub const fn maca8hr(&self) -> &MACA8HR {
        &self.maca8hr
    }
    #[doc = "0x84 - Ethernet MAC address8 low register"]
    #[inline(always)]
    pub const fn maca8lr(&self) -> &MACA8LR {
        &self.maca8lr
    }
    #[doc = "0x88 - Ethernet MAC address 9 high register"]
    #[inline(always)]
    pub const fn maca9hr(&self) -> &MACA9HR {
        &self.maca9hr
    }
    #[doc = "0x8c - Ethernet MAC address9 low register"]
    #[inline(always)]
    pub const fn maca9lr(&self) -> &MACA9LR {
        &self.maca9lr
    }
    #[doc = "0x90 - Ethernet MAC address 10 high register"]
    #[inline(always)]
    pub const fn maca10hr(&self) -> &MACA10HR {
        &self.maca10hr
    }
    #[doc = "0x94 - Ethernet MAC address10 low register"]
    #[inline(always)]
    pub const fn maca10lr(&self) -> &MACA10LR {
        &self.maca10lr
    }
    #[doc = "0x98 - Ethernet MAC address 11 high register"]
    #[inline(always)]
    pub const fn maca11hr(&self) -> &MACA11HR {
        &self.maca11hr
    }
    #[doc = "0x9c - Ethernet MAC address11 low register"]
    #[inline(always)]
    pub const fn maca11lr(&self) -> &MACA11LR {
        &self.maca11lr
    }
    #[doc = "0xa0 - Ethernet MAC address 12 high register"]
    #[inline(always)]
    pub const fn maca12hr(&self) -> &MACA12HR {
        &self.maca12hr
    }
    #[doc = "0xa4 - Ethernet MAC address12 low register"]
    #[inline(always)]
    pub const fn maca12lr(&self) -> &MACA12LR {
        &self.maca12lr
    }
    #[doc = "0xa8 - Ethernet MAC address 13 high register"]
    #[inline(always)]
    pub const fn maca13hr(&self) -> &MACA13HR {
        &self.maca13hr
    }
    #[doc = "0xac - Ethernet MAC address13 low register"]
    #[inline(always)]
    pub const fn maca13lr(&self) -> &MACA13LR {
        &self.maca13lr
    }
    #[doc = "0xb0 - Ethernet MAC address 14 high register"]
    #[inline(always)]
    pub const fn maca14hr(&self) -> &MACA14HR {
        &self.maca14hr
    }
    #[doc = "0xb4 - Ethernet MAC address14 low register"]
    #[inline(always)]
    pub const fn maca14lr(&self) -> &MACA14LR {
        &self.maca14lr
    }
    #[doc = "0xb8 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca15hr(&self) -> &MACA15HR {
        &self.maca15hr
    }
    #[doc = "0xbc - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca15lr(&self) -> &MACA15LR {
        &self.maca15lr
    }
    #[doc = "0x800 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca16hr(&self) -> &MACA16HR {
        &self.maca16hr
    }
    #[doc = "0x804 - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca16lr(&self) -> &MACA16LR {
        &self.maca16lr
    }
    #[doc = "0x808 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca17hr(&self) -> &MACA17HR {
        &self.maca17hr
    }
    #[doc = "0x80c - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca17lr(&self) -> &MACA17LR {
        &self.maca17lr
    }
    #[doc = "0x810 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca18hr(&self) -> &MACA18HR {
        &self.maca18hr
    }
    #[doc = "0x814 - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca18lr(&self) -> &MACA18LR {
        &self.maca18lr
    }
    #[doc = "0x818 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca19hr(&self) -> &MACA19HR {
        &self.maca19hr
    }
    #[doc = "0x81c - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca19lr(&self) -> &MACA19LR {
        &self.maca19lr
    }
    #[doc = "0x820 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca20hr(&self) -> &MACA20HR {
        &self.maca20hr
    }
    #[doc = "0x824 - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca20lr(&self) -> &MACA20LR {
        &self.maca20lr
    }
    #[doc = "0x828 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca21hr(&self) -> &MACA21HR {
        &self.maca21hr
    }
    #[doc = "0x82c - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca21lr(&self) -> &MACA21LR {
        &self.maca21lr
    }
    #[doc = "0x830 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca22hr(&self) -> &MACA22HR {
        &self.maca22hr
    }
    #[doc = "0x834 - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca22lr(&self) -> &MACA22LR {
        &self.maca22lr
    }
    #[doc = "0x838 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca23hr(&self) -> &MACA23HR {
        &self.maca23hr
    }
    #[doc = "0x83c - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca23lr(&self) -> &MACA23LR {
        &self.maca23lr
    }
    #[doc = "0x840 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca24hr(&self) -> &MACA24HR {
        &self.maca24hr
    }
    #[doc = "0x844 - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca24lr(&self) -> &MACA24LR {
        &self.maca24lr
    }
    #[doc = "0x848 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca25hr(&self) -> &MACA25HR {
        &self.maca25hr
    }
    #[doc = "0x84c - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca25lr(&self) -> &MACA25LR {
        &self.maca25lr
    }
    #[doc = "0x850 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca26hr(&self) -> &MACA26HR {
        &self.maca26hr
    }
    #[doc = "0x854 - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca26lr(&self) -> &MACA26LR {
        &self.maca26lr
    }
    #[doc = "0x858 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca27hr(&self) -> &MACA27HR {
        &self.maca27hr
    }
    #[doc = "0x85c - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca27lr(&self) -> &MACA27LR {
        &self.maca27lr
    }
    #[doc = "0x860 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca28hr(&self) -> &MACA28HR {
        &self.maca28hr
    }
    #[doc = "0x864 - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca28lr(&self) -> &MACA28LR {
        &self.maca28lr
    }
    #[doc = "0x868 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca29hr(&self) -> &MACA29HR {
        &self.maca29hr
    }
    #[doc = "0x86c - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca29lr(&self) -> &MACA29LR {
        &self.maca29lr
    }
    #[doc = "0x870 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca30hr(&self) -> &MACA30HR {
        &self.maca30hr
    }
    #[doc = "0x874 - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca30lr(&self) -> &MACA30LR {
        &self.maca30lr
    }
    #[doc = "0x878 - Ethernet MAC address 15 high register"]
    #[inline(always)]
    pub const fn maca31hr(&self) -> &MACA31HR {
        &self.maca31hr
    }
    #[doc = "0x87c - Ethernet MAC address15 low register"]
    #[inline(always)]
    pub const fn maca31lr(&self) -> &MACA31LR {
        &self.maca31lr
    }
}
#[doc = "CR (rw) register accessor: Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Ethernet MAC configuration register"]
pub mod cr;
#[doc = "FFR (rw) register accessor: Ethernet MAC frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffr`]
module"]
pub type FFR = crate::Reg<ffr::FFR_SPEC>;
#[doc = "Ethernet MAC frame filter register"]
pub mod ffr;
#[doc = "HTHR (rw) register accessor: Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hthr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hthr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hthr`]
module"]
pub type HTHR = crate::Reg<hthr::HTHR_SPEC>;
#[doc = "Ethernet MAC hash table high register"]
pub mod hthr;
#[doc = "HTLR (rw) register accessor: Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htlr`]
module"]
pub type HTLR = crate::Reg<htlr::HTLR_SPEC>;
#[doc = "Ethernet MAC hash table low register"]
pub mod htlr;
#[doc = "MIIAR (rw) register accessor: Ethernet MAC MII address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miiar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miiar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miiar`]
module"]
pub type MIIAR = crate::Reg<miiar::MIIAR_SPEC>;
#[doc = "Ethernet MAC MII address register"]
pub mod miiar;
#[doc = "MIIDR (rw) register accessor: Ethernet MAC MII data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miidr`]
module"]
pub type MIIDR = crate::Reg<miidr::MIIDR_SPEC>;
#[doc = "Ethernet MAC MII data register"]
pub mod miidr;
#[doc = "FCR (rw) register accessor: Ethernet MAC flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "Ethernet MAC flow control register"]
pub mod fcr;
#[doc = "VLANTR (rw) register accessor: Ethernet MAC VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlantr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlantr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlantr`]
module"]
pub type VLANTR = crate::Reg<vlantr::VLANTR_SPEC>;
#[doc = "Ethernet MAC VLAN tag register"]
pub mod vlantr;
#[doc = "SR (rw) register accessor: Ethernet MAC VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Ethernet MAC VLAN tag register"]
pub mod sr;
#[doc = "MACA0HR (rw) register accessor: Ethernet MAC address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0hr`]
module"]
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0hr;
#[doc = "MACA0LR (rw) register accessor: Ethernet MAC address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0lr`]
module"]
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1HR (rw) register accessor: Ethernet MAC address 1 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1hr`]
module"]
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1hr;
#[doc = "MACA1LR (rw) register accessor: Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1lr`]
module"]
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "MACA2HR (rw) register accessor: Ethernet MAC address 2 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2hr`]
module"]
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2hr;
#[doc = "MACA2LR (rw) register accessor: Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2lr`]
module"]
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca2lr;
#[doc = "MACA3HR (rw) register accessor: Ethernet MAC address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3hr`]
module"]
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3hr;
#[doc = "MACA3LR (rw) register accessor: Ethernet MAC address3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3lr`]
module"]
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
#[doc = "Ethernet MAC address3 low register"]
pub mod maca3lr;
#[doc = "MACA4HR (rw) register accessor: Ethernet MAC address 4 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca4hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca4hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca4hr`]
module"]
pub type MACA4HR = crate::Reg<maca4hr::MACA4HR_SPEC>;
#[doc = "Ethernet MAC address 4 high register"]
pub mod maca4hr;
#[doc = "MACA4LR (rw) register accessor: Ethernet MAC address4 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca4lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca4lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca4lr`]
module"]
pub type MACA4LR = crate::Reg<maca4lr::MACA4LR_SPEC>;
#[doc = "Ethernet MAC address4 low register"]
pub mod maca4lr;
#[doc = "MACA5HR (rw) register accessor: Ethernet MAC address 5 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca5hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca5hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca5hr`]
module"]
pub type MACA5HR = crate::Reg<maca5hr::MACA5HR_SPEC>;
#[doc = "Ethernet MAC address 5 high register"]
pub mod maca5hr;
#[doc = "MACA5LR (rw) register accessor: Ethernet MAC address5 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca5lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca5lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca5lr`]
module"]
pub type MACA5LR = crate::Reg<maca5lr::MACA5LR_SPEC>;
#[doc = "Ethernet MAC address5 low register"]
pub mod maca5lr;
#[doc = "MACA6HR (rw) register accessor: Ethernet MAC address 6 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca6hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca6hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca6hr`]
module"]
pub type MACA6HR = crate::Reg<maca6hr::MACA6HR_SPEC>;
#[doc = "Ethernet MAC address 6 high register"]
pub mod maca6hr;
#[doc = "MACA6LR (rw) register accessor: Ethernet MAC address6 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca6lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca6lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca6lr`]
module"]
pub type MACA6LR = crate::Reg<maca6lr::MACA6LR_SPEC>;
#[doc = "Ethernet MAC address6 low register"]
pub mod maca6lr;
#[doc = "MACA7HR (rw) register accessor: Ethernet MAC address 7 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca7hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca7hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca7hr`]
module"]
pub type MACA7HR = crate::Reg<maca7hr::MACA7HR_SPEC>;
#[doc = "Ethernet MAC address 7 high register"]
pub mod maca7hr;
#[doc = "MACA7LR (rw) register accessor: Ethernet MAC address7 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca7lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca7lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca7lr`]
module"]
pub type MACA7LR = crate::Reg<maca7lr::MACA7LR_SPEC>;
#[doc = "Ethernet MAC address7 low register"]
pub mod maca7lr;
#[doc = "MACA8HR (rw) register accessor: Ethernet MAC address 8 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca8hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca8hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca8hr`]
module"]
pub type MACA8HR = crate::Reg<maca8hr::MACA8HR_SPEC>;
#[doc = "Ethernet MAC address 8 high register"]
pub mod maca8hr;
#[doc = "MACA8LR (rw) register accessor: Ethernet MAC address8 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca8lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca8lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca8lr`]
module"]
pub type MACA8LR = crate::Reg<maca8lr::MACA8LR_SPEC>;
#[doc = "Ethernet MAC address8 low register"]
pub mod maca8lr;
#[doc = "MACA9HR (rw) register accessor: Ethernet MAC address 9 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca9hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca9hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca9hr`]
module"]
pub type MACA9HR = crate::Reg<maca9hr::MACA9HR_SPEC>;
#[doc = "Ethernet MAC address 9 high register"]
pub mod maca9hr;
#[doc = "MACA9LR (rw) register accessor: Ethernet MAC address9 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca9lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca9lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca9lr`]
module"]
pub type MACA9LR = crate::Reg<maca9lr::MACA9LR_SPEC>;
#[doc = "Ethernet MAC address9 low register"]
pub mod maca9lr;
#[doc = "MACA10HR (rw) register accessor: Ethernet MAC address 10 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca10hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca10hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca10hr`]
module"]
pub type MACA10HR = crate::Reg<maca10hr::MACA10HR_SPEC>;
#[doc = "Ethernet MAC address 10 high register"]
pub mod maca10hr;
#[doc = "MACA10LR (rw) register accessor: Ethernet MAC address10 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca10lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca10lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca10lr`]
module"]
pub type MACA10LR = crate::Reg<maca10lr::MACA10LR_SPEC>;
#[doc = "Ethernet MAC address10 low register"]
pub mod maca10lr;
#[doc = "MACA11HR (rw) register accessor: Ethernet MAC address 11 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca11hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca11hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca11hr`]
module"]
pub type MACA11HR = crate::Reg<maca11hr::MACA11HR_SPEC>;
#[doc = "Ethernet MAC address 11 high register"]
pub mod maca11hr;
#[doc = "MACA11LR (rw) register accessor: Ethernet MAC address11 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca11lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca11lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca11lr`]
module"]
pub type MACA11LR = crate::Reg<maca11lr::MACA11LR_SPEC>;
#[doc = "Ethernet MAC address11 low register"]
pub mod maca11lr;
#[doc = "MACA12HR (rw) register accessor: Ethernet MAC address 12 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca12hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca12hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca12hr`]
module"]
pub type MACA12HR = crate::Reg<maca12hr::MACA12HR_SPEC>;
#[doc = "Ethernet MAC address 12 high register"]
pub mod maca12hr;
#[doc = "MACA12LR (rw) register accessor: Ethernet MAC address12 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca12lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca12lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca12lr`]
module"]
pub type MACA12LR = crate::Reg<maca12lr::MACA12LR_SPEC>;
#[doc = "Ethernet MAC address12 low register"]
pub mod maca12lr;
#[doc = "MACA13HR (rw) register accessor: Ethernet MAC address 13 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca13hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca13hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca13hr`]
module"]
pub type MACA13HR = crate::Reg<maca13hr::MACA13HR_SPEC>;
#[doc = "Ethernet MAC address 13 high register"]
pub mod maca13hr;
#[doc = "MACA13LR (rw) register accessor: Ethernet MAC address13 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca13lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca13lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca13lr`]
module"]
pub type MACA13LR = crate::Reg<maca13lr::MACA13LR_SPEC>;
#[doc = "Ethernet MAC address13 low register"]
pub mod maca13lr;
#[doc = "MACA14HR (rw) register accessor: Ethernet MAC address 14 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca14hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca14hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca14hr`]
module"]
pub type MACA14HR = crate::Reg<maca14hr::MACA14HR_SPEC>;
#[doc = "Ethernet MAC address 14 high register"]
pub mod maca14hr;
#[doc = "MACA14LR (rw) register accessor: Ethernet MAC address14 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca14lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca14lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca14lr`]
module"]
pub type MACA14LR = crate::Reg<maca14lr::MACA14LR_SPEC>;
#[doc = "Ethernet MAC address14 low register"]
pub mod maca14lr;
#[doc = "MACA15HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca15hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca15hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca15hr`]
module"]
pub type MACA15HR = crate::Reg<maca15hr::MACA15HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca15hr;
#[doc = "MACA15LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca15lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca15lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca15lr`]
module"]
pub type MACA15LR = crate::Reg<maca15lr::MACA15LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca15lr;
#[doc = "MACA16HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca16hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca16hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca16hr`]
module"]
pub type MACA16HR = crate::Reg<maca16hr::MACA16HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca16hr;
#[doc = "MACA16LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca16lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca16lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca16lr`]
module"]
pub type MACA16LR = crate::Reg<maca16lr::MACA16LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca16lr;
#[doc = "MACA17HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca17hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca17hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca17hr`]
module"]
pub type MACA17HR = crate::Reg<maca17hr::MACA17HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca17hr;
#[doc = "MACA17LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca17lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca17lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca17lr`]
module"]
pub type MACA17LR = crate::Reg<maca17lr::MACA17LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca17lr;
#[doc = "MACA18HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca18hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca18hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca18hr`]
module"]
pub type MACA18HR = crate::Reg<maca18hr::MACA18HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca18hr;
#[doc = "MACA18LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca18lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca18lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca18lr`]
module"]
pub type MACA18LR = crate::Reg<maca18lr::MACA18LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca18lr;
#[doc = "MACA19HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca19hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca19hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca19hr`]
module"]
pub type MACA19HR = crate::Reg<maca19hr::MACA19HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca19hr;
#[doc = "MACA19LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca19lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca19lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca19lr`]
module"]
pub type MACA19LR = crate::Reg<maca19lr::MACA19LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca19lr;
#[doc = "MACA20HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca20hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca20hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca20hr`]
module"]
pub type MACA20HR = crate::Reg<maca20hr::MACA20HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca20hr;
#[doc = "MACA20LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca20lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca20lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca20lr`]
module"]
pub type MACA20LR = crate::Reg<maca20lr::MACA20LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca20lr;
#[doc = "MACA21HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca21hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca21hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca21hr`]
module"]
pub type MACA21HR = crate::Reg<maca21hr::MACA21HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca21hr;
#[doc = "MACA21LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca21lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca21lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca21lr`]
module"]
pub type MACA21LR = crate::Reg<maca21lr::MACA21LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca21lr;
#[doc = "MACA22HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca22hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca22hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca22hr`]
module"]
pub type MACA22HR = crate::Reg<maca22hr::MACA22HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca22hr;
#[doc = "MACA22LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca22lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca22lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca22lr`]
module"]
pub type MACA22LR = crate::Reg<maca22lr::MACA22LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca22lr;
#[doc = "MACA23HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca23hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca23hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca23hr`]
module"]
pub type MACA23HR = crate::Reg<maca23hr::MACA23HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca23hr;
#[doc = "MACA23LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca23lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca23lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca23lr`]
module"]
pub type MACA23LR = crate::Reg<maca23lr::MACA23LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca23lr;
#[doc = "MACA24HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca24hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca24hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca24hr`]
module"]
pub type MACA24HR = crate::Reg<maca24hr::MACA24HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca24hr;
#[doc = "MACA24LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca24lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca24lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca24lr`]
module"]
pub type MACA24LR = crate::Reg<maca24lr::MACA24LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca24lr;
#[doc = "MACA25HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca25hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca25hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca25hr`]
module"]
pub type MACA25HR = crate::Reg<maca25hr::MACA25HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca25hr;
#[doc = "MACA25LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca25lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca25lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca25lr`]
module"]
pub type MACA25LR = crate::Reg<maca25lr::MACA25LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca25lr;
#[doc = "MACA26HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca26hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca26hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca26hr`]
module"]
pub type MACA26HR = crate::Reg<maca26hr::MACA26HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca26hr;
#[doc = "MACA26LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca26lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca26lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca26lr`]
module"]
pub type MACA26LR = crate::Reg<maca26lr::MACA26LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca26lr;
#[doc = "MACA27HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca27hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca27hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca27hr`]
module"]
pub type MACA27HR = crate::Reg<maca27hr::MACA27HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca27hr;
#[doc = "MACA27LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca27lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca27lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca27lr`]
module"]
pub type MACA27LR = crate::Reg<maca27lr::MACA27LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca27lr;
#[doc = "MACA28HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca28hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca28hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca28hr`]
module"]
pub type MACA28HR = crate::Reg<maca28hr::MACA28HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca28hr;
#[doc = "MACA28LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca28lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca28lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca28lr`]
module"]
pub type MACA28LR = crate::Reg<maca28lr::MACA28LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca28lr;
#[doc = "MACA29HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca29hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca29hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca29hr`]
module"]
pub type MACA29HR = crate::Reg<maca29hr::MACA29HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca29hr;
#[doc = "MACA29LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca29lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca29lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca29lr`]
module"]
pub type MACA29LR = crate::Reg<maca29lr::MACA29LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca29lr;
#[doc = "MACA30HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca30hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca30hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca30hr`]
module"]
pub type MACA30HR = crate::Reg<maca30hr::MACA30HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca30hr;
#[doc = "MACA30LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca30lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca30lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca30lr`]
module"]
pub type MACA30LR = crate::Reg<maca30lr::MACA30LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca30lr;
#[doc = "MACA31HR (rw) register accessor: Ethernet MAC address 15 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca31hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca31hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca31hr`]
module"]
pub type MACA31HR = crate::Reg<maca31hr::MACA31HR_SPEC>;
#[doc = "Ethernet MAC address 15 high register"]
pub mod maca31hr;
#[doc = "MACA31LR (rw) register accessor: Ethernet MAC address15 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca31lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca31lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca31lr`]
module"]
pub type MACA31LR = crate::Reg<maca31lr::MACA31LR_SPEC>;
#[doc = "Ethernet MAC address15 low register"]
pub mod maca31lr;
