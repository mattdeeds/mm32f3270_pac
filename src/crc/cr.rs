#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RESET` writer - CRC reset"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLY32_MGN` reader - CRC algorithm selection"]
pub type POLY32_MGN_R = crate::BitReader;
#[doc = "Field `POLY32_MGN` writer - CRC algorithm selection"]
pub type POLY32_MGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_BITSEL` reader - Input bit selection"]
pub type CRC_BITSEL_R = crate::FieldReader;
#[doc = "Field `CRC_BITSEL` writer - Input bit selection"]
pub type CRC_BITSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIG_EI` reader - Input big or small selection"]
pub type BIG_EI_R = crate::BitReader;
#[doc = "Field `BIG_EI` writer - Input big or small selection"]
pub type BIG_EI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIG_EO` reader - Output big or small selection"]
pub type BIG_EO_R = crate::BitReader;
#[doc = "Field `BIG_EO` writer - Output big or small selection"]
pub type BIG_EO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - CRC algorithm selection"]
    #[inline(always)]
    pub fn poly32_mgn(&self) -> POLY32_MGN_R {
        POLY32_MGN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Input bit selection"]
    #[inline(always)]
    pub fn crc_bitsel(&self) -> CRC_BITSEL_R {
        CRC_BITSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Input big or small selection"]
    #[inline(always)]
    pub fn big_ei(&self) -> BIG_EI_R {
        BIG_EI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output big or small selection"]
    #[inline(always)]
    pub fn big_eo(&self) -> BIG_EO_R {
        BIG_EO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CR_SPEC> {
        RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - CRC algorithm selection"]
    #[inline(always)]
    #[must_use]
    pub fn poly32_mgn(&mut self) -> POLY32_MGN_W<CR_SPEC> {
        POLY32_MGN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Input bit selection"]
    #[inline(always)]
    #[must_use]
    pub fn crc_bitsel(&mut self) -> CRC_BITSEL_W<CR_SPEC> {
        CRC_BITSEL_W::new(self, 2)
    }
    #[doc = "Bit 4 - Input big or small selection"]
    #[inline(always)]
    #[must_use]
    pub fn big_ei(&mut self) -> BIG_EI_W<CR_SPEC> {
        BIG_EI_W::new(self, 4)
    }
    #[doc = "Bit 5 - Output big or small selection"]
    #[inline(always)]
    #[must_use]
    pub fn big_eo(&mut self) -> BIG_EO_W<CR_SPEC> {
        BIG_EO_W::new(self, 5)
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
