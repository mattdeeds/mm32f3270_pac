#[doc = "Register `INT_STAT` reader"]
pub type R = crate::R<INT_STAT_SPEC>;
#[doc = "Register `INT_STAT` writer"]
pub type W = crate::W<INT_STAT_SPEC>;
#[doc = "Field `USB_RST` reader - When otg_fs decodes valid USB reset, this position bit"]
pub type USB_RST_R = crate::BitReader;
#[doc = "Field `USB_RST` writer - When otg_fs decodes valid USB reset, this position bit"]
pub type USB_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - This position bit when any error condition in the ERR_STAT register occurs"]
pub type ERROR_R = crate::BitReader;
#[doc = "Field `ERROR` writer - This position bit when any error condition in the ERR_STAT register occurs"]
pub type ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_TOK` reader - If the otg_fs receives a sof token, this location bit"]
pub type SOF_TOK_R = crate::BitReader;
#[doc = "Field `SOF_TOK` writer - If the otg_fs receives a sof token, this location bit"]
pub type SOF_TOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOK_DNE` reader - This bit is set when the token being processed completes"]
pub type TOK_DNE_R = crate::BitReader;
#[doc = "Field `TOK_DNE` writer - This bit is set when the token being processed completes"]
pub type TOK_DNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - If otg_fs detects 3MS idle on the USB bus signal, then Position 1"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - If otg_fs detects 3MS idle on the USB bus signal, then Position 1"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - When the K state is observed on the DP_DM signal line within 2.5 microseconds, this position 1"]
pub type RESUME_R = crate::BitReader;
#[doc = "Field `RESUME` writer - When the K state is observed on the DP_DM signal line within 2.5 microseconds, this position 1"]
pub type RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTACH` reader - If OTG_FS detects a connection to a USB peripheral, the location bit"]
pub type ATTACH_R = crate::BitReader;
#[doc = "Field `ATTACH` writer - If OTG_FS detects a connection to a USB peripheral, the location bit"]
pub type ATTACH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Stall interrupt is used in slave mode and master mode. In slave mode,SIE set when sending stall handshake packet."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - Stall interrupt is used in slave mode and master mode. In slave mode,SIE set when sending stall handshake packet."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When otg_fs decodes valid USB reset, this position bit"]
    #[inline(always)]
    pub fn usb_rst(&self) -> USB_RST_R {
        USB_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This position bit when any error condition in the ERR_STAT register occurs"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If the otg_fs receives a sof token, this location bit"]
    #[inline(always)]
    pub fn sof_tok(&self) -> SOF_TOK_R {
        SOF_TOK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the token being processed completes"]
    #[inline(always)]
    pub fn tok_dne(&self) -> TOK_DNE_R {
        TOK_DNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If otg_fs detects 3MS idle on the USB bus signal, then Position 1"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When the K state is observed on the DP_DM signal line within 2.5 microseconds, this position 1"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If OTG_FS detects a connection to a USB peripheral, the location bit"]
    #[inline(always)]
    pub fn attach(&self) -> ATTACH_R {
        ATTACH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stall interrupt is used in slave mode and master mode. In slave mode,SIE set when sending stall handshake packet."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When otg_fs decodes valid USB reset, this position bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rst(&mut self) -> USB_RST_W<INT_STAT_SPEC> {
        USB_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - This position bit when any error condition in the ERR_STAT register occurs"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<INT_STAT_SPEC> {
        ERROR_W::new(self, 1)
    }
    #[doc = "Bit 2 - If the otg_fs receives a sof token, this location bit"]
    #[inline(always)]
    #[must_use]
    pub fn sof_tok(&mut self) -> SOF_TOK_W<INT_STAT_SPEC> {
        SOF_TOK_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is set when the token being processed completes"]
    #[inline(always)]
    #[must_use]
    pub fn tok_dne(&mut self) -> TOK_DNE_W<INT_STAT_SPEC> {
        TOK_DNE_W::new(self, 3)
    }
    #[doc = "Bit 4 - If otg_fs detects 3MS idle on the USB bus signal, then Position 1"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<INT_STAT_SPEC> {
        SLEEP_W::new(self, 4)
    }
    #[doc = "Bit 5 - When the K state is observed on the DP_DM signal line within 2.5 microseconds, this position 1"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<INT_STAT_SPEC> {
        RESUME_W::new(self, 5)
    }
    #[doc = "Bit 6 - If OTG_FS detects a connection to a USB peripheral, the location bit"]
    #[inline(always)]
    #[must_use]
    pub fn attach(&mut self) -> ATTACH_W<INT_STAT_SPEC> {
        ATTACH_W::new(self, 6)
    }
    #[doc = "Bit 7 - Stall interrupt is used in slave mode and master mode. In slave mode,SIE set when sending stall handshake packet."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<INT_STAT_SPEC> {
        STALL_W::new(self, 7)
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
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STAT_SPEC;
impl crate::RegisterSpec for INT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_stat::R`](R) reader structure"]
impl crate::Readable for INT_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_stat::W`](W) writer structure"]
impl crate::Writable for INT_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_STAT to value 0x01"]
impl crate::Resettable for INT_STAT_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
