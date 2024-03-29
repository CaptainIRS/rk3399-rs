#[doc = "Register `PCIE_RC_UNCORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<PcieRcUncorrectableErrorMaskSpec>;
#[doc = "Register `PCIE_RC_UNCORRECTABLE_ERROR_MASK` writer"]
pub type W = crate::W<PcieRcUncorrectableErrorMaskSpec>;
#[doc = "Field `R29` reader - Reserved \\[R29\\]\n\nReserved"]
pub type R29R = crate::FieldReader;
#[doc = "Field `DLPER` reader - Data Link Protocol Error Mask \\[DLPER\\]\n\nThis bit is set to mask the reporting\n\nof Data Link Protocol Errors.\n\nSTICKY."]
pub type DlperR = crate::BitReader;
#[doc = "Field `DLPER` writer - Data Link Protocol Error Mask \\[DLPER\\]\n\nThis bit is set to mask the reporting\n\nof Data Link Protocol Errors.\n\nSTICKY."]
pub type DlperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R30` reader - Reserved \\[R30\\]\n\nReserved"]
pub type R30R = crate::FieldReader;
#[doc = "Field `PTM` reader - Poisoned TLP Mask \\[PTM\\]\n\nThis bit is set to mask the reporting\n\nof a Poisoned TLP. STICKY."]
pub type PtmR = crate::BitReader;
#[doc = "Field `PTM` writer - Poisoned TLP Mask \\[PTM\\]\n\nThis bit is set to mask the reporting\n\nof a Poisoned TLP. STICKY."]
pub type PtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCPER` reader - Flow Control Protocol Error Mask \\[FCPER\\]\n\nThis bit is set to mask the reporting\n\nof Flow Control Protocol Errors.\n\nSTICKY."]
pub type FcperR = crate::BitReader;
#[doc = "Field `FCPER` writer - Flow Control Protocol Error Mask \\[FCPER\\]\n\nThis bit is set to mask the reporting\n\nof Flow Control Protocol Errors.\n\nSTICKY."]
pub type FcperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTM` reader - Completion Timeout Mask \\[CTM\\]\n\nThis bit is set to mask the reporting\n\nof Completion Timeouts. STICKY."]
pub type CtmR = crate::BitReader;
#[doc = "Field `CTM` writer - Completion Timeout Mask \\[CTM\\]\n\nThis bit is set to mask the reporting\n\nof Completion Timeouts. STICKY."]
pub type CtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM` reader - Completer Abort Mask \\[CAM\\]\n\nThis bit is set to mask the reporting\n\nof the core sending a Completer\n\nAbort. STICKY."]
pub type CamR = crate::BitReader;
#[doc = "Field `CAM` writer - Completer Abort Mask \\[CAM\\]\n\nThis bit is set to mask the reporting\n\nof the core sending a Completer\n\nAbort. STICKY."]
pub type CamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCM` reader - Unexpected Completion Mask \\[UCM\\]\n\nThis bit is set to mask the reporting\n\nof unexpected Completions received\n\nby the core. STICKY."]
pub type UcmR = crate::BitReader;
#[doc = "Field `UCM` writer - Unexpected Completion Mask \\[UCM\\]\n\nThis bit is set to mask the reporting\n\nof unexpected Completions received\n\nby the core. STICKY."]
pub type UcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` reader - Receiver Overflow Mask \\[ROM\\]\n\nThis bit is set to mask the reporting\n\nof violations of receive credit.\n\nSTICKY."]
pub type RomR = crate::BitReader;
#[doc = "Field `ROM` writer - Receiver Overflow Mask \\[ROM\\]\n\nThis bit is set to mask the reporting\n\nof violations of receive credit.\n\nSTICKY."]
pub type RomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTM` reader - Malformed TLP Mask \\[MTM\\]\n\nThis bit is set to mask the reporting\n\nof malformed TLPs received from\n\nthe link. STICKY."]
pub type MtmR = crate::BitReader;
#[doc = "Field `MTM` writer - Malformed TLP Mask \\[MTM\\]\n\nThis bit is set to mask the reporting\n\nof malformed TLPs received from\n\nthe link. STICKY."]
pub type MtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEM` reader - ECRC Error Mask \\[EEM\\]\n\nThis bit is set to mask the reporting\n\nof ECRC errors. STICKY."]
pub type EemR = crate::BitReader;
#[doc = "Field `EEM` writer - ECRC Error Mask \\[EEM\\]\n\nThis bit is set to mask the reporting\n\nof ECRC errors. STICKY."]
pub type EemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UREM` reader - Unsupported Request Error Mask \\[UREM\\]\n\nThis bit is set to mask the reporting\n\nof unexpected requests received\n\nfrom the link. STICKY."]
pub type UremR = crate::BitReader;
#[doc = "Field `UREM` writer - Unsupported Request Error Mask \\[UREM\\]\n\nThis bit is set to mask the reporting\n\nof unexpected requests received\n\nfrom the link. STICKY."]
pub type UremW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R31` reader - Reserved \\[R31\\]\n\nReserved"]
pub type R31R = crate::BitReader;
#[doc = "Field `UIEM` reader - Uncorrectable Internal Error Mask \\[UIEM\\]\n\nThis bit is set to mask the reporting\n\nof internal errors. STICKY."]
pub type UiemR = crate::BitReader;
#[doc = "Field `UIEM` writer - Uncorrectable Internal Error Mask \\[UIEM\\]\n\nThis bit is set to mask the reporting\n\nof internal errors. STICKY."]
pub type UiemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R32` reader - Reserved \\[R32\\]\n\nReserved"]
pub type R32R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R29\\]\n\nReserved"]
    #[inline(always)]
    pub fn r29(&self) -> R29R {
        R29R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Data Link Protocol Error Mask \\[DLPER\\]\n\nThis bit is set to mask the reporting\n\nof Data Link Protocol Errors.\n\nSTICKY."]
    #[inline(always)]
    pub fn dlper(&self) -> DlperR {
        DlperR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - Reserved \\[R30\\]\n\nReserved"]
    #[inline(always)]
    pub fn r30(&self) -> R30R {
        R30R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Poisoned TLP Mask \\[PTM\\]\n\nThis bit is set to mask the reporting\n\nof a Poisoned TLP. STICKY."]
    #[inline(always)]
    pub fn ptm(&self) -> PtmR {
        PtmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Mask \\[FCPER\\]\n\nThis bit is set to mask the reporting\n\nof Flow Control Protocol Errors.\n\nSTICKY."]
    #[inline(always)]
    pub fn fcper(&self) -> FcperR {
        FcperR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Completion Timeout Mask \\[CTM\\]\n\nThis bit is set to mask the reporting\n\nof Completion Timeouts. STICKY."]
    #[inline(always)]
    pub fn ctm(&self) -> CtmR {
        CtmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Completer Abort Mask \\[CAM\\]\n\nThis bit is set to mask the reporting\n\nof the core sending a Completer\n\nAbort. STICKY."]
    #[inline(always)]
    pub fn cam(&self) -> CamR {
        CamR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Unexpected Completion Mask \\[UCM\\]\n\nThis bit is set to mask the reporting\n\nof unexpected Completions received\n\nby the core. STICKY."]
    #[inline(always)]
    pub fn ucm(&self) -> UcmR {
        UcmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Overflow Mask \\[ROM\\]\n\nThis bit is set to mask the reporting\n\nof violations of receive credit.\n\nSTICKY."]
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Malformed TLP Mask \\[MTM\\]\n\nThis bit is set to mask the reporting\n\nof malformed TLPs received from\n\nthe link. STICKY."]
    #[inline(always)]
    pub fn mtm(&self) -> MtmR {
        MtmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ECRC Error Mask \\[EEM\\]\n\nThis bit is set to mask the reporting\n\nof ECRC errors. STICKY."]
    #[inline(always)]
    pub fn eem(&self) -> EemR {
        EemR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Unsupported Request Error Mask \\[UREM\\]\n\nThis bit is set to mask the reporting\n\nof unexpected requests received\n\nfrom the link. STICKY."]
    #[inline(always)]
    pub fn urem(&self) -> UremR {
        UremR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R31\\]\n\nReserved"]
    #[inline(always)]
    pub fn r31(&self) -> R31R {
        R31R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Mask \\[UIEM\\]\n\nThis bit is set to mask the reporting\n\nof internal errors. STICKY."]
    #[inline(always)]
    pub fn uiem(&self) -> UiemR {
        UiemR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R32\\]\n\nReserved"]
    #[inline(always)]
    pub fn r32(&self) -> R32R {
        R32R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Data Link Protocol Error Mask \\[DLPER\\]\n\nThis bit is set to mask the reporting\n\nof Data Link Protocol Errors.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn dlper(&mut self) -> DlperW<PcieRcUncorrectableErrorMaskSpec> {
        DlperW::new(self, 4)
    }
    #[doc = "Bit 12 - Poisoned TLP Mask \\[PTM\\]\n\nThis bit is set to mask the reporting\n\nof a Poisoned TLP. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ptm(&mut self) -> PtmW<PcieRcUncorrectableErrorMaskSpec> {
        PtmW::new(self, 12)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Mask \\[FCPER\\]\n\nThis bit is set to mask the reporting\n\nof Flow Control Protocol Errors.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn fcper(&mut self) -> FcperW<PcieRcUncorrectableErrorMaskSpec> {
        FcperW::new(self, 13)
    }
    #[doc = "Bit 14 - Completion Timeout Mask \\[CTM\\]\n\nThis bit is set to mask the reporting\n\nof Completion Timeouts. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ctm(&mut self) -> CtmW<PcieRcUncorrectableErrorMaskSpec> {
        CtmW::new(self, 14)
    }
    #[doc = "Bit 15 - Completer Abort Mask \\[CAM\\]\n\nThis bit is set to mask the reporting\n\nof the core sending a Completer\n\nAbort. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cam(&mut self) -> CamW<PcieRcUncorrectableErrorMaskSpec> {
        CamW::new(self, 15)
    }
    #[doc = "Bit 16 - Unexpected Completion Mask \\[UCM\\]\n\nThis bit is set to mask the reporting\n\nof unexpected Completions received\n\nby the core. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ucm(&mut self) -> UcmW<PcieRcUncorrectableErrorMaskSpec> {
        UcmW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Overflow Mask \\[ROM\\]\n\nThis bit is set to mask the reporting\n\nof violations of receive credit.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> RomW<PcieRcUncorrectableErrorMaskSpec> {
        RomW::new(self, 17)
    }
    #[doc = "Bit 18 - Malformed TLP Mask \\[MTM\\]\n\nThis bit is set to mask the reporting\n\nof malformed TLPs received from\n\nthe link. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn mtm(&mut self) -> MtmW<PcieRcUncorrectableErrorMaskSpec> {
        MtmW::new(self, 18)
    }
    #[doc = "Bit 19 - ECRC Error Mask \\[EEM\\]\n\nThis bit is set to mask the reporting\n\nof ECRC errors. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn eem(&mut self) -> EemW<PcieRcUncorrectableErrorMaskSpec> {
        EemW::new(self, 19)
    }
    #[doc = "Bit 20 - Unsupported Request Error Mask \\[UREM\\]\n\nThis bit is set to mask the reporting\n\nof unexpected requests received\n\nfrom the link. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn urem(&mut self) -> UremW<PcieRcUncorrectableErrorMaskSpec> {
        UremW::new(self, 20)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Mask \\[UIEM\\]\n\nThis bit is set to mask the reporting\n\nof internal errors. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn uiem(&mut self) -> UiemW<PcieRcUncorrectableErrorMaskSpec> {
        UiemW::new(self, 22)
    }
}
#[doc = "Uncorrectable Error Mask Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_uncorrectable_error_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_uncorrectable_error_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcUncorrectableErrorMaskSpec;
impl crate::RegisterSpec for PcieRcUncorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_uncorrectable_error_mask::R`](R) reader structure"]
impl crate::Readable for PcieRcUncorrectableErrorMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_uncorrectable_error_mask::W`](W) writer structure"]
impl crate::Writable for PcieRcUncorrectableErrorMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_UNCORRECTABLE_ERROR_MASK to value 0x0040_0000"]
impl crate::Resettable for PcieRcUncorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0x0040_0000;
}
