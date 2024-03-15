#[doc = "Register `PCIE_CLIENT_ERR_CNT` reader"]
pub type R = crate::R<PcieClientErrCntSpec>;
#[doc = "Register `PCIE_CLIENT_ERR_CNT` writer"]
pub type W = crate::W<PcieClientErrCntSpec>;
#[doc = "Field `FATAL_ERR_CNT` reader - Fatal error counter Fatal error counter, write all one(8'hff) clear the counter."]
pub type FatalErrCntR = crate::FieldReader;
#[doc = "Field `FATAL_ERR_CNT` writer - Fatal error counter Fatal error counter, write all one(8'hff) clear the counter."]
pub type FatalErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFATAL_ERR_CNT` reader - Non-fatal error counter Non-fatal error counter, write all one(8'hff) clear the counter."]
pub type NfatalErrCntR = crate::FieldReader;
#[doc = "Field `NFATAL_ERR_CNT` writer - Non-fatal error counter Non-fatal error counter, write all one(8'hff) clear the counter."]
pub type NfatalErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CORR_ERR_CNT` reader - Correctable error counter Correctable error counter, write all one(8'hff) clear the counter."]
pub type CorrErrCntR = crate::FieldReader;
#[doc = "Field `CORR_ERR_CNT` writer - Correctable error counter Correctable error counter, write all one(8'hff) clear the counter."]
pub type CorrErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Fatal error counter Fatal error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    pub fn fatal_err_cnt(&self) -> FatalErrCntR {
        FatalErrCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-fatal error counter Non-fatal error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    pub fn nfatal_err_cnt(&self) -> NfatalErrCntR {
        NfatalErrCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Correctable error counter Correctable error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    pub fn corr_err_cnt(&self) -> CorrErrCntR {
        CorrErrCntR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fatal error counter Fatal error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    #[must_use]
    pub fn fatal_err_cnt(&mut self) -> FatalErrCntW<PcieClientErrCntSpec> {
        FatalErrCntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Non-fatal error counter Non-fatal error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    #[must_use]
    pub fn nfatal_err_cnt(&mut self) -> NfatalErrCntW<PcieClientErrCntSpec> {
        NfatalErrCntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Correctable error counter Correctable error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    #[must_use]
    pub fn corr_err_cnt(&mut self) -> CorrErrCntW<PcieClientErrCntSpec> {
        CorrErrCntW::new(self, 16)
    }
}
#[doc = "Error counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_err_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_err_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientErrCntSpec;
impl crate::RegisterSpec for PcieClientErrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_err_cnt::R`](R) reader structure"]
impl crate::Readable for PcieClientErrCntSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_err_cnt::W`](W) writer structure"]
impl crate::Writable for PcieClientErrCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00ff_ffff;
}
#[doc = "`reset()` method sets PCIE_CLIENT_ERR_CNT to value 0"]
impl crate::Resettable for PcieClientErrCntSpec {
    const RESET_VALUE: u32 = 0;
}