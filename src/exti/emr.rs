#[doc = "Register `EMR` reader"]
pub type R = crate::R<EMR_SPEC>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EMR_SPEC>;
#[doc = "Field `EMR0` reader - Event Mask on line 0"]
pub type EMR0_R = crate::BitReader;
#[doc = "Field `EMR0` writer - Event Mask on line 0"]
pub type EMR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR1` reader - Event Mask on line 1"]
pub type EMR1_R = crate::BitReader;
#[doc = "Field `EMR1` writer - Event Mask on line 1"]
pub type EMR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR2` reader - Event Mask on line 2"]
pub type EMR2_R = crate::BitReader;
#[doc = "Field `EMR2` writer - Event Mask on line 2"]
pub type EMR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR3` reader - Event Mask on line 3"]
pub type EMR3_R = crate::BitReader;
#[doc = "Field `EMR3` writer - Event Mask on line 3"]
pub type EMR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR4` reader - Event Mask on line 4"]
pub type EMR4_R = crate::BitReader;
#[doc = "Field `EMR4` writer - Event Mask on line 4"]
pub type EMR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR5` reader - Event Mask on line 5"]
pub type EMR5_R = crate::BitReader;
#[doc = "Field `EMR5` writer - Event Mask on line 5"]
pub type EMR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR6` reader - Event Mask on line 6"]
pub type EMR6_R = crate::BitReader;
#[doc = "Field `EMR6` writer - Event Mask on line 6"]
pub type EMR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR7` reader - Event Mask on line 7"]
pub type EMR7_R = crate::BitReader;
#[doc = "Field `EMR7` writer - Event Mask on line 7"]
pub type EMR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR8` reader - Event Mask on line 8"]
pub type EMR8_R = crate::BitReader;
#[doc = "Field `EMR8` writer - Event Mask on line 8"]
pub type EMR8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR9` reader - Event Mask on line 9"]
pub type EMR9_R = crate::BitReader;
#[doc = "Field `EMR9` writer - Event Mask on line 9"]
pub type EMR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR10` reader - Event Mask on line 10"]
pub type EMR10_R = crate::BitReader;
#[doc = "Field `EMR10` writer - Event Mask on line 10"]
pub type EMR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR11` reader - Event Mask on line 11"]
pub type EMR11_R = crate::BitReader;
#[doc = "Field `EMR11` writer - Event Mask on line 11"]
pub type EMR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR12` reader - Event Mask on line 12"]
pub type EMR12_R = crate::BitReader;
#[doc = "Field `EMR12` writer - Event Mask on line 12"]
pub type EMR12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR13` reader - Event Mask on line 13"]
pub type EMR13_R = crate::BitReader;
#[doc = "Field `EMR13` writer - Event Mask on line 13"]
pub type EMR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR14` reader - Event Mask on line 14"]
pub type EMR14_R = crate::BitReader;
#[doc = "Field `EMR14` writer - Event Mask on line 14"]
pub type EMR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR15` reader - Event Mask on line 15"]
pub type EMR15_R = crate::BitReader;
#[doc = "Field `EMR15` writer - Event Mask on line 15"]
pub type EMR15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR16` reader - Event Mask on line 16"]
pub type EMR16_R = crate::BitReader;
#[doc = "Field `EMR16` writer - Event Mask on line 16"]
pub type EMR16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR17` reader - Event Mask on line 17"]
pub type EMR17_R = crate::BitReader;
#[doc = "Field `EMR17` writer - Event Mask on line 17"]
pub type EMR17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR18` reader - Event Mask on line 18"]
pub type EMR18_R = crate::BitReader;
#[doc = "Field `EMR18` writer - Event Mask on line 18"]
pub type EMR18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR19` reader - Event Mask on line 19"]
pub type EMR19_R = crate::BitReader;
#[doc = "Field `EMR19` writer - Event Mask on line 19"]
pub type EMR19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR20` reader - Event Mask on line 20"]
pub type EMR20_R = crate::BitReader;
#[doc = "Field `EMR20` writer - Event Mask on line 20"]
pub type EMR20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR24` reader - Event Mask on line 24"]
pub type EMR24_R = crate::BitReader;
#[doc = "Field `EMR24` writer - Event Mask on line 24"]
pub type EMR24_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn emr0(&self) -> EMR0_R {
        EMR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn emr1(&self) -> EMR1_R {
        EMR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn emr2(&self) -> EMR2_R {
        EMR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn emr3(&self) -> EMR3_R {
        EMR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn emr4(&self) -> EMR4_R {
        EMR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn emr5(&self) -> EMR5_R {
        EMR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn emr6(&self) -> EMR6_R {
        EMR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn emr7(&self) -> EMR7_R {
        EMR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn emr8(&self) -> EMR8_R {
        EMR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn emr9(&self) -> EMR9_R {
        EMR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn emr10(&self) -> EMR10_R {
        EMR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn emr11(&self) -> EMR11_R {
        EMR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn emr12(&self) -> EMR12_R {
        EMR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn emr13(&self) -> EMR13_R {
        EMR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn emr14(&self) -> EMR14_R {
        EMR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn emr15(&self) -> EMR15_R {
        EMR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn emr16(&self) -> EMR16_R {
        EMR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn emr17(&self) -> EMR17_R {
        EMR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    pub fn emr18(&self) -> EMR18_R {
        EMR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event Mask on line 19"]
    #[inline(always)]
    pub fn emr19(&self) -> EMR19_R {
        EMR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event Mask on line 20"]
    #[inline(always)]
    pub fn emr20(&self) -> EMR20_R {
        EMR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Event Mask on line 24"]
    #[inline(always)]
    pub fn emr24(&self) -> EMR24_R {
        EMR24_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn emr0(&mut self) -> EMR0_W<EMR_SPEC> {
        EMR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn emr1(&mut self) -> EMR1_W<EMR_SPEC> {
        EMR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn emr2(&mut self) -> EMR2_W<EMR_SPEC> {
        EMR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn emr3(&mut self) -> EMR3_W<EMR_SPEC> {
        EMR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn emr4(&mut self) -> EMR4_W<EMR_SPEC> {
        EMR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn emr5(&mut self) -> EMR5_W<EMR_SPEC> {
        EMR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn emr6(&mut self) -> EMR6_W<EMR_SPEC> {
        EMR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn emr7(&mut self) -> EMR7_W<EMR_SPEC> {
        EMR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn emr8(&mut self) -> EMR8_W<EMR_SPEC> {
        EMR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn emr9(&mut self) -> EMR9_W<EMR_SPEC> {
        EMR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn emr10(&mut self) -> EMR10_W<EMR_SPEC> {
        EMR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn emr11(&mut self) -> EMR11_W<EMR_SPEC> {
        EMR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn emr12(&mut self) -> EMR12_W<EMR_SPEC> {
        EMR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn emr13(&mut self) -> EMR13_W<EMR_SPEC> {
        EMR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn emr14(&mut self) -> EMR14_W<EMR_SPEC> {
        EMR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn emr15(&mut self) -> EMR15_W<EMR_SPEC> {
        EMR15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn emr16(&mut self) -> EMR16_W<EMR_SPEC> {
        EMR16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn emr17(&mut self) -> EMR17_W<EMR_SPEC> {
        EMR17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn emr18(&mut self) -> EMR18_W<EMR_SPEC> {
        EMR18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Event Mask on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn emr19(&mut self) -> EMR19_W<EMR_SPEC> {
        EMR19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Event Mask on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn emr20(&mut self) -> EMR20_W<EMR_SPEC> {
        EMR20_W::new(self, 20)
    }
    #[doc = "Bit 24 - Event Mask on line 24"]
    #[inline(always)]
    #[must_use]
    pub fn emr24(&mut self) -> EMR24_W<EMR_SPEC> {
        EMR24_W::new(self, 24)
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
#[doc = "Event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
