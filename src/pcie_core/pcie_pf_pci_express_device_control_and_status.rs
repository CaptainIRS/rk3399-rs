#[doc = "Register `PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PciePfPciExpressDeviceControlAndStatusSpec>;
#[doc = "Register `PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PciePfPciExpressDeviceControlAndStatusSpec>;
#[doc = "Field `ECER` reader - Enable Correctable Error Reporting \\[ECER\\]
Enables the sending of ERR_COR messages by the core on the detection of correctable errors."]
pub type EcerR = crate::BitReader;
#[doc = "Field `ECER` writer - Enable Correctable Error Reporting \\[ECER\\]
Enables the sending of ERR_COR messages by the core on the detection of correctable errors."]
pub type EcerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENFER` reader - Enable Non- Fatal Error Reporting \\[ENFER\\]
Enables the sending of ERR_NONFATAL messages by the core on the detection of non-fatal errors."]
pub type EnferR = crate::BitReader;
#[doc = "Field `ENFER` writer - Enable Non- Fatal Error Reporting \\[ENFER\\]
Enables the sending of ERR_NONFATAL messages by the core on the detection of non-fatal errors."]
pub type EnferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFER` reader - Enable Fatal Error Reporting \\[EFER\\]
Enables the sending of ERR_FATAL messages by the core on the detection of fatal errors."]
pub type EferR = crate::BitReader;
#[doc = "Field `EFER` writer - Enable Fatal Error Reporting \\[EFER\\]
Enables the sending of ERR_FATAL messages by the core on the detection of fatal errors."]
pub type EferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EURR` reader - Enable Unsupported Request Reporting \\[EURR\\]
Enables the sending of error messages by the core on receiving unsupported requests."]
pub type EurrR = crate::BitReader;
#[doc = "Field `EURR` writer - Enable Unsupported Request Reporting \\[EURR\\]
Enables the sending of error messages by the core on receiving unsupported requests."]
pub type EurrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERO` reader - Enable Relaxed Ordering \\[ERO\\]
When set, this bit indicates that the device is allowed to set the Relaxed Ordering bit in the Attributes field of transactions initiated from it, when the transactions do not require Strong Ordering."]
pub type EroR = crate::BitReader;
#[doc = "Field `ERO` writer - Enable Relaxed Ordering \\[ERO\\]
When set, this bit indicates that the device is allowed to set the Relaxed Ordering bit in the Attributes field of transactions initiated from it, when the transactions do not require Strong Ordering."]
pub type EroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPS` reader - Max Payload Size \\[MPS\\]
Specifies the maximum TLP payload size configured. The device must be able to receive a TLP of this maximum size, and should not generate TLPs larger than this value. The configuration program sets this field based on the maximum payload size in the Device Capabilities Register, and the capability of the other side."]
pub type MpsR = crate::FieldReader;
#[doc = "Field `MPS` writer - Max Payload Size \\[MPS\\]
Specifies the maximum TLP payload size configured. The device must be able to receive a TLP of this maximum size, and should not generate TLPs larger than this value. The configuration program sets this field based on the maximum payload size in the Device Capabilities Register, and the capability of the other side."]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETFE` reader - Extended Tag Field Enable \\[ETFE\\]
Enables the extension of the tag field from 5 to 8 bits."]
pub type EtfeR = crate::BitReader;
#[doc = "Field `EPH` reader - Enable Phantom Functions \\[EPH\\]
This field is hardwired to 0 as the core does not support this feature."]
pub type EphR = crate::BitReader;
#[doc = "Field `EAP` reader - Enable Aux Power \\[EAP\\]
Used only when device used aux power. This field is hardwired to 0."]
pub type EapR = crate::BitReader;
#[doc = "Field `ENS` reader - Enable No Snoop \\[ENS\\]
When set to 1, the device is allowed to set the No Snoop bit in initiated transactions in which cache coherency is not needed."]
pub type EnsR = crate::BitReader;
#[doc = "Field `ENS` writer - Enable No Snoop \\[ENS\\]
When set to 1, the device is allowed to set the No Snoop bit in initiated transactions in which cache coherency is not needed."]
pub type EnsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRRS` reader - Max Read Request Size \\[MRRS\\]
Specifies the maximum size allowed in read requests generated by the device."]
pub type MrrsR = crate::FieldReader;
#[doc = "Field `MRRS` writer - Max Read Request Size \\[MRRS\\]
Specifies the maximum size allowed in read requests generated by the device."]
pub type MrrsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLR` reader - Function- Level Reset \\[FLR\\]
Writing a 1 into this bit position generates a Function-Level Reset for the selected Function. This bit reads as 0."]
pub type FlrR = crate::BitReader;
#[doc = "Field `FLR` writer - Function- Level Reset \\[FLR\\]
Writing a 1 into this bit position generates a Function-Level Reset for the selected Function. This bit reads as 0."]
pub type FlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CED` reader - Correctable Error Detected \\[CED\\]
Set to 1 by the core when it detects a correctable error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type CedR = crate::BitReader;
#[doc = "Field `CED` writer - Correctable Error Detected \\[CED\\]
Set to 1 by the core when it detects a correctable error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type CedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NFED` reader - Non-Fatal Error Detected \\[NFED\\]
Set to 1 by the core when it detects a non-fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type NfedR = crate::BitReader;
#[doc = "Field `NFED` writer - Non-Fatal Error Detected \\[NFED\\]
Set to 1 by the core when it detects a non-fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type NfedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FED` reader - Fatal Error Detected \\[FED\\]
Set to 1 by the core when it detects a fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type FedR = crate::BitReader;
#[doc = "Field `FED` writer - Fatal Error Detected \\[FED\\]
Set to 1 by the core when it detects a fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type FedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `URD` reader - Unsupported Request Detected \\[URD\\]
Set to 1 by the core when it receives an unsupported request, regardless of whether its reporting is enabled or not."]
pub type UrdR = crate::BitReader;
#[doc = "Field `URD` writer - Unsupported Request Detected \\[URD\\]
Set to 1 by the core when it receives an unsupported request, regardless of whether its reporting is enabled or not."]
pub type UrdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APD` reader - Aux Power Detected \\[APD\\]
Set when auxiliary power is detected by the device. This is an unused field."]
pub type ApdR = crate::BitReader;
#[doc = "Field `TP` reader - Transaction Pending \\[TP\\]
Indicates if any of the Non-Posted requests issued by the Function are still pending."]
pub type TpR = crate::BitReader;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable Correctable Error Reporting \\[ECER\\]
Enables the sending of ERR_COR messages by the core on the detection of correctable errors."]
    #[inline(always)]
    pub fn ecer(&self) -> EcerR {
        EcerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Non- Fatal Error Reporting \\[ENFER\\]
Enables the sending of ERR_NONFATAL messages by the core on the detection of non-fatal errors."]
    #[inline(always)]
    pub fn enfer(&self) -> EnferR {
        EnferR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Fatal Error Reporting \\[EFER\\]
Enables the sending of ERR_FATAL messages by the core on the detection of fatal errors."]
    #[inline(always)]
    pub fn efer(&self) -> EferR {
        EferR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Unsupported Request Reporting \\[EURR\\]
Enables the sending of error messages by the core on receiving unsupported requests."]
    #[inline(always)]
    pub fn eurr(&self) -> EurrR {
        EurrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Relaxed Ordering \\[ERO\\]
When set, this bit indicates that the device is allowed to set the Relaxed Ordering bit in the Attributes field of transactions initiated from it, when the transactions do not require Strong Ordering."]
    #[inline(always)]
    pub fn ero(&self) -> EroR {
        EroR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max Payload Size \\[MPS\\]
Specifies the maximum TLP payload size configured. The device must be able to receive a TLP of this maximum size, and should not generate TLPs larger than this value. The configuration program sets this field based on the maximum payload size in the Device Capabilities Register, and the capability of the other side."]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Extended Tag Field Enable \\[ETFE\\]
Enables the extension of the tag field from 5 to 8 bits."]
    #[inline(always)]
    pub fn etfe(&self) -> EtfeR {
        EtfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Phantom Functions \\[EPH\\]
This field is hardwired to 0 as the core does not support this feature."]
    #[inline(always)]
    pub fn eph(&self) -> EphR {
        EphR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Aux Power \\[EAP\\]
Used only when device used aux power. This field is hardwired to 0."]
    #[inline(always)]
    pub fn eap(&self) -> EapR {
        EapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable No Snoop \\[ENS\\]
When set to 1, the device is allowed to set the No Snoop bit in initiated transactions in which cache coherency is not needed."]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Max Read Request Size \\[MRRS\\]
Specifies the maximum size allowed in read requests generated by the device."]
    #[inline(always)]
    pub fn mrrs(&self) -> MrrsR {
        MrrsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Function- Level Reset \\[FLR\\]
Writing a 1 into this bit position generates a Function-Level Reset for the selected Function. This bit reads as 0."]
    #[inline(always)]
    pub fn flr(&self) -> FlrR {
        FlrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Correctable Error Detected \\[CED\\]
Set to 1 by the core when it detects a correctable error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    pub fn ced(&self) -> CedR {
        CedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Non-Fatal Error Detected \\[NFED\\]
Set to 1 by the core when it detects a non-fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    pub fn nfed(&self) -> NfedR {
        NfedR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fatal Error Detected \\[FED\\]
Set to 1 by the core when it detects a fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    pub fn fed(&self) -> FedR {
        FedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Unsupported Request Detected \\[URD\\]
Set to 1 by the core when it receives an unsupported request, regardless of whether its reporting is enabled or not."]
    #[inline(always)]
    pub fn urd(&self) -> UrdR {
        UrdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Aux Power Detected \\[APD\\]
Set when auxiliary power is detected by the device. This is an unused field."]
    #[inline(always)]
    pub fn apd(&self) -> ApdR {
        ApdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transaction Pending \\[TP\\]
Indicates if any of the Non-Posted requests issued by the Function are still pending."]
    #[inline(always)]
    pub fn tp(&self) -> TpR {
        TpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Correctable Error Reporting \\[ECER\\]
Enables the sending of ERR_COR messages by the core on the detection of correctable errors."]
    #[inline(always)]
    #[must_use]
    pub fn ecer(&mut self) -> EcerW<PciePfPciExpressDeviceControlAndStatusSpec> {
        EcerW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Non- Fatal Error Reporting \\[ENFER\\]
Enables the sending of ERR_NONFATAL messages by the core on the detection of non-fatal errors."]
    #[inline(always)]
    #[must_use]
    pub fn enfer(&mut self) -> EnferW<PciePfPciExpressDeviceControlAndStatusSpec> {
        EnferW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Fatal Error Reporting \\[EFER\\]
Enables the sending of ERR_FATAL messages by the core on the detection of fatal errors."]
    #[inline(always)]
    #[must_use]
    pub fn efer(&mut self) -> EferW<PciePfPciExpressDeviceControlAndStatusSpec> {
        EferW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Unsupported Request Reporting \\[EURR\\]
Enables the sending of error messages by the core on receiving unsupported requests."]
    #[inline(always)]
    #[must_use]
    pub fn eurr(&mut self) -> EurrW<PciePfPciExpressDeviceControlAndStatusSpec> {
        EurrW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Relaxed Ordering \\[ERO\\]
When set, this bit indicates that the device is allowed to set the Relaxed Ordering bit in the Attributes field of transactions initiated from it, when the transactions do not require Strong Ordering."]
    #[inline(always)]
    #[must_use]
    pub fn ero(&mut self) -> EroW<PciePfPciExpressDeviceControlAndStatusSpec> {
        EroW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Max Payload Size \\[MPS\\]
Specifies the maximum TLP payload size configured. The device must be able to receive a TLP of this maximum size, and should not generate TLPs larger than this value. The configuration program sets this field based on the maximum payload size in the Device Capabilities Register, and the capability of the other side."]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MpsW<PciePfPciExpressDeviceControlAndStatusSpec> {
        MpsW::new(self, 5)
    }
    #[doc = "Bit 11 - Enable No Snoop \\[ENS\\]
When set to 1, the device is allowed to set the No Snoop bit in initiated transactions in which cache coherency is not needed."]
    #[inline(always)]
    #[must_use]
    pub fn ens(&mut self) -> EnsW<PciePfPciExpressDeviceControlAndStatusSpec> {
        EnsW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Max Read Request Size \\[MRRS\\]
Specifies the maximum size allowed in read requests generated by the device."]
    #[inline(always)]
    #[must_use]
    pub fn mrrs(&mut self) -> MrrsW<PciePfPciExpressDeviceControlAndStatusSpec> {
        MrrsW::new(self, 12)
    }
    #[doc = "Bit 15 - Function- Level Reset \\[FLR\\]
Writing a 1 into this bit position generates a Function-Level Reset for the selected Function. This bit reads as 0."]
    #[inline(always)]
    #[must_use]
    pub fn flr(&mut self) -> FlrW<PciePfPciExpressDeviceControlAndStatusSpec> {
        FlrW::new(self, 15)
    }
    #[doc = "Bit 16 - Correctable Error Detected \\[CED\\]
Set to 1 by the core when it detects a correctable error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CedW<PciePfPciExpressDeviceControlAndStatusSpec> {
        CedW::new(self, 16)
    }
    #[doc = "Bit 17 - Non-Fatal Error Detected \\[NFED\\]
Set to 1 by the core when it detects a non-fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn nfed(&mut self) -> NfedW<PciePfPciExpressDeviceControlAndStatusSpec> {
        NfedW::new(self, 17)
    }
    #[doc = "Bit 18 - Fatal Error Detected \\[FED\\]
Set to 1 by the core when it detects a fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn fed(&mut self) -> FedW<PciePfPciExpressDeviceControlAndStatusSpec> {
        FedW::new(self, 18)
    }
    #[doc = "Bit 19 - Unsupported Request Detected \\[URD\\]
Set to 1 by the core when it receives an unsupported request, regardless of whether its reporting is enabled or not."]
    #[inline(always)]
    #[must_use]
    pub fn urd(&mut self) -> UrdW<PciePfPciExpressDeviceControlAndStatusSpec> {
        UrdW::new(self, 19)
    }
}
#[doc = "PCI Express Device Control and Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_device_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_pci_express_device_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfPciExpressDeviceControlAndStatusSpec;
impl crate::RegisterSpec for PciePfPciExpressDeviceControlAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_pci_express_device_control_and_status::R`](R) reader structure"]
impl crate::Readable for PciePfPciExpressDeviceControlAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_pci_express_device_control_and_status::W`](W) writer structure"]
impl crate::Writable for PciePfPciExpressDeviceControlAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_0000;
}
#[doc = "`reset()` method sets PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS to value 0x2810"]
impl crate::Resettable for PciePfPciExpressDeviceControlAndStatusSpec {
    const RESET_VALUE: u32 = 0x2810;
}