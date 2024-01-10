#[doc = "Register `EP_CTL7` reader"]
pub type R = crate::R<EP_CTL7_SPEC>;
#[doc = "Register `EP_CTL7` writer"]
pub type W = crate::W<EP_CTL7_SPEC>;
#[doc = "Field `EP_HSHK_7` reader - Setting this bit defines whether the endpoint performs a handshake to the endpoint during the transaction"]
pub type EP_HSHK_7_R = crate::BitReader;
#[doc = "Field `EP_HSHK_7` writer - Setting this bit defines whether the endpoint performs a handshake to the endpoint during the transaction"]
pub type EP_HSHK_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_STALL_7` reader - Any access to this endpoint will cause the otg_fs to return a stall handshake"]
pub type EP_STALL_7_R = crate::BitReader;
#[doc = "Field `EP_STALL_7` writer - Any access to this endpoint will cause the otg_fs to return a stall handshake"]
pub type EP_STALL_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_TX_EN_7` reader - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_TX_EN_7_R = crate::BitReader;
#[doc = "Field `EP_TX_EN_7` writer - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_TX_EN_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_RX_EN_7` reader - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_RX_EN_7_R = crate::BitReader;
#[doc = "Field `EP_RX_EN_7` writer - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_RX_EN_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_CTL_DIS_7` reader - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_CTL_DIS_7_R = crate::BitReader;
#[doc = "Field `EP_CTL_DIS_7` writer - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
pub type EP_CTL_DIS_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY_DIS_7` reader - This is a bit for master mode only and exists only in the control register of endpoint 0"]
pub type RETRY_DIS_7_R = crate::BitReader;
#[doc = "Field `RETRY_DIS_7` writer - This is a bit for master mode only and exists only in the control register of endpoint 0"]
pub type RETRY_DIS_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_WO_HUB_7` reader - This is a bit for master mode only and exists only in the control endpt0_rg register of endpoint 0."]
pub type HOST_WO_HUB_7_R = crate::BitReader;
#[doc = "Field `HOST_WO_HUB_7` writer - This is a bit for master mode only and exists only in the control endpt0_rg register of endpoint 0."]
pub type HOST_WO_HUB_7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting this bit defines whether the endpoint performs a handshake to the endpoint during the transaction"]
    #[inline(always)]
    pub fn ep_hshk_7(&self) -> EP_HSHK_7_R {
        EP_HSHK_7_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Any access to this endpoint will cause the otg_fs to return a stall handshake"]
    #[inline(always)]
    pub fn ep_stall_7(&self) -> EP_STALL_7_R {
        EP_STALL_7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    pub fn ep_tx_en_7(&self) -> EP_TX_EN_7_R {
        EP_TX_EN_7_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    pub fn ep_rx_en_7(&self) -> EP_RX_EN_7_R {
        EP_RX_EN_7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    pub fn ep_ctl_dis_7(&self) -> EP_CTL_DIS_7_R {
        EP_CTL_DIS_7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - This is a bit for master mode only and exists only in the control register of endpoint 0"]
    #[inline(always)]
    pub fn retry_dis_7(&self) -> RETRY_DIS_7_R {
        RETRY_DIS_7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is a bit for master mode only and exists only in the control endpt0_rg register of endpoint 0."]
    #[inline(always)]
    pub fn host_wo_hub_7(&self) -> HOST_WO_HUB_7_R {
        HOST_WO_HUB_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit defines whether the endpoint performs a handshake to the endpoint during the transaction"]
    #[inline(always)]
    #[must_use]
    pub fn ep_hshk_7(&mut self) -> EP_HSHK_7_W<EP_CTL7_SPEC> {
        EP_HSHK_7_W::new(self, 0)
    }
    #[doc = "Bit 1 - Any access to this endpoint will cause the otg_fs to return a stall handshake"]
    #[inline(always)]
    #[must_use]
    pub fn ep_stall_7(&mut self) -> EP_STALL_7_W<EP_CTL7_SPEC> {
        EP_STALL_7_W::new(self, 1)
    }
    #[doc = "Bit 2 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ep_tx_en_7(&mut self) -> EP_TX_EN_7_W<EP_CTL7_SPEC> {
        EP_TX_EN_7_W::new(self, 2)
    }
    #[doc = "Bit 3 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ep_rx_en_7(&mut self) -> EP_RX_EN_7_W<EP_CTL7_SPEC> {
        EP_RX_EN_7_W::new(self, 3)
    }
    #[doc = "Bit 4 - These three bits define whether the endpoint is enabled and the direction of the endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ep_ctl_dis_7(&mut self) -> EP_CTL_DIS_7_W<EP_CTL7_SPEC> {
        EP_CTL_DIS_7_W::new(self, 4)
    }
    #[doc = "Bit 6 - This is a bit for master mode only and exists only in the control register of endpoint 0"]
    #[inline(always)]
    #[must_use]
    pub fn retry_dis_7(&mut self) -> RETRY_DIS_7_W<EP_CTL7_SPEC> {
        RETRY_DIS_7_W::new(self, 6)
    }
    #[doc = "Bit 7 - This is a bit for master mode only and exists only in the control endpt0_rg register of endpoint 0."]
    #[inline(always)]
    #[must_use]
    pub fn host_wo_hub_7(&mut self) -> HOST_WO_HUB_7_W<EP_CTL7_SPEC> {
        HOST_WO_HUB_7_W::new(self, 7)
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
#[doc = "Endpoint control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_ctl7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_ctl7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP_CTL7_SPEC;
impl crate::RegisterSpec for EP_CTL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_ctl7::R`](R) reader structure"]
impl crate::Readable for EP_CTL7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep_ctl7::W`](W) writer structure"]
impl crate::Writable for EP_CTL7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP_CTL7 to value 0"]
impl crate::Resettable for EP_CTL7_SPEC {
    const RESET_VALUE: u32 = 0;
}
