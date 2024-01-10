#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DR_SPEC>;
#[doc = "Field `DAT` reader - This register contains the data to be transimitted or received on the i2c bus."]
pub type DAT_R = crate::FieldReader;
#[doc = "Field `DAT` writer - This register contains the data to be transimitted or received on the i2c bus."]
pub type DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMD` writer - This bit controls whether a read or a write is perormed"]
pub type CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This register contains the data to be transimitted or received on the i2c bus."]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register contains the data to be transimitted or received on the i2c bus."]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<DR_SPEC> {
        DAT_W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit controls whether a read or a write is perormed"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<DR_SPEC> {
        CMD_W::new(self, 8)
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
#[doc = "Data Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: u32 = 0;
}
