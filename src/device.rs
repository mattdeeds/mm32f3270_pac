#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    uid1: UID1,
    uid2: UID2,
    uid3: UID3,
    uid4: UID4,
}
impl RegisterBlock {
    #[doc = "0x00 - Unique product indentification register 1"]
    #[inline(always)]
    pub const fn uid1(&self) -> &UID1 {
        &self.uid1
    }
    #[doc = "0x02 - Unique product indentification register 2"]
    #[inline(always)]
    pub const fn uid2(&self) -> &UID2 {
        &self.uid2
    }
    #[doc = "0x04 - Unique product indentification register 3"]
    #[inline(always)]
    pub const fn uid3(&self) -> &UID3 {
        &self.uid3
    }
    #[doc = "0x08 - Unique product indentification register 4"]
    #[inline(always)]
    pub const fn uid4(&self) -> &UID4 {
        &self.uid4
    }
}
#[doc = "UID1 (r) register accessor: Unique product indentification register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uid1`]
module"]
pub type UID1 = crate::Reg<uid1::UID1_SPEC>;
#[doc = "Unique product indentification register 1"]
pub mod uid1;
#[doc = "UID2 (r) register accessor: Unique product indentification register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uid2`]
module"]
pub type UID2 = crate::Reg<uid2::UID2_SPEC>;
#[doc = "Unique product indentification register 2"]
pub mod uid2;
#[doc = "UID3 (r) register accessor: Unique product indentification register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uid3`]
module"]
pub type UID3 = crate::Reg<uid3::UID3_SPEC>;
#[doc = "Unique product indentification register 3"]
pub mod uid3;
#[doc = "UID4 (r) register accessor: Unique product indentification register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uid4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uid4`]
module"]
pub type UID4 = crate::Reg<uid4::UID4_SPEC>;
#[doc = "Unique product indentification register 4"]
pub mod uid4;
