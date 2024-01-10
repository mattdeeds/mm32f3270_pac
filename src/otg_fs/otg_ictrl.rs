#[doc = "Register `OTG_ICTRL` reader"]
pub type R = crate::R<OTG_ICTRL_SPEC>;
#[doc = "Register `OTG_ICTRL` writer"]
pub type W = crate::W<OTG_ICTRL_SPEC>;
#[doc = "Field `A_VBUS_VLD_EN` reader - Enable a device VBUS valid interrupt"]
pub type A_VBUS_VLD_EN_R = crate::BitReader;
#[doc = "Field `A_VBUS_VLD_EN` writer - Enable a device VBUS valid interrupt"]
pub type A_VBUS_VLD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_SESS_END_EN` reader - Enable b device VBUS valid interrupt"]
pub type B_SESS_END_EN_R = crate::BitReader;
#[doc = "Field `B_SESS_END_EN` writer - Enable b device VBUS valid interrupt"]
pub type B_SESS_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESS_VLD_EN` reader - Enables the session to be effectively interrupted"]
pub type SESS_VLD_EN_R = crate::BitReader;
#[doc = "Field `SESS_VLD_EN` writer - Enables the session to be effectively interrupted"]
pub type SESS_VLD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_STATE_EN` reader - Enable line state change interrupt."]
pub type LINE_STATE_EN_R = crate::BitReader;
#[doc = "Field `LINE_STATE_EN` writer - Enable line state change interrupt."]
pub type LINE_STATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_1MSEC_EN` reader - Enable 1 ms timer interrupt."]
pub type _1MSEC_EN_R = crate::BitReader;
#[doc = "Field `_1MSEC_EN` writer - Enable 1 ms timer interrupt."]
pub type _1MSEC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_EN` reader - Enable ID signal interrupt"]
pub type ID_EN_R = crate::BitReader;
#[doc = "Field `ID_EN` writer - Enable ID signal interrupt"]
pub type ID_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable a device VBUS valid interrupt"]
    #[inline(always)]
    pub fn a_vbus_vld_en(&self) -> A_VBUS_VLD_EN_R {
        A_VBUS_VLD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable b device VBUS valid interrupt"]
    #[inline(always)]
    pub fn b_sess_end_en(&self) -> B_SESS_END_EN_R {
        B_SESS_END_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the session to be effectively interrupted"]
    #[inline(always)]
    pub fn sess_vld_en(&self) -> SESS_VLD_EN_R {
        SESS_VLD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable line state change interrupt."]
    #[inline(always)]
    pub fn line_state_en(&self) -> LINE_STATE_EN_R {
        LINE_STATE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable 1 ms timer interrupt."]
    #[inline(always)]
    pub fn _1msec_en(&self) -> _1MSEC_EN_R {
        _1MSEC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable ID signal interrupt"]
    #[inline(always)]
    pub fn id_en(&self) -> ID_EN_R {
        ID_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable a device VBUS valid interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn a_vbus_vld_en(&mut self) -> A_VBUS_VLD_EN_W<OTG_ICTRL_SPEC> {
        A_VBUS_VLD_EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Enable b device VBUS valid interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn b_sess_end_en(&mut self) -> B_SESS_END_EN_W<OTG_ICTRL_SPEC> {
        B_SESS_END_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the session to be effectively interrupted"]
    #[inline(always)]
    #[must_use]
    pub fn sess_vld_en(&mut self) -> SESS_VLD_EN_W<OTG_ICTRL_SPEC> {
        SESS_VLD_EN_W::new(self, 3)
    }
    #[doc = "Bit 5 - Enable line state change interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn line_state_en(&mut self) -> LINE_STATE_EN_W<OTG_ICTRL_SPEC> {
        LINE_STATE_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable 1 ms timer interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn _1msec_en(&mut self) -> _1MSEC_EN_W<OTG_ICTRL_SPEC> {
        _1MSEC_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable ID signal interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn id_en(&mut self) -> ID_EN_W<OTG_ICTRL_SPEC> {
        ID_EN_W::new(self, 7)
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
#[doc = "OTG Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_ictrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_ictrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_ICTRL_SPEC;
impl crate::RegisterSpec for OTG_ICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_ictrl::R`](R) reader structure"]
impl crate::Readable for OTG_ICTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_ictrl::W`](W) writer structure"]
impl crate::Writable for OTG_ICTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_ICTRL to value 0"]
impl crate::Resettable for OTG_ICTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
