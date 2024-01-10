#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `OKIE` reader - SYNC event OK interrupt enable"]
pub type OKIE_R = crate::BitReader;
#[doc = "Field `OKIE` writer - SYNC event OK interrupt enable"]
pub type OKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARNIE` reader - SYNC warning interrupt enable"]
pub type WARNIE_R = crate::BitReader;
#[doc = "Field `WARNIE` writer - SYNC warning interrupt enable"]
pub type WARNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Synchronization or trimming error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Synchronization or trimming error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXPTIE` reader - Expected SYNC interrupt enable"]
pub type EXPTIE_R = crate::BitReader;
#[doc = "Field `EXPTIE` writer - Expected SYNC interrupt enable"]
pub type EXPTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTEN` reader - Frequency error counter enable"]
pub type CNTEN_R = crate::BitReader;
#[doc = "Field `CNTEN` writer - Frequency error counter enable"]
pub type CNTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_TRIMEN` reader - Automatic trimming enable"]
pub type AUTO_TRIMEN_R = crate::BitReader;
#[doc = "Field `AUTO_TRIMEN` writer - Automatic trimming enable"]
pub type AUTO_TRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWSYNC` reader - Generate software SYNC event"]
pub type SWSYNC_R = crate::BitReader;
#[doc = "Field `SWSYNC` writer - Generate software SYNC event"]
pub type SWSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM` reader - HSI48 oscillator smooth trimming"]
pub type TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `TRIM` writer - HSI48 oscillator smooth trimming"]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn okie(&self) -> OKIE_R {
        OKIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn warnie(&self) -> WARNIE_R {
        WARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn exptie(&self) -> EXPTIE_R {
        EXPTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Frequency error counter enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic trimming enable"]
    #[inline(always)]
    pub fn auto_trimen(&self) -> AUTO_TRIMEN_R {
        AUTO_TRIMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generate software SYNC event"]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17 - HSI48 oscillator smooth trimming"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn okie(&mut self) -> OKIE_W<CR_SPEC> {
        OKIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn warnie(&mut self) -> WARNIE_W<CR_SPEC> {
        WARNIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CR_SPEC> {
        ERRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn exptie(&mut self) -> EXPTIE_W<CR_SPEC> {
        EXPTIE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Frequency error counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnten(&mut self) -> CNTEN_W<CR_SPEC> {
        CNTEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic trimming enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_trimen(&mut self) -> AUTO_TRIMEN_W<CR_SPEC> {
        AUTO_TRIMEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Generate software SYNC event"]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SWSYNC_W<CR_SPEC> {
        SWSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:17 - HSI48 oscillator smooth trimming"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<CR_SPEC> {
        TRIM_W::new(self, 8)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x2000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
