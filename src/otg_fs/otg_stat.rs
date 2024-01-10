#[doc = "Register `OTG_STAT` reader"]
pub type R = crate::R<OTG_STAT_SPEC>;
#[doc = "Register `OTG_STAT` writer"]
pub type W = crate::W<OTG_STAT_SPEC>;
#[doc = "Field `A_VBUS_VLD` reader - When the VBUS voltage is higher than the VBUS effective threshold of a device, this bit will be set bit"]
pub type A_VBUS_VLD_R = crate::BitReader;
#[doc = "Field `A_VBUS_VLD` writer - When the VBUS voltage is higher than the VBUS effective threshold of a device, this bit will be set bit"]
pub type A_VBUS_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_SESS_END` reader - This bit is set when the VBUS voltage is below the B device session end threshold"]
pub type B_SESS_END_R = crate::BitReader;
#[doc = "Field `B_SESS_END` writer - This bit is set when the VBUS voltage is below the B device session end threshold"]
pub type B_SESS_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESS_VLD` reader - This bit is set when the VBUS voltage is higher than the valid threshold of the B device session bit"]
pub type SESS_VLD_R = crate::BitReader;
#[doc = "Field `SESS_VLD` writer - This bit is set when the VBUS voltage is higher than the valid threshold of the B device session bit"]
pub type SESS_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_STATE_STABLE` reader - Dithering"]
pub type LINE_STATE_STABLE_R = crate::BitReader;
#[doc = "Field `LINE_STATE_STABLE` writer - Dithering"]
pub type LINE_STATE_STABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_1MSEC` reader - This bit is reserved for the 1 ms counter and is not valid for software"]
pub type _1MSEC_R = crate::BitReader;
#[doc = "Field `_1MSEC` writer - This bit is reserved for the 1 ms counter and is not valid for software"]
pub type _1MSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID` reader - Indicates the current status of the ID pin on the USB connector"]
pub type ID_R = crate::BitReader;
#[doc = "Field `ID` writer - Indicates the current status of the ID pin on the USB connector"]
pub type ID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When the VBUS voltage is higher than the VBUS effective threshold of a device, this bit will be set bit"]
    #[inline(always)]
    pub fn a_vbus_vld(&self) -> A_VBUS_VLD_R {
        A_VBUS_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the VBUS voltage is below the B device session end threshold"]
    #[inline(always)]
    pub fn b_sess_end(&self) -> B_SESS_END_R {
        B_SESS_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the VBUS voltage is higher than the valid threshold of the B device session bit"]
    #[inline(always)]
    pub fn sess_vld(&self) -> SESS_VLD_R {
        SESS_VLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Dithering"]
    #[inline(always)]
    pub fn line_state_stable(&self) -> LINE_STATE_STABLE_R {
        LINE_STATE_STABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1 ms counter and is not valid for software"]
    #[inline(always)]
    pub fn _1msec(&self) -> _1MSEC_R {
        _1MSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the current status of the ID pin on the USB connector"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When the VBUS voltage is higher than the VBUS effective threshold of a device, this bit will be set bit"]
    #[inline(always)]
    #[must_use]
    pub fn a_vbus_vld(&mut self) -> A_VBUS_VLD_W<OTG_STAT_SPEC> {
        A_VBUS_VLD_W::new(self, 0)
    }
    #[doc = "Bit 2 - This bit is set when the VBUS voltage is below the B device session end threshold"]
    #[inline(always)]
    #[must_use]
    pub fn b_sess_end(&mut self) -> B_SESS_END_W<OTG_STAT_SPEC> {
        B_SESS_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is set when the VBUS voltage is higher than the valid threshold of the B device session bit"]
    #[inline(always)]
    #[must_use]
    pub fn sess_vld(&mut self) -> SESS_VLD_W<OTG_STAT_SPEC> {
        SESS_VLD_W::new(self, 3)
    }
    #[doc = "Bit 5 - Dithering"]
    #[inline(always)]
    #[must_use]
    pub fn line_state_stable(&mut self) -> LINE_STATE_STABLE_W<OTG_STAT_SPEC> {
        LINE_STATE_STABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1 ms counter and is not valid for software"]
    #[inline(always)]
    #[must_use]
    pub fn _1msec(&mut self) -> _1MSEC_W<OTG_STAT_SPEC> {
        _1MSEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates the current status of the ID pin on the USB connector"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<OTG_STAT_SPEC> {
        ID_W::new(self, 7)
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
#[doc = "OTG Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_STAT_SPEC;
impl crate::RegisterSpec for OTG_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_stat::R`](R) reader structure"]
impl crate::Readable for OTG_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_stat::W`](W) writer structure"]
impl crate::Writable for OTG_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_STAT to value 0xa8"]
impl crate::Resettable for OTG_STAT_SPEC {
    const RESET_VALUE: u32 = 0xa8;
}
