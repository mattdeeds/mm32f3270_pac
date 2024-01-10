#[doc = "Register `TXID0_B` reader"]
pub type R = crate::R<TXID0_B_SPEC>;
#[doc = "Register `TXID0_B` writer"]
pub type W = crate::W<TXID0_B_SPEC>;
#[doc = "Field `ID3` reader - Identifier bit 3"]
pub type ID3_R = crate::BitReader;
#[doc = "Field `ID3` writer - Identifier bit 3"]
pub type ID3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID4` reader - Identifier bit 4"]
pub type ID4_R = crate::BitReader;
#[doc = "Field `ID4` writer - Identifier bit 4"]
pub type ID4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID5` reader - Identifier bit 5"]
pub type ID5_R = crate::BitReader;
#[doc = "Field `ID5` writer - Identifier bit 5"]
pub type ID5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID6` reader - Identifier bit 6"]
pub type ID6_R = crate::BitReader;
#[doc = "Field `ID6` writer - Identifier bit 6"]
pub type ID6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID7` reader - Identifier bit 7"]
pub type ID7_R = crate::BitReader;
#[doc = "Field `ID7` writer - Identifier bit 7"]
pub type ID7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID8` reader - Identifier bit 8"]
pub type ID8_R = crate::BitReader;
#[doc = "Field `ID8` writer - Identifier bit 8"]
pub type ID8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID9` reader - Identifier bit 9"]
pub type ID9_R = crate::BitReader;
#[doc = "Field `ID9` writer - Identifier bit 9"]
pub type ID9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID10` reader - Identifier bit 10"]
pub type ID10_R = crate::BitReader;
#[doc = "Field `ID10` writer - Identifier bit 10"]
pub type ID10_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Identifier bit 3"]
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifier bit 4"]
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier bit 5"]
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Identifier bit 6"]
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Identifier bit 7"]
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 8"]
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 9"]
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 10"]
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Identifier bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn id3(&mut self) -> ID3_W<TXID0_B_SPEC> {
        ID3_W::new(self, 0)
    }
    #[doc = "Bit 1 - Identifier bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn id4(&mut self) -> ID4_W<TXID0_B_SPEC> {
        ID4_W::new(self, 1)
    }
    #[doc = "Bit 2 - Identifier bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn id5(&mut self) -> ID5_W<TXID0_B_SPEC> {
        ID5_W::new(self, 2)
    }
    #[doc = "Bit 3 - Identifier bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn id6(&mut self) -> ID6_W<TXID0_B_SPEC> {
        ID6_W::new(self, 3)
    }
    #[doc = "Bit 4 - Identifier bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn id7(&mut self) -> ID7_W<TXID0_B_SPEC> {
        ID7_W::new(self, 4)
    }
    #[doc = "Bit 5 - Identifier bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn id8(&mut self) -> ID8_W<TXID0_B_SPEC> {
        ID8_W::new(self, 5)
    }
    #[doc = "Bit 6 - Identifier bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn id9(&mut self) -> ID9_W<TXID0_B_SPEC> {
        ID9_W::new(self, 6)
    }
    #[doc = "Bit 7 - Identifier bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn id10(&mut self) -> ID10_W<TXID0_B_SPEC> {
        ID10_W::new(self, 7)
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
#[doc = "Basic TX ID register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txid0_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txid0_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXID0_B_SPEC;
impl crate::RegisterSpec for TXID0_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txid0_b::R`](R) reader structure"]
impl crate::Readable for TXID0_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txid0_b::W`](W) writer structure"]
impl crate::Writable for TXID0_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXID0_B to value 0"]
impl crate::Resettable for TXID0_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
