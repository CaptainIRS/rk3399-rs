#[doc = "Register `PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2` reader"]
pub type R = crate::R<PcieRcPciExpressDeviceControlAndStatus2Spec>;
#[doc = "Register `PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2` writer"]
pub type W = crate::W<PcieRcPciExpressDeviceControlAndStatus2Spec>;
#[doc = "Field `CTV` reader - Completion Timeout Value \\[CTV\\]\n\nSpecifies the Completion Timeout\n\nvalue for the device. Allowable\n\nvalues are 0101 (sub-range 1) and\n\n0110 (sub-range 2). The\n\ncorresponding timeout values are\n\nstored in the local management\n\nregister's Completion Timeout\n\nInterval Registers 0 and 1,\n\nrespectively."]
pub type CtvR = crate::FieldReader;
#[doc = "Field `CTV` writer - Completion Timeout Value \\[CTV\\]\n\nSpecifies the Completion Timeout\n\nvalue for the device. Allowable\n\nvalues are 0101 (sub-range 1) and\n\n0110 (sub-range 2). The\n\ncorresponding timeout values are\n\nstored in the local management\n\nregister's Completion Timeout\n\nInterval Registers 0 and 1,\n\nrespectively."]
pub type CtvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTD` reader - Completion Timeout Disable \\[CTD\\]\n\nSetting this bit disables the\n\nCompletion Timeout in the device."]
pub type CtdR = crate::BitReader;
#[doc = "Field `CTD` writer - Completion Timeout Disable \\[CTD\\]\n\nSetting this bit disables the\n\nCompletion Timeout in the device."]
pub type CtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFE` reader - ARI Forwarding Enable \\[AFE\\]\n\nA 1 in this filed indicates that the\n\nport treats fields 7:0 of the ID as\n\nfunction number while converting a\n\nType 1 config packet to type 0 config\n\npacket."]
pub type AfeR = crate::BitReader;
#[doc = "Field `AFE` writer - ARI Forwarding Enable \\[AFE\\]\n\nA 1 in this filed indicates that the\n\nport treats fields 7:0 of the ID as\n\nfunction number while converting a\n\nType 1 config packet to type 0 config\n\npacket."]
pub type AfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AORE` reader - Atomic Op Requester Enable \\[AORE\\]\n\nThis bit must be set to enable the\n\ngeneration of Atomic Op Requests. If\n\nthe client logic attempts to send an\n\nAtomic Op when this bit is not set,\n\nlogic in the core will nullify the TLP\n\non its way to the link."]
pub type AoreR = crate::BitReader;
#[doc = "Field `R18` reader - Reserved \\[R18\\]\n\nReserved"]
pub type R18R = crate::BitReader;
#[doc = "Field `IRE` reader - IDO Request Enable \\[IRE\\]\n\nWhen this bit is 1, the RC is allowed\n\nto set the ID- based Ordering (IDO)\n\nAttribute bit in the requests it\n\ngenerates."]
pub type IreR = crate::BitReader;
#[doc = "Field `ICE` reader - IDO Completion Enable \\[ICE\\]\n\nWhen this bit is 1, the RC is allowed\n\nto set the ID-based Ordering (IDO)\n\nAttribute bit in the Completions it\n\ngenerates."]
pub type IceR = crate::BitReader;
#[doc = "Field `LTRME` reader - LTR Mechanism Enable \\[LTRME\\]\n\nThis must be set to 1 to enable the\n\nLatency Tolerance Reporting\n\nMechanism. This bit is implemented\n\nonly in PF 0. Its default value is 1,\n\nbut can be modified from the local\n\nmanagement bus. This bit is read-\n\nonly in PF 1."]
pub type LtrmeR = crate::BitReader;
#[doc = "Field `LTRME` writer - LTR Mechanism Enable \\[LTRME\\]\n\nThis must be set to 1 to enable the\n\nLatency Tolerance Reporting\n\nMechanism. This bit is implemented\n\nonly in PF 0. Its default value is 1,\n\nbut can be modified from the local\n\nmanagement bus. This bit is read-\n\nonly in PF 1."]
pub type LtrmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R19` reader - Reserved \\[R19\\]\n\nReserved"]
pub type R19R = crate::FieldReader;
#[doc = "Field `OBFFE` reader - OBFF Enable \\[OBFFE\\]\n\nEnables the Optimized Buffer\n\nFlush/Fill (OBFF) capability in the\n\ndevice. Valid settings are 00\n\n(disabled), 01 (Variation A), and 10\n\n(Variation B)."]
pub type ObffeR = crate::FieldReader;
#[doc = "Field `OBFFE` writer - OBFF Enable \\[OBFFE\\]\n\nEnables the Optimized Buffer\n\nFlush/Fill (OBFF) capability in the\n\ndevice. Valid settings are 00\n\n(disabled), 01 (Variation A), and 10\n\n(Variation B)."]
pub type ObffeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R20` reader - Reserved \\[R20\\]\n\n(no description)"]
pub type R20R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - Completion Timeout Value \\[CTV\\]\n\nSpecifies the Completion Timeout\n\nvalue for the device. Allowable\n\nvalues are 0101 (sub-range 1) and\n\n0110 (sub-range 2). The\n\ncorresponding timeout values are\n\nstored in the local management\n\nregister's Completion Timeout\n\nInterval Registers 0 and 1,\n\nrespectively."]
    #[inline(always)]
    pub fn ctv(&self) -> CtvR {
        CtvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Completion Timeout Disable \\[CTD\\]\n\nSetting this bit disables the\n\nCompletion Timeout in the device."]
    #[inline(always)]
    pub fn ctd(&self) -> CtdR {
        CtdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARI Forwarding Enable \\[AFE\\]\n\nA 1 in this filed indicates that the\n\nport treats fields 7:0 of the ID as\n\nfunction number while converting a\n\nType 1 config packet to type 0 config\n\npacket."]
    #[inline(always)]
    pub fn afe(&self) -> AfeR {
        AfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Atomic Op Requester Enable \\[AORE\\]\n\nThis bit must be set to enable the\n\ngeneration of Atomic Op Requests. If\n\nthe client logic attempts to send an\n\nAtomic Op when this bit is not set,\n\nlogic in the core will nullify the TLP\n\non its way to the link."]
    #[inline(always)]
    pub fn aore(&self) -> AoreR {
        AoreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved \\[R18\\]\n\nReserved"]
    #[inline(always)]
    pub fn r18(&self) -> R18R {
        R18R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IDO Request Enable \\[IRE\\]\n\nWhen this bit is 1, the RC is allowed\n\nto set the ID- based Ordering (IDO)\n\nAttribute bit in the requests it\n\ngenerates."]
    #[inline(always)]
    pub fn ire(&self) -> IreR {
        IreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDO Completion Enable \\[ICE\\]\n\nWhen this bit is 1, the RC is allowed\n\nto set the ID-based Ordering (IDO)\n\nAttribute bit in the Completions it\n\ngenerates."]
    #[inline(always)]
    pub fn ice(&self) -> IceR {
        IceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LTR Mechanism Enable \\[LTRME\\]\n\nThis must be set to 1 to enable the\n\nLatency Tolerance Reporting\n\nMechanism. This bit is implemented\n\nonly in PF 0. Its default value is 1,\n\nbut can be modified from the local\n\nmanagement bus. This bit is read-\n\nonly in PF 1."]
    #[inline(always)]
    pub fn ltrme(&self) -> LtrmeR {
        LtrmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Reserved \\[R19\\]\n\nReserved"]
    #[inline(always)]
    pub fn r19(&self) -> R19R {
        R19R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - OBFF Enable \\[OBFFE\\]\n\nEnables the Optimized Buffer\n\nFlush/Fill (OBFF) capability in the\n\ndevice. Valid settings are 00\n\n(disabled), 01 (Variation A), and 10\n\n(Variation B)."]
    #[inline(always)]
    pub fn obffe(&self) -> ObffeR {
        ObffeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:31 - Reserved \\[R20\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r20(&self) -> R20R {
        R20R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Completion Timeout Value \\[CTV\\]\n\nSpecifies the Completion Timeout\n\nvalue for the device. Allowable\n\nvalues are 0101 (sub-range 1) and\n\n0110 (sub-range 2). The\n\ncorresponding timeout values are\n\nstored in the local management\n\nregister's Completion Timeout\n\nInterval Registers 0 and 1,\n\nrespectively."]
    #[inline(always)]
    #[must_use]
    pub fn ctv(&mut self) -> CtvW<PcieRcPciExpressDeviceControlAndStatus2Spec> {
        CtvW::new(self, 0)
    }
    #[doc = "Bit 4 - Completion Timeout Disable \\[CTD\\]\n\nSetting this bit disables the\n\nCompletion Timeout in the device."]
    #[inline(always)]
    #[must_use]
    pub fn ctd(&mut self) -> CtdW<PcieRcPciExpressDeviceControlAndStatus2Spec> {
        CtdW::new(self, 4)
    }
    #[doc = "Bit 5 - ARI Forwarding Enable \\[AFE\\]\n\nA 1 in this filed indicates that the\n\nport treats fields 7:0 of the ID as\n\nfunction number while converting a\n\nType 1 config packet to type 0 config\n\npacket."]
    #[inline(always)]
    #[must_use]
    pub fn afe(&mut self) -> AfeW<PcieRcPciExpressDeviceControlAndStatus2Spec> {
        AfeW::new(self, 5)
    }
    #[doc = "Bit 10 - LTR Mechanism Enable \\[LTRME\\]\n\nThis must be set to 1 to enable the\n\nLatency Tolerance Reporting\n\nMechanism. This bit is implemented\n\nonly in PF 0. Its default value is 1,\n\nbut can be modified from the local\n\nmanagement bus. This bit is read-\n\nonly in PF 1."]
    #[inline(always)]
    #[must_use]
    pub fn ltrme(&mut self) -> LtrmeW<PcieRcPciExpressDeviceControlAndStatus2Spec> {
        LtrmeW::new(self, 10)
    }
    #[doc = "Bits 13:14 - OBFF Enable \\[OBFFE\\]\n\nEnables the Optimized Buffer\n\nFlush/Fill (OBFF) capability in the\n\ndevice. Valid settings are 00\n\n(disabled), 01 (Variation A), and 10\n\n(Variation B)."]
    #[inline(always)]
    #[must_use]
    pub fn obffe(&mut self) -> ObffeW<PcieRcPciExpressDeviceControlAndStatus2Spec> {
        ObffeW::new(self, 13)
    }
}
#[doc = "PCI Express Device Control and Status 2 Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_device_control_and_status_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_pci_express_device_control_and_status_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcPciExpressDeviceControlAndStatus2Spec;
impl crate::RegisterSpec for PcieRcPciExpressDeviceControlAndStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_pci_express_device_control_and_status_2::R`](R) reader structure"]
impl crate::Readable for PcieRcPciExpressDeviceControlAndStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_pci_express_device_control_and_status_2::W`](W) writer structure"]
impl crate::Writable for PcieRcPciExpressDeviceControlAndStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2 to value 0"]
impl crate::Resettable for PcieRcPciExpressDeviceControlAndStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
