#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCR_SPEC>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCR_SPEC>;
#[doc = "Field `UARTEN` reader - UART mode selection bit"]
pub type UARTEN_R = crate::BitReader;
#[doc = "Field `UARTEN` writer - UART mode selection bit"]
pub type UARTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMODE` reader - DMA mode selection bit"]
pub type DMAMODE_R = crate::BitReader;
#[doc = "Field `DMAMODE` writer - DMA mode selection bit"]
pub type DMAMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOFLOWEN` reader - Automatic flow control enable bit"]
pub type AUTOFLOWEN_R = crate::BitReader;
#[doc = "Field `AUTOFLOWEN` writer - Automatic flow control enable bit"]
pub type AUTOFLOWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Enable receive"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXEN` writer - Enable receive"]
pub type RXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Enable transmit"]
pub type TXEN_R = crate::BitReader;
#[doc = "Field `TXEN` writer - Enable transmit"]
pub type TXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELB8` reader - Select bit8"]
pub type SELB8_R = crate::BitReader;
#[doc = "Field `SELB8` writer - Select bit8"]
pub type SELB8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - change swap"]
pub type SWAP_R = crate::BitReader;
#[doc = "Field `SWAP` writer - change swap"]
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTOG` reader - Toggle RX"]
pub type RXTOG_R = crate::BitReader;
#[doc = "Field `RXTOG` writer - Toggle RX"]
pub type RXTOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTOG` reader - Toggle TX"]
pub type TXTOG_R = crate::BitReader;
#[doc = "Field `TXTOG` writer - Toggle TX"]
pub type TXTOG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART mode selection bit"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA mode selection bit"]
    #[inline(always)]
    pub fn dmamode(&self) -> DMAMODE_R {
        DMAMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic flow control enable bit"]
    #[inline(always)]
    pub fn autoflowen(&self) -> AUTOFLOWEN_R {
        AUTOFLOWEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable receive"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable transmit"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Select bit8"]
    #[inline(always)]
    pub fn selb8(&self) -> SELB8_R {
        SELB8_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - change swap"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Toggle RX"]
    #[inline(always)]
    pub fn rxtog(&self) -> RXTOG_R {
        RXTOG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Toggle TX"]
    #[inline(always)]
    pub fn txtog(&self) -> TXTOG_R {
        TXTOG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UARTEN_W<GCR_SPEC> {
        UARTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmamode(&mut self) -> DMAMODE_W<GCR_SPEC> {
        DMAMODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic flow control enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn autoflowen(&mut self) -> AUTOFLOWEN_W<GCR_SPEC> {
        AUTOFLOWEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable receive"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<GCR_SPEC> {
        RXEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable transmit"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<GCR_SPEC> {
        TXEN_W::new(self, 4)
    }
    #[doc = "Bit 7 - Select bit8"]
    #[inline(always)]
    #[must_use]
    pub fn selb8(&mut self) -> SELB8_W<GCR_SPEC> {
        SELB8_W::new(self, 7)
    }
    #[doc = "Bit 8 - change swap"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<GCR_SPEC> {
        SWAP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Toggle RX"]
    #[inline(always)]
    #[must_use]
    pub fn rxtog(&mut self) -> RXTOG_W<GCR_SPEC> {
        RXTOG_W::new(self, 9)
    }
    #[doc = "Bit 10 - Toggle TX"]
    #[inline(always)]
    #[must_use]
    pub fn txtog(&mut self) -> TXTOG_W<GCR_SPEC> {
        TXTOG_W::new(self, 10)
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
#[doc = "Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
