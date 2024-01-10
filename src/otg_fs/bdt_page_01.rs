#[doc = "Register `BDT_PAGE_01` reader"]
pub type R = crate::R<BDT_PAGE_01_SPEC>;
#[doc = "Register `BDT_PAGE_01` writer"]
pub type W = crate::W<BDT_PAGE_01_SPEC>;
#[doc = "Field `BDT_BA` reader - The 7_bit value provides address bits 15 to 9 of the BDT base address,which defines the location of the buffer descriptor table in the system memory"]
pub type BDT_BA_R = crate::FieldReader;
#[doc = "Field `BDT_BA` writer - The 7_bit value provides address bits 15 to 9 of the BDT base address,which defines the location of the buffer descriptor table in the system memory"]
pub type BDT_BA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - The 7_bit value provides address bits 15 to 9 of the BDT base address,which defines the location of the buffer descriptor table in the system memory"]
    #[inline(always)]
    pub fn bdt_ba(&self) -> BDT_BA_R {
        BDT_BA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - The 7_bit value provides address bits 15 to 9 of the BDT base address,which defines the location of the buffer descriptor table in the system memory"]
    #[inline(always)]
    #[must_use]
    pub fn bdt_ba(&mut self) -> BDT_BA_W<BDT_PAGE_01_SPEC> {
        BDT_BA_W::new(self, 1)
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
#[doc = "BDT page register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdt_page_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdt_page_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDT_PAGE_01_SPEC;
impl crate::RegisterSpec for BDT_PAGE_01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdt_page_01::R`](R) reader structure"]
impl crate::Readable for BDT_PAGE_01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bdt_page_01::W`](W) writer structure"]
impl crate::Writable for BDT_PAGE_01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDT_PAGE_01 to value 0"]
impl crate::Resettable for BDT_PAGE_01_SPEC {
    const RESET_VALUE: u32 = 0;
}
