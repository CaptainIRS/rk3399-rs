#[doc = "Register `PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PcieRcPciExpressDeviceControlAndStatusSpec>;
#[doc = "Register `PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PcieRcPciExpressDeviceControlAndStatusSpec>;
#[doc = "Field `ECER` reader - Enable Correctable Error Reporting \\[ECER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
pub type EcerR = crate::BitReader;
#[doc = "Field `ECER` writer - Enable Correctable Error Reporting \\[ECER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
pub type EcerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENFER` reader - Enable Non- Fatal Error Reporting \\[ENFER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
pub type EnferR = crate::BitReader;
#[doc = "Field `ENFER` writer - Enable Non- Fatal Error Reporting \\[ENFER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
pub type EnferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFER` reader - Enable Fatal Error Reporting \\[EFER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
pub type EferR = crate::BitReader;
#[doc = "Field `EFER` writer - Enable Fatal Error Reporting \\[EFER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
pub type EferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EURR` reader - Enable Unsupported Request Reporting \\[EURR\\]\n\nEnables the sending of error\n\nmessages by the core on receiving\n\nunsupported requests."]
pub type EurrR = crate::BitReader;
#[doc = "Field `EURR` writer - Enable Unsupported Request Reporting \\[EURR\\]\n\nEnables the sending of error\n\nmessages by the core on receiving\n\nunsupported requests."]
pub type EurrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERO` reader - Enable Relaxed Ordering \\[ERO\\]\n\nWhen set, this bit indicates that the\n\ndevice is allowed to set the Relaxed\n\nOrdering bit in the Attributes field of\n\ntransactions initiated from it. when\n\nthe transactions do not require\n\nStrong Ordering."]
pub type EroR = crate::BitReader;
#[doc = "Field `ERO` writer - Enable Relaxed Ordering \\[ERO\\]\n\nWhen set, this bit indicates that the\n\ndevice is allowed to set the Relaxed\n\nOrdering bit in the Attributes field of\n\ntransactions initiated from it. when\n\nthe transactions do not require\n\nStrong Ordering."]
pub type EroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MP` reader - Max Payload Size \\[MP\\]\n\nSpecifies the maximum TLP payload\n\nsize configured. The device must be\n\nable to receive a TLP of this\n\nmaximum size, and should not\n\ngenerate TLP's larger than this\n\nvalue. Software must set this field\n\nbased on the maximum payload size\n\nin the Device Capabilities Register,\n\nand the capability of the other side."]
pub type MpR = crate::FieldReader;
#[doc = "Field `MP` writer - Max Payload Size \\[MP\\]\n\nSpecifies the maximum TLP payload\n\nsize configured. The device must be\n\nable to receive a TLP of this\n\nmaximum size, and should not\n\ngenerate TLP's larger than this\n\nvalue. Software must set this field\n\nbased on the maximum payload size\n\nin the Device Capabilities Register,\n\nand the capability of the other side."]
pub type MpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETE` reader - Extended Tag Enable \\[ETE\\]\n\nextended tag not enabled. Hence\n\nhard coded to zero ."]
pub type EteR = crate::BitReader;
#[doc = "Field `PFE` reader - phantum functions enable \\[PFE\\]\n\nHardwired to 0"]
pub type PfeR = crate::BitReader;
#[doc = "Field `APPME` reader - aux power PM enable \\[APPME\\]\n\nHardwired to 0"]
pub type AppmeR = crate::BitReader;
#[doc = "Field `ENS` reader - Enable no snoop \\[ENS\\]\n\nIf this bit is Set, the Function is\n\npermitted to Set the No Snoop bit\n\nin the Requester Attributes of\n\ntransactions it initiates that do not\n\nrequire hardware enforced cache\n\ncoherency."]
pub type EnsR = crate::BitReader;
#[doc = "Field `ENS` writer - Enable no snoop \\[ENS\\]\n\nIf this bit is Set, the Function is\n\npermitted to Set the No Snoop bit\n\nin the Requester Attributes of\n\ntransactions it initiates that do not\n\nrequire hardware enforced cache\n\ncoherency."]
pub type EnsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRR` reader - Max Read Request Size \\[MRR\\]\n\nSpecifies the maximum size allowed\n\nin read requests generated by the\n\ndevice."]
pub type MrrR = crate::FieldReader;
#[doc = "Field `MRR` writer - Max Read Request Size \\[MRR\\]\n\nSpecifies the maximum size allowed\n\nin read requests generated by the\n\ndevice."]
pub type MrrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R7` reader - Reserved \\[R7\\]\n\nHardwired to 0."]
pub type R7R = crate::BitReader;
#[doc = "Field `CED` reader - Correctable Error Detected \\[CED\\]\n\nSet to 1 by the core when it detects\n\na correctable error, regardless of\n\nwhether the error is masked."]
pub type CedR = crate::BitReader;
#[doc = "Field `CED` writer - Correctable Error Detected \\[CED\\]\n\nSet to 1 by the core when it detects\n\na correctable error, regardless of\n\nwhether the error is masked."]
pub type CedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NFED` reader - Non-Fatal Error Detected \\[NFED\\]\n\nSet to 1 by the core when it detects\n\na non-fatal error, regardless of\n\nwhether the error is masked."]
pub type NfedR = crate::BitReader;
#[doc = "Field `NFED` writer - Non-Fatal Error Detected \\[NFED\\]\n\nSet to 1 by the core when it detects\n\na non-fatal error, regardless of\n\nwhether the error is masked."]
pub type NfedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FED` reader - Fatal Error Detected \\[FED\\]\n\nSet to 1 by the core when it detects\n\na fatal error, regardless of whether\n\nthe error is masked."]
pub type FedR = crate::BitReader;
#[doc = "Field `FED` writer - Fatal Error Detected \\[FED\\]\n\nSet to 1 by the core when it detects\n\na fatal error, regardless of whether\n\nthe error is masked."]
pub type FedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `URD` reader - Unsupported Request Detected \\[URD\\]\n\nSet to 1 by the core when it receives\n\nan unsupported request."]
pub type UrdR = crate::BitReader;
#[doc = "Field `URD` writer - Unsupported Request Detected \\[URD\\]\n\nSet to 1 by the core when it receives\n\nan unsupported request."]
pub type UrdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APD` reader - Aux Power Detected \\[APD\\]\n\nSet when auxiliary power is detected\n\nby the device. This is an unused\n\nfield."]
pub type ApdR = crate::BitReader;
#[doc = "Field `TP` reader - Transaction Pending \\[TP\\]\n\nIndicates if any of the Non-Posted\n\nrequests issued by the RC are still\n\npending."]
pub type TpR = crate::BitReader;
#[doc = "Field `R8` reader - Reserved \\[R8\\]\n\n(no description)"]
pub type R8R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable Correctable Error Reporting \\[ECER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
    #[inline(always)]
    pub fn ecer(&self) -> EcerR {
        EcerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Non- Fatal Error Reporting \\[ENFER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
    #[inline(always)]
    pub fn enfer(&self) -> EnferR {
        EnferR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Fatal Error Reporting \\[EFER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
    #[inline(always)]
    pub fn efer(&self) -> EferR {
        EferR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Unsupported Request Reporting \\[EURR\\]\n\nEnables the sending of error\n\nmessages by the core on receiving\n\nunsupported requests."]
    #[inline(always)]
    pub fn eurr(&self) -> EurrR {
        EurrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Relaxed Ordering \\[ERO\\]\n\nWhen set, this bit indicates that the\n\ndevice is allowed to set the Relaxed\n\nOrdering bit in the Attributes field of\n\ntransactions initiated from it. when\n\nthe transactions do not require\n\nStrong Ordering."]
    #[inline(always)]
    pub fn ero(&self) -> EroR {
        EroR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max Payload Size \\[MP\\]\n\nSpecifies the maximum TLP payload\n\nsize configured. The device must be\n\nable to receive a TLP of this\n\nmaximum size, and should not\n\ngenerate TLP's larger than this\n\nvalue. Software must set this field\n\nbased on the maximum payload size\n\nin the Device Capabilities Register,\n\nand the capability of the other side."]
    #[inline(always)]
    pub fn mp(&self) -> MpR {
        MpR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Extended Tag Enable \\[ETE\\]\n\nextended tag not enabled. Hence\n\nhard coded to zero ."]
    #[inline(always)]
    pub fn ete(&self) -> EteR {
        EteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - phantum functions enable \\[PFE\\]\n\nHardwired to 0"]
    #[inline(always)]
    pub fn pfe(&self) -> PfeR {
        PfeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aux power PM enable \\[APPME\\]\n\nHardwired to 0"]
    #[inline(always)]
    pub fn appme(&self) -> AppmeR {
        AppmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable no snoop \\[ENS\\]\n\nIf this bit is Set, the Function is\n\npermitted to Set the No Snoop bit\n\nin the Requester Attributes of\n\ntransactions it initiates that do not\n\nrequire hardware enforced cache\n\ncoherency."]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Max Read Request Size \\[MRR\\]\n\nSpecifies the maximum size allowed\n\nin read requests generated by the\n\ndevice."]
    #[inline(always)]
    pub fn mrr(&self) -> MrrR {
        MrrR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Reserved \\[R7\\]\n\nHardwired to 0."]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Correctable Error Detected \\[CED\\]\n\nSet to 1 by the core when it detects\n\na correctable error, regardless of\n\nwhether the error is masked."]
    #[inline(always)]
    pub fn ced(&self) -> CedR {
        CedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Non-Fatal Error Detected \\[NFED\\]\n\nSet to 1 by the core when it detects\n\na non-fatal error, regardless of\n\nwhether the error is masked."]
    #[inline(always)]
    pub fn nfed(&self) -> NfedR {
        NfedR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fatal Error Detected \\[FED\\]\n\nSet to 1 by the core when it detects\n\na fatal error, regardless of whether\n\nthe error is masked."]
    #[inline(always)]
    pub fn fed(&self) -> FedR {
        FedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Unsupported Request Detected \\[URD\\]\n\nSet to 1 by the core when it receives\n\nan unsupported request."]
    #[inline(always)]
    pub fn urd(&self) -> UrdR {
        UrdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Aux Power Detected \\[APD\\]\n\nSet when auxiliary power is detected\n\nby the device. This is an unused\n\nfield."]
    #[inline(always)]
    pub fn apd(&self) -> ApdR {
        ApdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transaction Pending \\[TP\\]\n\nIndicates if any of the Non-Posted\n\nrequests issued by the RC are still\n\npending."]
    #[inline(always)]
    pub fn tp(&self) -> TpR {
        TpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - Reserved \\[R8\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Correctable Error Reporting \\[ECER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
    #[inline(always)]
    #[must_use]
    pub fn ecer(&mut self) -> EcerW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        EcerW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Non- Fatal Error Reporting \\[ENFER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
    #[inline(always)]
    #[must_use]
    pub fn enfer(&mut self) -> EnferW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        EnferW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Fatal Error Reporting \\[EFER\\]\n\nThis bit is not used by the core in\n\nRoot Port mode."]
    #[inline(always)]
    #[must_use]
    pub fn efer(&mut self) -> EferW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        EferW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Unsupported Request Reporting \\[EURR\\]\n\nEnables the sending of error\n\nmessages by the core on receiving\n\nunsupported requests."]
    #[inline(always)]
    #[must_use]
    pub fn eurr(&mut self) -> EurrW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        EurrW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Relaxed Ordering \\[ERO\\]\n\nWhen set, this bit indicates that the\n\ndevice is allowed to set the Relaxed\n\nOrdering bit in the Attributes field of\n\ntransactions initiated from it. when\n\nthe transactions do not require\n\nStrong Ordering."]
    #[inline(always)]
    #[must_use]
    pub fn ero(&mut self) -> EroW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        EroW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Max Payload Size \\[MP\\]\n\nSpecifies the maximum TLP payload\n\nsize configured. The device must be\n\nable to receive a TLP of this\n\nmaximum size, and should not\n\ngenerate TLP's larger than this\n\nvalue. Software must set this field\n\nbased on the maximum payload size\n\nin the Device Capabilities Register,\n\nand the capability of the other side."]
    #[inline(always)]
    #[must_use]
    pub fn mp(&mut self) -> MpW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        MpW::new(self, 5)
    }
    #[doc = "Bit 11 - Enable no snoop \\[ENS\\]\n\nIf this bit is Set, the Function is\n\npermitted to Set the No Snoop bit\n\nin the Requester Attributes of\n\ntransactions it initiates that do not\n\nrequire hardware enforced cache\n\ncoherency."]
    #[inline(always)]
    #[must_use]
    pub fn ens(&mut self) -> EnsW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        EnsW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Max Read Request Size \\[MRR\\]\n\nSpecifies the maximum size allowed\n\nin read requests generated by the\n\ndevice."]
    #[inline(always)]
    #[must_use]
    pub fn mrr(&mut self) -> MrrW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        MrrW::new(self, 12)
    }
    #[doc = "Bit 16 - Correctable Error Detected \\[CED\\]\n\nSet to 1 by the core when it detects\n\na correctable error, regardless of\n\nwhether the error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CedW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        CedW::new(self, 16)
    }
    #[doc = "Bit 17 - Non-Fatal Error Detected \\[NFED\\]\n\nSet to 1 by the core when it detects\n\na non-fatal error, regardless of\n\nwhether the error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn nfed(&mut self) -> NfedW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        NfedW::new(self, 17)
    }
    #[doc = "Bit 18 - Fatal Error Detected \\[FED\\]\n\nSet to 1 by the core when it detects\n\na fatal error, regardless of whether\n\nthe error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn fed(&mut self) -> FedW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        FedW::new(self, 18)
    }
    #[doc = "Bit 19 - Unsupported Request Detected \\[URD\\]\n\nSet to 1 by the core when it receives\n\nan unsupported request."]
    #[inline(always)]
    #[must_use]
    pub fn urd(&mut self) -> UrdW<PcieRcPciExpressDeviceControlAndStatusSpec> {
        UrdW::new(self, 19)
    }
}
#[doc = "PCI Express Device Control and Status Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_device_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_pci_express_device_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcPciExpressDeviceControlAndStatusSpec;
impl crate::RegisterSpec for PcieRcPciExpressDeviceControlAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_pci_express_device_control_and_status::R`](R) reader structure"]
impl crate::Readable for PcieRcPciExpressDeviceControlAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_pci_express_device_control_and_status::W`](W) writer structure"]
impl crate::Writable for PcieRcPciExpressDeviceControlAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_0000;
}
#[doc = "`reset()` method sets PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS to value 0x2810"]
impl crate::Resettable for PcieRcPciExpressDeviceControlAndStatusSpec {
    const RESET_VALUE: u32 = 0x2810;
}
