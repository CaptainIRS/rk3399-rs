#[doc = "Register `PCIE_LM_ECC_CORRECTABLE_ERROR_COUNT` reader"]
pub type R = crate::R<PcieLmEccCorrectableErrorCountSpec>;
#[doc = "Register `PCIE_LM_ECC_CORRECTABLE_ERROR_COUNT` writer"]
pub type W = crate::W<PcieLmEccCorrectableErrorCountSpec>;
#[doc = "Field `PFRCER` reader - PNP FIFO RAM Correctable Error Count \\[PFRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the PNP\n\nFIFO RAM. This is an 8-bit saturating\n\ncounter that can be cleared by\n\nwriting all 1's into it."]
pub type PfrcerR = crate::FieldReader;
#[doc = "Field `PFRCER` writer - PNP FIFO RAM Correctable Error Count \\[PFRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the PNP\n\nFIFO RAM. This is an 8-bit saturating\n\ncounter that can be cleared by\n\nwriting all 1's into it."]
pub type PfrcerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SFRCER` reader - SC FIFO RAM Correctable Error Count \\[SFRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the SC\n\nFIFO RAM. This is an 8-bit saturating\n\ncounter that can be cleared by\n\nwriting all 1's into it."]
pub type SfrcerR = crate::FieldReader;
#[doc = "Field `SFRCER` writer - SC FIFO RAM Correctable Error Count \\[SFRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the SC\n\nFIFO RAM. This is an 8-bit saturating\n\ncounter that can be cleared by\n\nwriting all 1's into it."]
pub type SfrcerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RRCER` reader - Replay RAM Correctable Error Count \\[RRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the\n\nReplay Buffer RAM. This is an 8- bit\n\nsaturating counter that can be\n\ncleared by writing all 1's into it."]
pub type RrcerR = crate::FieldReader;
#[doc = "Field `RRCER` writer - Replay RAM Correctable Error Count \\[RRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the\n\nReplay Buffer RAM. This is an 8- bit\n\nsaturating counter that can be\n\ncleared by writing all 1's into it."]
pub type RrcerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R12` reader - TPH ST RAM Correctable Error Count \\[R12\\]\n\nNumber of correctable errors\n\ndetected while reading from the TPH\n\nSteering Tag RAM. This is an 8-bit\n\nsaturating counter that can be\n\ncleared by writing all 1s into it."]
pub type R12R = crate::FieldReader;
#[doc = "Field `R12` writer - TPH ST RAM Correctable Error Count \\[R12\\]\n\nNumber of correctable errors\n\ndetected while reading from the TPH\n\nSteering Tag RAM. This is an 8-bit\n\nsaturating counter that can be\n\ncleared by writing all 1s into it."]
pub type R12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PNP FIFO RAM Correctable Error Count \\[PFRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the PNP\n\nFIFO RAM. This is an 8-bit saturating\n\ncounter that can be cleared by\n\nwriting all 1's into it."]
    #[inline(always)]
    pub fn pfrcer(&self) -> PfrcerR {
        PfrcerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SC FIFO RAM Correctable Error Count \\[SFRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the SC\n\nFIFO RAM. This is an 8-bit saturating\n\ncounter that can be cleared by\n\nwriting all 1's into it."]
    #[inline(always)]
    pub fn sfrcer(&self) -> SfrcerR {
        SfrcerR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Replay RAM Correctable Error Count \\[RRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the\n\nReplay Buffer RAM. This is an 8- bit\n\nsaturating counter that can be\n\ncleared by writing all 1's into it."]
    #[inline(always)]
    pub fn rrcer(&self) -> RrcerR {
        RrcerR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TPH ST RAM Correctable Error Count \\[R12\\]\n\nNumber of correctable errors\n\ndetected while reading from the TPH\n\nSteering Tag RAM. This is an 8-bit\n\nsaturating counter that can be\n\ncleared by writing all 1s into it."]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PNP FIFO RAM Correctable Error Count \\[PFRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the PNP\n\nFIFO RAM. This is an 8-bit saturating\n\ncounter that can be cleared by\n\nwriting all 1's into it."]
    #[inline(always)]
    #[must_use]
    pub fn pfrcer(&mut self) -> PfrcerW<PcieLmEccCorrectableErrorCountSpec> {
        PfrcerW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SC FIFO RAM Correctable Error Count \\[SFRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the SC\n\nFIFO RAM. This is an 8-bit saturating\n\ncounter that can be cleared by\n\nwriting all 1's into it."]
    #[inline(always)]
    #[must_use]
    pub fn sfrcer(&mut self) -> SfrcerW<PcieLmEccCorrectableErrorCountSpec> {
        SfrcerW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Replay RAM Correctable Error Count \\[RRCER\\]\n\nNumber of correctable errors\n\ndetected while reading from the\n\nReplay Buffer RAM. This is an 8- bit\n\nsaturating counter that can be\n\ncleared by writing all 1's into it."]
    #[inline(always)]
    #[must_use]
    pub fn rrcer(&mut self) -> RrcerW<PcieLmEccCorrectableErrorCountSpec> {
        RrcerW::new(self, 16)
    }
    #[doc = "Bits 24:31 - TPH ST RAM Correctable Error Count \\[R12\\]\n\nNumber of correctable errors\n\ndetected while reading from the TPH\n\nSteering Tag RAM. This is an 8-bit\n\nsaturating counter that can be\n\ncleared by writing all 1s into it."]
    #[inline(always)]
    #[must_use]
    pub fn r12(&mut self) -> R12W<PcieLmEccCorrectableErrorCountSpec> {
        R12W::new(self, 24)
    }
}
#[doc = "ECC Correctable Error Count Register\n\nNumber of correctable errors\n\ndetected while reading from the TPH\n\nSteering Tag RAM. This is an 8-bit\n\nsaturating counter that can be\n\ncleared by writing all 1s into it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_ecc_correctable_error_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_ecc_correctable_error_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmEccCorrectableErrorCountSpec;
impl crate::RegisterSpec for PcieLmEccCorrectableErrorCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_ecc_correctable_error_count::R`](R) reader structure"]
impl crate::Readable for PcieLmEccCorrectableErrorCountSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_ecc_correctable_error_count::W`](W) writer structure"]
impl crate::Writable for PcieLmEccCorrectableErrorCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets PCIE_LM_ECC_CORRECTABLE_ERROR_COUNT to value 0"]
impl crate::Resettable for PcieLmEccCorrectableErrorCountSpec {
    const RESET_VALUE: u32 = 0;
}
