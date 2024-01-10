#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `ADIF` reader - ADC interrupt flag"]
pub type ADIF_R = crate::BitReader;
#[doc = "Field `ADIF` writer - ADC interrupt flag"]
pub type ADIF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ADWIF` reader - ADC window comparator interrupt flag"]
pub type ADWIF_R = crate::BitReader;
#[doc = "Field `ADWIF` writer - ADC window comparator interrupt flag"]
pub type ADWIF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `CHANNEL` reader - Current conversion channel"]
pub type CHANNEL_R = crate::FieldReader;
#[doc = "Field `VALID` reader - Valid flag"]
pub type VALID_R = crate::FieldReader<u16>;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OVERRUN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - ADC interrupt flag"]
    #[inline(always)]
    pub fn adif(&self) -> ADIF_R {
        ADIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt flag"]
    #[inline(always)]
    pub fn adwif(&self) -> ADWIF_R {
        ADWIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Current conversion channel"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:19 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - ADC interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adif(&mut self) -> ADIF_W<SR_SPEC> {
        ADIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adwif(&mut self) -> ADWIF_W<SR_SPEC> {
        ADWIF_W::new(self, 1)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
