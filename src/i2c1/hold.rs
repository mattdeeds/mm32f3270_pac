#[doc = "Register `HOLD` reader"]
pub type R = crate::R<HOLD_SPEC>;
#[doc = "Register `HOLD` writer"]
pub type W = crate::W<HOLD_SPEC>;
#[doc = "Field `TX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period"]
pub type TX_HOLD_R = crate::FieldReader<u16>;
#[doc = "Field `TX_HOLD` writer - Sets the required SDA hold time in units of ic_clk period"]
pub type TX_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period"]
pub type RX_HOLD_R = crate::FieldReader;
#[doc = "Field `RX_HOLD` writer - Sets the required SDA hold time in units of ic_clk period"]
pub type RX_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    pub fn tx_hold(&self) -> TX_HOLD_R {
        TX_HOLD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    pub fn rx_hold(&self) -> RX_HOLD_R {
        RX_HOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hold(&mut self) -> TX_HOLD_W<HOLD_SPEC> {
        TX_HOLD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hold(&mut self) -> RX_HOLD_W<HOLD_SPEC> {
        RX_HOLD_W::new(self, 16)
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
#[doc = "SDA Hold Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOLD_SPEC;
impl crate::RegisterSpec for HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hold::R`](R) reader structure"]
impl crate::Readable for HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hold::W`](W) writer structure"]
impl crate::Writable for HOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOLD to value 0x01"]
impl crate::Resettable for HOLD_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
