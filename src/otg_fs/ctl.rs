#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `USB_EN_SOF_EN` reader - Setting this bit will make otg_fs work, clearing it will disable otg_fs"]
pub type USB_EN_SOF_EN_R = crate::BitReader;
#[doc = "Field `USB_EN_SOF_EN` writer - Setting this bit will make otg_fs work, clearing it will disable otg_fs"]
pub type USB_EN_SOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODD_RST` reader - Setting this bit will reset all BDT odd Ping_Pong bits to 0 and then specify an even number of BDT libraries"]
pub type ODD_RST_R = crate::BitReader;
#[doc = "Field `ODD_RST` writer - Setting this bit will reset all BDT odd Ping_Pong bits to 0 and then specify an even number of BDT libraries"]
pub type ODD_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - Setting this bit will allow the otg_fs to perform the recovery signal"]
pub type RESUME_R = crate::BitReader;
#[doc = "Field `RESUME` writer - Setting this bit will allow the otg_fs to perform the recovery signal"]
pub type RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_MODE_EN` reader - Setting this bit causes the otg_fs to work in host mode"]
pub type HOST_MODE_EN_R = crate::BitReader;
#[doc = "Field `HOST_MODE_EN` writer - Setting this bit causes the otg_fs to work in host mode"]
pub type HOST_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Setting this bit will cause otg_fs to generate USB reset signal"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Setting this bit will cause otg_fs to generate USB reset signal"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDSUSPEND_TOKENBUSY` reader - When otg_fs is in slave mode,it is TXD_ Suspend token when otg_fs is in host mode_ Busy"]
pub type TXDSUSPEND_TOKENBUSY_R = crate::BitReader;
#[doc = "Field `TXDSUSPEND_TOKENBUSY` writer - When otg_fs is in slave mode,it is TXD_ Suspend token when otg_fs is in host mode_ Busy"]
pub type TXDSUSPEND_TOKENBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE0` reader - SE0 signal received by USB differential receiver"]
pub type SE0_R = crate::BitReader;
#[doc = "Field `SE0` writer - SE0 signal received by USB differential receiver"]
pub type SE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSTATE` reader - USB differential receiver receives jstate signal"]
pub type JSTATE_R = crate::BitReader;
#[doc = "Field `JSTATE` writer - USB differential receiver receives jstate signal"]
pub type JSTATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting this bit will make otg_fs work, clearing it will disable otg_fs"]
    #[inline(always)]
    pub fn usb_en_sof_en(&self) -> USB_EN_SOF_EN_R {
        USB_EN_SOF_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit will reset all BDT odd Ping_Pong bits to 0 and then specify an even number of BDT libraries"]
    #[inline(always)]
    pub fn odd_rst(&self) -> ODD_RST_R {
        ODD_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit will allow the otg_fs to perform the recovery signal"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting this bit causes the otg_fs to work in host mode"]
    #[inline(always)]
    pub fn host_mode_en(&self) -> HOST_MODE_EN_R {
        HOST_MODE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit will cause otg_fs to generate USB reset signal"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When otg_fs is in slave mode,it is TXD_ Suspend token when otg_fs is in host mode_ Busy"]
    #[inline(always)]
    pub fn txdsuspend_tokenbusy(&self) -> TXDSUSPEND_TOKENBUSY_R {
        TXDSUSPEND_TOKENBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SE0 signal received by USB differential receiver"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB differential receiver receives jstate signal"]
    #[inline(always)]
    pub fn jstate(&self) -> JSTATE_R {
        JSTATE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit will make otg_fs work, clearing it will disable otg_fs"]
    #[inline(always)]
    #[must_use]
    pub fn usb_en_sof_en(&mut self) -> USB_EN_SOF_EN_W<CTL_SPEC> {
        USB_EN_SOF_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Setting this bit will reset all BDT odd Ping_Pong bits to 0 and then specify an even number of BDT libraries"]
    #[inline(always)]
    #[must_use]
    pub fn odd_rst(&mut self) -> ODD_RST_W<CTL_SPEC> {
        ODD_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Setting this bit will allow the otg_fs to perform the recovery signal"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<CTL_SPEC> {
        RESUME_W::new(self, 2)
    }
    #[doc = "Bit 3 - Setting this bit causes the otg_fs to work in host mode"]
    #[inline(always)]
    #[must_use]
    pub fn host_mode_en(&mut self) -> HOST_MODE_EN_W<CTL_SPEC> {
        HOST_MODE_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Setting this bit will cause otg_fs to generate USB reset signal"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CTL_SPEC> {
        RESET_W::new(self, 4)
    }
    #[doc = "Bit 5 - When otg_fs is in slave mode,it is TXD_ Suspend token when otg_fs is in host mode_ Busy"]
    #[inline(always)]
    #[must_use]
    pub fn txdsuspend_tokenbusy(&mut self) -> TXDSUSPEND_TOKENBUSY_W<CTL_SPEC> {
        TXDSUSPEND_TOKENBUSY_W::new(self, 5)
    }
    #[doc = "Bit 6 - SE0 signal received by USB differential receiver"]
    #[inline(always)]
    #[must_use]
    pub fn se0(&mut self) -> SE0_W<CTL_SPEC> {
        SE0_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB differential receiver receives jstate signal"]
    #[inline(always)]
    #[must_use]
    pub fn jstate(&mut self) -> JSTATE_W<CTL_SPEC> {
        JSTATE_W::new(self, 7)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x40"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
