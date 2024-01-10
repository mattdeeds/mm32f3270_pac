#[doc = "Register `TXID1_B` reader"]
pub type R = crate::R<TXID1_B_SPEC>;
#[doc = "Register `TXID1_B` writer"]
pub type W = crate::W<TXID1_B_SPEC>;
#[doc = "Field `DLC` reader - Data length code"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - Data length code"]
pub type DLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTR` reader - Remote transmission request"]
pub type RTR_R = crate::BitReader;
#[doc = "Field `RTR` writer - Remote transmission request"]
pub type RTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID0` reader - Identifier bit 0"]
pub type ID0_R = crate::BitReader;
#[doc = "Field `ID0` writer - Identifier bit 0"]
pub type ID0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID1` reader - Identifier bit 1"]
pub type ID1_R = crate::BitReader;
#[doc = "Field `ID1` writer - Identifier bit 1"]
pub type ID1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID2` reader - Identifier bit 2"]
pub type ID2_R = crate::BitReader;
#[doc = "Field `ID2` writer - Identifier bit 2"]
pub type ID2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 0"]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 1"]
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 2"]
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<TXID1_B_SPEC> {
        DLC_W::new(self, 0)
    }
    #[doc = "Bit 4 - Remote transmission request"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<TXID1_B_SPEC> {
        RTR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Identifier bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn id0(&mut self) -> ID0_W<TXID1_B_SPEC> {
        ID0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Identifier bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn id1(&mut self) -> ID1_W<TXID1_B_SPEC> {
        ID1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Identifier bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn id2(&mut self) -> ID2_W<TXID1_B_SPEC> {
        ID2_W::new(self, 7)
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
#[doc = "Basic TX ID register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txid1_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txid1_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXID1_B_SPEC;
impl crate::RegisterSpec for TXID1_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txid1_b::R`](R) reader structure"]
impl crate::Readable for TXID1_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txid1_b::W`](W) writer structure"]
impl crate::Writable for TXID1_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXID1_B to value 0"]
impl crate::Resettable for TXID1_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
