#[doc = "Register `CHSR` reader"]
pub type R = crate::R<CHSR_SPEC>;
#[doc = "Register `CHSR` writer"]
pub type W = crate::W<CHSR_SPEC>;
#[doc = "Field `CHEN0` reader - Analog input channel 0 enable"]
pub type CHEN0_R = crate::BitReader;
#[doc = "Field `CHEN0` writer - Analog input channel 0 enable"]
pub type CHEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN1` reader - Analog input channel 1 enable"]
pub type CHEN1_R = crate::BitReader;
#[doc = "Field `CHEN1` writer - Analog input channel 1 enable"]
pub type CHEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN2` reader - Analog input channel 2 enable"]
pub type CHEN2_R = crate::BitReader;
#[doc = "Field `CHEN2` writer - Analog input channel 2 enable"]
pub type CHEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN3` reader - Analog input channel 3 enable"]
pub type CHEN3_R = crate::BitReader;
#[doc = "Field `CHEN3` writer - Analog input channel 3 enable"]
pub type CHEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN4` reader - Analog input channel 4 enable"]
pub type CHEN4_R = crate::BitReader;
#[doc = "Field `CHEN4` writer - Analog input channel 4 enable"]
pub type CHEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN5` reader - Analog input channel 5 enable"]
pub type CHEN5_R = crate::BitReader;
#[doc = "Field `CHEN5` writer - Analog input channel 5 enable"]
pub type CHEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN6` reader - Analog input channel 6 enable"]
pub type CHEN6_R = crate::BitReader;
#[doc = "Field `CHEN6` writer - Analog input channel 6 enable"]
pub type CHEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN7` reader - Analog input channel 7 enable"]
pub type CHEN7_R = crate::BitReader;
#[doc = "Field `CHEN7` writer - Analog input channel 7 enable"]
pub type CHEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN8` reader - Analog input channel 8 enable"]
pub type CHEN8_R = crate::BitReader;
#[doc = "Field `CHEN8` writer - Analog input channel 8 enable"]
pub type CHEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN9` reader - Analog input channel 9 enable"]
pub type CHEN9_R = crate::BitReader;
#[doc = "Field `CHEN9` writer - Analog input channel 9 enable"]
pub type CHEN9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENTS` reader - emperature Sensor enable"]
pub type CHENTS_R = crate::BitReader;
#[doc = "Field `CHENTS` writer - emperature Sensor enable"]
pub type CHENTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENVS` reader - Voltage Sensor enable"]
pub type CHENVS_R = crate::BitReader;
#[doc = "Field `CHENVS` writer - Voltage Sensor enable"]
pub type CHENVS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog input channel 0 enable"]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog input channel 1 enable"]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog input channel 2 enable"]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog input channel 3 enable"]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog input channel 4 enable"]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog input channel 5 enable"]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog input channel 6 enable"]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog input channel 7 enable"]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog input channel 8 enable"]
    #[inline(always)]
    pub fn chen8(&self) -> CHEN8_R {
        CHEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog input channel 9 enable"]
    #[inline(always)]
    pub fn chen9(&self) -> CHEN9_R {
        CHEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - emperature Sensor enable"]
    #[inline(always)]
    pub fn chents(&self) -> CHENTS_R {
        CHENTS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Voltage Sensor enable"]
    #[inline(always)]
    pub fn chenvs(&self) -> CHENVS_R {
        CHENVS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog input channel 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen0(&mut self) -> CHEN0_W<CHSR_SPEC> {
        CHEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog input channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen1(&mut self) -> CHEN1_W<CHSR_SPEC> {
        CHEN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Analog input channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen2(&mut self) -> CHEN2_W<CHSR_SPEC> {
        CHEN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog input channel 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen3(&mut self) -> CHEN3_W<CHSR_SPEC> {
        CHEN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog input channel 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen4(&mut self) -> CHEN4_W<CHSR_SPEC> {
        CHEN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog input channel 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen5(&mut self) -> CHEN5_W<CHSR_SPEC> {
        CHEN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Analog input channel 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen6(&mut self) -> CHEN6_W<CHSR_SPEC> {
        CHEN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog input channel 7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen7(&mut self) -> CHEN7_W<CHSR_SPEC> {
        CHEN7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog input channel 8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen8(&mut self) -> CHEN8_W<CHSR_SPEC> {
        CHEN8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog input channel 9 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen9(&mut self) -> CHEN9_W<CHSR_SPEC> {
        CHEN9_W::new(self, 9)
    }
    #[doc = "Bit 14 - emperature Sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn chents(&mut self) -> CHENTS_W<CHSR_SPEC> {
        CHENTS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Voltage Sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn chenvs(&mut self) -> CHENVS_W<CHSR_SPEC> {
        CHENVS_W::new(self, 15)
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
#[doc = "Channel select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSR_SPEC;
impl crate::RegisterSpec for CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsr::R`](R) reader structure"]
impl crate::Readable for CHSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chsr::W`](W) writer structure"]
impl crate::Writable for CHSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for CHSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
