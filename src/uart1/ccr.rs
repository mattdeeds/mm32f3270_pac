#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `PEN` reader - Parity enable bit"]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable bit"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSEL` reader - Parity selection bit"]
pub type PSEL_R = crate::BitReader;
#[doc = "Field `PSEL` writer - Parity selection bit"]
pub type PSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPB0` reader - Stop bit 0 selection"]
pub type SPB0_R = crate::BitReader;
#[doc = "Field `SPB0` writer - Stop bit 0 selection"]
pub type SPB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK` reader - UART transmit frame break"]
pub type BRK_R = crate::BitReader;
#[doc = "Field `BRK` writer - UART transmit frame break"]
pub type BRK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAR` reader - UART width bit"]
pub type CHAR_R = crate::FieldReader;
#[doc = "Field `CHAR` writer - UART width bit"]
pub type CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPB1` reader - Stop bit 1 selection bit"]
pub type SPB1_R = crate::BitReader;
#[doc = "Field `SPB1` writer - Stop bit 1 selection bit"]
pub type SPB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8RXD` reader - Synchronous frame receive"]
pub type B8RXD_R = crate::BitReader;
#[doc = "Field `B8TXD` reader - Synchronous frame transmit"]
pub type B8TXD_R = crate::BitReader;
#[doc = "Field `B8TXD` writer - Synchronous frame transmit"]
pub type B8TXD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8POL` reader - Synchronous frame polarity control bit"]
pub type B8POL_R = crate::BitReader;
#[doc = "Field `B8POL` writer - Synchronous frame polarity control bit"]
pub type B8POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8TOG` reader - Synchronous frame auto toggle bit"]
pub type B8TOG_R = crate::BitReader;
#[doc = "Field `B8TOG` writer - Synchronous frame auto toggle bit"]
pub type B8TOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8EN` reader - Synchronous frame enable bit"]
pub type B8EN_R = crate::BitReader;
#[doc = "Field `B8EN` writer - Synchronous frame enable bit"]
pub type B8EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWU` reader - Receive wake up method"]
pub type RWU_R = crate::BitReader;
#[doc = "Field `RWU` writer - Receive wake up method"]
pub type RWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - Wake up method"]
pub type WAKE_R = crate::BitReader;
#[doc = "Field `WAKE` writer - Wake up method"]
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIN` reader - UART LIN enable bit"]
pub type LIN_R = crate::BitReader;
#[doc = "Field `LIN` writer - UART LIN enable bit"]
pub type LIN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity enable bit"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity selection bit"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop bit 0 selection"]
    #[inline(always)]
    pub fn spb0(&self) -> SPB0_R {
        SPB0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART transmit frame break"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - UART width bit"]
    #[inline(always)]
    pub fn char(&self) -> CHAR_R {
        CHAR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Stop bit 1 selection bit"]
    #[inline(always)]
    pub fn spb1(&self) -> SPB1_R {
        SPB1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronous frame receive"]
    #[inline(always)]
    pub fn b8rxd(&self) -> B8RXD_R {
        B8RXD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronous frame transmit"]
    #[inline(always)]
    pub fn b8txd(&self) -> B8TXD_R {
        B8TXD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronous frame polarity control bit"]
    #[inline(always)]
    pub fn b8pol(&self) -> B8POL_R {
        B8POL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronous frame auto toggle bit"]
    #[inline(always)]
    pub fn b8tog(&self) -> B8TOG_R {
        B8TOG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronous frame enable bit"]
    #[inline(always)]
    pub fn b8en(&self) -> B8EN_R {
        B8EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive wake up method"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake up method"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UART LIN enable bit"]
    #[inline(always)]
    pub fn lin(&self) -> LIN_R {
        LIN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<CCR_SPEC> {
        PEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Parity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<CCR_SPEC> {
        PSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Stop bit 0 selection"]
    #[inline(always)]
    #[must_use]
    pub fn spb0(&mut self) -> SPB0_W<CCR_SPEC> {
        SPB0_W::new(self, 2)
    }
    #[doc = "Bit 3 - UART transmit frame break"]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BRK_W<CCR_SPEC> {
        BRK_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - UART width bit"]
    #[inline(always)]
    #[must_use]
    pub fn char(&mut self) -> CHAR_W<CCR_SPEC> {
        CHAR_W::new(self, 4)
    }
    #[doc = "Bit 6 - Stop bit 1 selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn spb1(&mut self) -> SPB1_W<CCR_SPEC> {
        SPB1_W::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous frame transmit"]
    #[inline(always)]
    #[must_use]
    pub fn b8txd(&mut self) -> B8TXD_W<CCR_SPEC> {
        B8TXD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Synchronous frame polarity control bit"]
    #[inline(always)]
    #[must_use]
    pub fn b8pol(&mut self) -> B8POL_W<CCR_SPEC> {
        B8POL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Synchronous frame auto toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn b8tog(&mut self) -> B8TOG_W<CCR_SPEC> {
        B8TOG_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronous frame enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn b8en(&mut self) -> B8EN_W<CCR_SPEC> {
        B8EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Receive wake up method"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<CCR_SPEC> {
        RWU_W::new(self, 12)
    }
    #[doc = "Bit 13 - Wake up method"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<CCR_SPEC> {
        WAKE_W::new(self, 13)
    }
    #[doc = "Bit 14 - UART LIN enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lin(&mut self) -> LIN_W<CCR_SPEC> {
        LIN_W::new(self, 14)
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
#[doc = "common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0x30"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
