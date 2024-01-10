#[doc = "Register `BDT_PAGE_02` reader"]
pub type R = crate::R<BDT_PAGE_02_SPEC>;
#[doc = "Register `BDT_PAGE_02` writer"]
pub type W = crate::W<BDT_PAGE_02_SPEC>;
#[doc = "Field `BDT_BA_23_16` reader - The 8_bit value provides address bits 23 to 16 of the BDT base address, which defines the location of the buffer descriptor table in the system memory"]
pub type BDT_BA_23_16_R = crate::FieldReader;
#[doc = "Field `BDT_BA_23_16` writer - The 8_bit value provides address bits 23 to 16 of the BDT base address, which defines the location of the buffer descriptor table in the system memory"]
pub type BDT_BA_23_16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The 8_bit value provides address bits 23 to 16 of the BDT base address, which defines the location of the buffer descriptor table in the system memory"]
    #[inline(always)]
    pub fn bdt_ba_23_16(&self) -> BDT_BA_23_16_R {
        BDT_BA_23_16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The 8_bit value provides address bits 23 to 16 of the BDT base address, which defines the location of the buffer descriptor table in the system memory"]
    #[inline(always)]
    #[must_use]
    pub fn bdt_ba_23_16(&mut self) -> BDT_BA_23_16_W<BDT_PAGE_02_SPEC> {
        BDT_BA_23_16_W::new(self, 0)
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
#[doc = "BDT page register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdt_page_02::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdt_page_02::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDT_PAGE_02_SPEC;
impl crate::RegisterSpec for BDT_PAGE_02_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdt_page_02::R`](R) reader structure"]
impl crate::Readable for BDT_PAGE_02_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bdt_page_02::W`](W) writer structure"]
impl crate::Writable for BDT_PAGE_02_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDT_PAGE_02 to value 0"]
impl crate::Resettable for BDT_PAGE_02_SPEC {
    const RESET_VALUE: u32 = 0;
}
