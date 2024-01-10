#[doc = "Register `TXID1_P` reader"]
pub type R = crate::R<TXID1_P_SPEC>;
#[doc = "Register `TXID1_P` writer"]
pub type W = crate::W<TXID1_P_SPEC>;
#[doc = "Field `ID13` reader - Identifier bit 13"]
pub type ID13_R = crate::BitReader;
#[doc = "Field `ID13` writer - Identifier bit 13"]
pub type ID13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID14` reader - Identifier bit 14"]
pub type ID14_R = crate::BitReader;
#[doc = "Field `ID14` writer - Identifier bit 14"]
pub type ID14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID15` reader - Identifier bit 15"]
pub type ID15_R = crate::BitReader;
#[doc = "Field `ID15` writer - Identifier bit 15"]
pub type ID15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID16` reader - Identifier bit 16"]
pub type ID16_R = crate::BitReader;
#[doc = "Field `ID16` writer - Identifier bit 16"]
pub type ID16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID17` reader - Identifier bit 17"]
pub type ID17_R = crate::BitReader;
#[doc = "Field `ID17` writer - Identifier bit 17"]
pub type ID17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID18` reader - Identifier bit 18"]
pub type ID18_R = crate::BitReader;
#[doc = "Field `ID18` writer - Identifier bit 18"]
pub type ID18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID19` reader - Identifier bit 19"]
pub type ID19_R = crate::BitReader;
#[doc = "Field `ID19` writer - Identifier bit 19"]
pub type ID19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID20` reader - Identifier bit 20"]
pub type ID20_R = crate::BitReader;
#[doc = "Field `ID20` writer - Identifier bit 20"]
pub type ID20_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Identifier bit 13"]
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifier bit 14"]
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier bit 15"]
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Identifier bit 16"]
    #[inline(always)]
    pub fn id16(&self) -> ID16_R {
        ID16_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Identifier bit 17"]
    #[inline(always)]
    pub fn id17(&self) -> ID17_R {
        ID17_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 18"]
    #[inline(always)]
    pub fn id18(&self) -> ID18_R {
        ID18_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 19"]
    #[inline(always)]
    pub fn id19(&self) -> ID19_R {
        ID19_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 20"]
    #[inline(always)]
    pub fn id20(&self) -> ID20_R {
        ID20_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Identifier bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn id13(&mut self) -> ID13_W<TXID1_P_SPEC> {
        ID13_W::new(self, 0)
    }
    #[doc = "Bit 1 - Identifier bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn id14(&mut self) -> ID14_W<TXID1_P_SPEC> {
        ID14_W::new(self, 1)
    }
    #[doc = "Bit 2 - Identifier bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn id15(&mut self) -> ID15_W<TXID1_P_SPEC> {
        ID15_W::new(self, 2)
    }
    #[doc = "Bit 3 - Identifier bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn id16(&mut self) -> ID16_W<TXID1_P_SPEC> {
        ID16_W::new(self, 3)
    }
    #[doc = "Bit 4 - Identifier bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn id17(&mut self) -> ID17_W<TXID1_P_SPEC> {
        ID17_W::new(self, 4)
    }
    #[doc = "Bit 5 - Identifier bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn id18(&mut self) -> ID18_W<TXID1_P_SPEC> {
        ID18_W::new(self, 5)
    }
    #[doc = "Bit 6 - Identifier bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn id19(&mut self) -> ID19_W<TXID1_P_SPEC> {
        ID19_W::new(self, 6)
    }
    #[doc = "Bit 7 - Identifier bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn id20(&mut self) -> ID20_W<TXID1_P_SPEC> {
        ID20_W::new(self, 7)
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
#[doc = "Peli TX ID register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txid1_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txid1_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXID1_P_SPEC;
impl crate::RegisterSpec for TXID1_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txid1_p::R`](R) reader structure"]
impl crate::Readable for TXID1_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txid1_p::W`](W) writer structure"]
impl crate::Writable for TXID1_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXID1_P to value 0"]
impl crate::Resettable for TXID1_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
