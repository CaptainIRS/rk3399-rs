#[doc = "Register `PCIE_PF_LINK_CONTROL_AND_STATUS_2` reader"]
pub type R = crate::R<PciePfLinkControlAndStatus2Spec>;
#[doc = "Register `PCIE_PF_LINK_CONTROL_AND_STATUS_2` writer"]
pub type W = crate::W<PciePfLinkControlAndStatus2Spec>;
#[doc = "Field `TLS` reader - Target Link Speed \\[TLS\\]
For an upstream component, this field sets an upper limit on Link operational speed during reconfiguration. Additionally for both upstream and downstream components, this field sets the target speed when the software forces the link into Compliance mode by setting the Enter Compliance bit in this register (0001 = 2.5 GT/ s, 0010 = 5 GT/s, 0100 = 8 GT/s). The default value of this field is 0001 (2.5 GT/s) when the PCIE_GENERATION_SEL strap pins of the core are set to 0, 0010 (5 GT/s) when the strap is set to 1. These bits are STICKY."]
pub type TlsR = crate::FieldReader;
#[doc = "Field `TLS` writer - Target Link Speed \\[TLS\\]
For an upstream component, this field sets an upper limit on Link operational speed during reconfiguration. Additionally for both upstream and downstream components, this field sets the target speed when the software forces the link into Compliance mode by setting the Enter Compliance bit in this register (0001 = 2.5 GT/ s, 0010 = 5 GT/s, 0100 = 8 GT/s). The default value of this field is 0001 (2.5 GT/s) when the PCIE_GENERATION_SEL strap pins of the core are set to 0, 0010 (5 GT/s) when the strap is set to 1. These bits are STICKY."]
pub type TlsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EC` reader - Enter Compliance \\[EC\\]
into the Compliance mode. The target speed for the Compliance mode is determined by the Target Link Speed field of this register. STICKY."]
pub type EcR = crate::BitReader;
#[doc = "Field `EC` writer - Enter Compliance \\[EC\\]
into the Compliance mode. The target speed for the Compliance mode is determined by the Target Link Speed field of this register. STICKY."]
pub type EcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASD` reader - Hardware Autonomous Speed Disable \\[HASD\\]
When this bit is set, the LTSSM is prevented from changing the operating speed of the link, other than reducing the speed to correct unreliable operation of the link. STICKY"]
pub type HasdR = crate::BitReader;
#[doc = "Field `HASD` writer - Hardware Autonomous Speed Disable \\[HASD\\]
When this bit is set, the LTSSM is prevented from changing the operating speed of the link, other than reducing the speed to correct unreliable operation of the link. STICKY"]
pub type HasdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDE` reader - Selectable De- emphasis \\[SDE\\]
This bit selects the de-emphasis level when the core is operating at 5 GT/s (0 = -6 dB, 1 = -3.5 dB). This is reserved for Endpoints."]
pub type SdeR = crate::BitReader;
#[doc = "Field `TM` reader - Transmit Margin \\[TM\\]
This field is intended for debug and compliance testing purposes only. It controls the non-de- emphasized voltage level at the transmitter outputs. Its encodings are: 000: Normal operating range. 001: 800 - 1200 mV for full swing and 400 - 700 mV for half swing. 010 - 111: See PCI Express Base Specification 2.0. This field is reset to 0 when the LTSSM enters the Polling Configuration substate during link training. STICKY."]
pub type TmR = crate::FieldReader;
#[doc = "Field `TM` writer - Transmit Margin \\[TM\\]
This field is intended for debug and compliance testing purposes only. It controls the non-de- emphasized voltage level at the transmitter outputs. Its encodings are: 000: Normal operating range. 001: 800 - 1200 mV for full swing and 400 - 700 mV for half swing. 010 - 111: See PCI Express Base Specification 2.0. This field is reset to 0 when the LTSSM enters the Polling Configuration substate during link training. STICKY."]
pub type TmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EMC` reader - Enter Modified Compliance \\[EMC\\]
This field is intended for debug and compliance testing purposes only. If this bit is set to 1, the device will transmit the Modified Compliance Pattern when the LTSSM enters the Polling. Compliance substate. STICKY."]
pub type EmcR = crate::BitReader;
#[doc = "Field `EMC` writer - Enter Modified Compliance \\[EMC\\]
This field is intended for debug and compliance testing purposes only. If this bit is set to 1, the device will transmit the Modified Compliance Pattern when the LTSSM enters the Polling. Compliance substate. STICKY."]
pub type EmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Compliance SOS \\[CS\\]
When this bit is set to 1, the device will transmit SKP ordered sets between compliance patterns. STICKY."]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Compliance SOS \\[CS\\]
When this bit is set to 1, the device will transmit SKP ordered sets between compliance patterns. STICKY."]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDE` reader - Compliance De- Emphasis \\[CDE\\]
This bit sets the de-emphasis level (for 5GT/s operation) or the Transmitter Preset level (for 8 GT/s operation) when the LTSSM enters the Polling Compliance state because of software setting the Enter Compliance bit in this register. It is used only when the link is running at 5 GT/s or 8 GT/s. At 5 GT/s, the only valid setting are 0 (-6dB) and 1 (-3.5 dB). STICKY."]
pub type CdeR = crate::FieldReader;
#[doc = "Field `CDE` writer - Compliance De- Emphasis \\[CDE\\]
This bit sets the de-emphasis level (for 5GT/s operation) or the Transmitter Preset level (for 8 GT/s operation) when the LTSSM enters the Polling Compliance state because of software setting the Enter Compliance bit in this register. It is used only when the link is running at 5 GT/s or 8 GT/s. At 5 GT/s, the only valid setting are 0 (-6dB) and 1 (-3.5 dB). STICKY."]
pub type CdeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CDEL` reader - Current De- Emphasis Level \\[CDEL\\]
This status bit indicates the current operating de- emphasis level of the transmitter (0 = -6 dB, 1 = -3.5 dB).This field is undefined when link is not at Gen2 speed."]
pub type CdelR = crate::BitReader;
#[doc = "Field `R20` reader - Reserved \\[R20\\]
Reserved"]
pub type R20R = crate::FieldReader;
#[doc = "Field `R19` reader - Reserved \\[R19\\]
Reserved"]
pub type R19R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Target Link Speed \\[TLS\\]
For an upstream component, this field sets an upper limit on Link operational speed during reconfiguration. Additionally for both upstream and downstream components, this field sets the target speed when the software forces the link into Compliance mode by setting the Enter Compliance bit in this register (0001 = 2.5 GT/ s, 0010 = 5 GT/s, 0100 = 8 GT/s). The default value of this field is 0001 (2.5 GT/s) when the PCIE_GENERATION_SEL strap pins of the core are set to 0, 0010 (5 GT/s) when the strap is set to 1. These bits are STICKY."]
    #[inline(always)]
    pub fn tls(&self) -> TlsR {
        TlsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enter Compliance \\[EC\\]
into the Compliance mode. The target speed for the Compliance mode is determined by the Target Link Speed field of this register. STICKY."]
    #[inline(always)]
    pub fn ec(&self) -> EcR {
        EcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware Autonomous Speed Disable \\[HASD\\]
When this bit is set, the LTSSM is prevented from changing the operating speed of the link, other than reducing the speed to correct unreliable operation of the link. STICKY"]
    #[inline(always)]
    pub fn hasd(&self) -> HasdR {
        HasdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selectable De- emphasis \\[SDE\\]
This bit selects the de-emphasis level when the core is operating at 5 GT/s (0 = -6 dB, 1 = -3.5 dB). This is reserved for Endpoints."]
    #[inline(always)]
    pub fn sde(&self) -> SdeR {
        SdeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - Transmit Margin \\[TM\\]
This field is intended for debug and compliance testing purposes only. It controls the non-de- emphasized voltage level at the transmitter outputs. Its encodings are: 000: Normal operating range. 001: 800 - 1200 mV for full swing and 400 - 700 mV for half swing. 010 - 111: See PCI Express Base Specification 2.0. This field is reset to 0 when the LTSSM enters the Polling Configuration substate during link training. STICKY."]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - Enter Modified Compliance \\[EMC\\]
This field is intended for debug and compliance testing purposes only. If this bit is set to 1, the device will transmit the Modified Compliance Pattern when the LTSSM enters the Polling. Compliance substate. STICKY."]
    #[inline(always)]
    pub fn emc(&self) -> EmcR {
        EmcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compliance SOS \\[CS\\]
When this bit is set to 1, the device will transmit SKP ordered sets between compliance patterns. STICKY."]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Compliance De- Emphasis \\[CDE\\]
This bit sets the de-emphasis level (for 5GT/s operation) or the Transmitter Preset level (for 8 GT/s operation) when the LTSSM enters the Polling Compliance state because of software setting the Enter Compliance bit in this register. It is used only when the link is running at 5 GT/s or 8 GT/s. At 5 GT/s, the only valid setting are 0 (-6dB) and 1 (-3.5 dB). STICKY."]
    #[inline(always)]
    pub fn cde(&self) -> CdeR {
        CdeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Current De- Emphasis Level \\[CDEL\\]
This status bit indicates the current operating de- emphasis level of the transmitter (0 = -6 dB, 1 = -3.5 dB).This field is undefined when link is not at Gen2 speed."]
    #[inline(always)]
    pub fn cdel(&self) -> CdelR {
        CdelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - Reserved \\[R20\\]
Reserved"]
    #[inline(always)]
    pub fn r20(&self) -> R20R {
        R20R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:31 - Reserved \\[R19\\]
Reserved"]
    #[inline(always)]
    pub fn r19(&self) -> R19R {
        R19R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Target Link Speed \\[TLS\\]
For an upstream component, this field sets an upper limit on Link operational speed during reconfiguration. Additionally for both upstream and downstream components, this field sets the target speed when the software forces the link into Compliance mode by setting the Enter Compliance bit in this register (0001 = 2.5 GT/ s, 0010 = 5 GT/s, 0100 = 8 GT/s). The default value of this field is 0001 (2.5 GT/s) when the PCIE_GENERATION_SEL strap pins of the core are set to 0, 0010 (5 GT/s) when the strap is set to 1. These bits are STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn tls(&mut self) -> TlsW<PciePfLinkControlAndStatus2Spec> {
        TlsW::new(self, 0)
    }
    #[doc = "Bit 4 - Enter Compliance \\[EC\\]
into the Compliance mode. The target speed for the Compliance mode is determined by the Target Link Speed field of this register. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ec(&mut self) -> EcW<PciePfLinkControlAndStatus2Spec> {
        EcW::new(self, 4)
    }
    #[doc = "Bit 5 - Hardware Autonomous Speed Disable \\[HASD\\]
When this bit is set, the LTSSM is prevented from changing the operating speed of the link, other than reducing the speed to correct unreliable operation of the link. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn hasd(&mut self) -> HasdW<PciePfLinkControlAndStatus2Spec> {
        HasdW::new(self, 5)
    }
    #[doc = "Bits 7:9 - Transmit Margin \\[TM\\]
This field is intended for debug and compliance testing purposes only. It controls the non-de- emphasized voltage level at the transmitter outputs. Its encodings are: 000: Normal operating range. 001: 800 - 1200 mV for full swing and 400 - 700 mV for half swing. 010 - 111: See PCI Express Base Specification 2.0. This field is reset to 0 when the LTSSM enters the Polling Configuration substate during link training. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TmW<PciePfLinkControlAndStatus2Spec> {
        TmW::new(self, 7)
    }
    #[doc = "Bit 10 - Enter Modified Compliance \\[EMC\\]
This field is intended for debug and compliance testing purposes only. If this bit is set to 1, the device will transmit the Modified Compliance Pattern when the LTSSM enters the Polling. Compliance substate. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn emc(&mut self) -> EmcW<PciePfLinkControlAndStatus2Spec> {
        EmcW::new(self, 10)
    }
    #[doc = "Bit 11 - Compliance SOS \\[CS\\]
When this bit is set to 1, the device will transmit SKP ordered sets between compliance patterns. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<PciePfLinkControlAndStatus2Spec> {
        CsW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Compliance De- Emphasis \\[CDE\\]
This bit sets the de-emphasis level (for 5GT/s operation) or the Transmitter Preset level (for 8 GT/s operation) when the LTSSM enters the Polling Compliance state because of software setting the Enter Compliance bit in this register. It is used only when the link is running at 5 GT/s or 8 GT/s. At 5 GT/s, the only valid setting are 0 (-6dB) and 1 (-3.5 dB). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cde(&mut self) -> CdeW<PciePfLinkControlAndStatus2Spec> {
        CdeW::new(self, 12)
    }
}
#[doc = "Link Control and Status Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_link_control_and_status_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_link_control_and_status_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfLinkControlAndStatus2Spec;
impl crate::RegisterSpec for PciePfLinkControlAndStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_link_control_and_status_2::R`](R) reader structure"]
impl crate::Readable for PciePfLinkControlAndStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_link_control_and_status_2::W`](W) writer structure"]
impl crate::Writable for PciePfLinkControlAndStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_LINK_CONTROL_AND_STATUS_2 to value 0x02"]
impl crate::Resettable for PciePfLinkControlAndStatus2Spec {
    const RESET_VALUE: u32 = 0x02;
}
