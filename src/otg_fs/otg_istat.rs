#[doc = "Register `OTG_ISTAT` reader"]
pub type R = crate::R<OTG_ISTAT_SPEC>;
#[doc = "Register `OTG_ISTAT` writer"]
pub type W = crate::W<OTG_ISTAT_SPEC>;
#[doc = "Field `A_VBUS_VLD_CHG` reader - This position is 1 when a VBUS change is detected on the A device."]
pub type A_VBUS_VLD_CHG_R = crate::BitReader;
#[doc = "Field `A_VBUS_VLD_CHG` writer - This position is 1 when a VBUS change is detected on the A device."]
pub type A_VBUS_VLD_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `B_SESS_END_CHG` reader - This position bit when a VBUS change is detected on the B device."]
pub type B_SESS_END_CHG_R = crate::BitReader;
#[doc = "Field `B_SESS_END_CHG` writer - This position bit when a VBUS change is detected on the B device."]
pub type B_SESS_END_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SESS_VLD_CHG` reader - When a VBUS change is detected, the position bit indicates that the session is valid or the session is no longer valid."]
pub type SESS_VLD_CHG_R = crate::BitReader;
#[doc = "Field `SESS_VLD_CHG` writer - When a VBUS change is detected, the position bit indicates that the session is valid or the session is no longer valid."]
pub type SESS_VLD_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LINE_STATE_CHG` reader - The interrupt can be used to detect reset and recover."]
pub type LINE_STATE_CHG_R = crate::BitReader;
#[doc = "Field `LINE_STATE_CHG` writer - The interrupt can be used to detect reset and recover."]
pub type LINE_STATE_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_1MSEC` reader - This bit is set when the 1 ms timer expires."]
pub type _1MSEC_R = crate::BitReader;
#[doc = "Field `_1MSEC` writer - This bit is set when the 1 ms timer expires."]
pub type _1MSEC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ID_CHG` reader - When the USB ID pin signal is detected to be changed, the position bit."]
pub type ID_CHG_R = crate::BitReader;
#[doc = "Field `ID_CHG` writer - When the USB ID pin signal is detected to be changed, the position bit."]
pub type ID_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - This position is 1 when a VBUS change is detected on the A device."]
    #[inline(always)]
    pub fn a_vbus_vld_chg(&self) -> A_VBUS_VLD_CHG_R {
        A_VBUS_VLD_CHG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - This position bit when a VBUS change is detected on the B device."]
    #[inline(always)]
    pub fn b_sess_end_chg(&self) -> B_SESS_END_CHG_R {
        B_SESS_END_CHG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When a VBUS change is detected, the position bit indicates that the session is valid or the session is no longer valid."]
    #[inline(always)]
    pub fn sess_vld_chg(&self) -> SESS_VLD_CHG_R {
        SESS_VLD_CHG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt can be used to detect reset and recover."]
    #[inline(always)]
    pub fn line_state_chg(&self) -> LINE_STATE_CHG_R {
        LINE_STATE_CHG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set when the 1 ms timer expires."]
    #[inline(always)]
    pub fn _1msec(&self) -> _1MSEC_R {
        _1MSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When the USB ID pin signal is detected to be changed, the position bit."]
    #[inline(always)]
    pub fn id_chg(&self) -> ID_CHG_R {
        ID_CHG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This position is 1 when a VBUS change is detected on the A device."]
    #[inline(always)]
    #[must_use]
    pub fn a_vbus_vld_chg(&mut self) -> A_VBUS_VLD_CHG_W<OTG_ISTAT_SPEC> {
        A_VBUS_VLD_CHG_W::new(self, 0)
    }
    #[doc = "Bit 2 - This position bit when a VBUS change is detected on the B device."]
    #[inline(always)]
    #[must_use]
    pub fn b_sess_end_chg(&mut self) -> B_SESS_END_CHG_W<OTG_ISTAT_SPEC> {
        B_SESS_END_CHG_W::new(self, 2)
    }
    #[doc = "Bit 3 - When a VBUS change is detected, the position bit indicates that the session is valid or the session is no longer valid."]
    #[inline(always)]
    #[must_use]
    pub fn sess_vld_chg(&mut self) -> SESS_VLD_CHG_W<OTG_ISTAT_SPEC> {
        SESS_VLD_CHG_W::new(self, 3)
    }
    #[doc = "Bit 5 - The interrupt can be used to detect reset and recover."]
    #[inline(always)]
    #[must_use]
    pub fn line_state_chg(&mut self) -> LINE_STATE_CHG_W<OTG_ISTAT_SPEC> {
        LINE_STATE_CHG_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is set when the 1 ms timer expires."]
    #[inline(always)]
    #[must_use]
    pub fn _1msec(&mut self) -> _1MSEC_W<OTG_ISTAT_SPEC> {
        _1MSEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - When the USB ID pin signal is detected to be changed, the position bit."]
    #[inline(always)]
    #[must_use]
    pub fn id_chg(&mut self) -> ID_CHG_W<OTG_ISTAT_SPEC> {
        ID_CHG_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_istat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_istat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_ISTAT_SPEC;
impl crate::RegisterSpec for OTG_ISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_istat::R`](R) reader structure"]
impl crate::Readable for OTG_ISTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_istat::W`](W) writer structure"]
impl crate::Writable for OTG_ISTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xed;
}
#[doc = "`reset()` method sets OTG_ISTAT to value 0xe8"]
impl crate::Resettable for OTG_ISTAT_SPEC {
    const RESET_VALUE: u32 = 0xe8;
}
