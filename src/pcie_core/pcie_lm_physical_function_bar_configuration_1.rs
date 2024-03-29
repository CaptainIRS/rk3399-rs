#[doc = "Register `PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_1` reader"]
pub type R = crate::R<PcieLmPhysicalFunctionBarConfiguration1Spec>;
#[doc = "Register `PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_1` writer"]
pub type W = crate::W<PcieLmPhysicalFunctionBarConfiguration1Spec>;
#[doc = "Field `BAR4A` reader - BAR 4 Aperture \\[BAR4A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 4 or 64bit BAR4-5. For 32-bit\n\nBAR 4, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB\n\nFor64-bit BAR4-5, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 =256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110= 512 MB, 10111 = 1 GB,\n\n11000 = 2 GB, 11001= 4 GB, 11010\n\n= 8 GB, 11011 = 16 GB, 11100 =32\n\nGB, 11101 = 64 GB, 11110 = 128\n\nGB, 11111 = 256 GB"]
pub type Bar4aR = crate::FieldReader;
#[doc = "Field `BAR4A` writer - BAR 4 Aperture \\[BAR4A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 4 or 64bit BAR4-5. For 32-bit\n\nBAR 4, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB\n\nFor64-bit BAR4-5, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 =256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110= 512 MB, 10111 = 1 GB,\n\n11000 = 2 GB, 11001= 4 GB, 11010\n\n= 8 GB, 11011 = 16 GB, 11100 =32\n\nGB, 11101 = 64 GB, 11110 = 128\n\nGB, 11111 = 256 GB"]
pub type Bar4aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "BAR 4 Control \\[BAR4C\\]\n\nSpecifies the configuration of BAR4.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\nValue on reset: 6"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar4c {
    #[doc = "3: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    B011 = 3,
    #[doc = "7: 64bit memory BAR, prefetchable"]
    B111 = 7,
}
impl From<Bar4c> for u8 {
    #[inline(always)]
    fn from(variant: Bar4c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bar4c {
    type Ux = u8;
}
#[doc = "Field `BAR4C` reader - BAR 4 Control \\[BAR4C\\]\n\nSpecifies the configuration of BAR4.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
pub type Bar4cR = crate::FieldReader<Bar4c>;
impl Bar4cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bar4c> {
        match self.bits {
            3 => Some(Bar4c::B011),
            7 => Some(Bar4c::B111),
            _ => None,
        }
    }
    #[doc = "Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Bar4c::B011
    }
    #[doc = "64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Bar4c::B111
    }
}
#[doc = "Field `BAR4C` writer - BAR 4 Control \\[BAR4C\\]\n\nSpecifies the configuration of BAR4.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
pub type Bar4cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bar4c>;
impl<'a, REG> Bar4cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Bar4c::B011)
    }
    #[doc = "64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Bar4c::B111)
    }
}
#[doc = "Field `BAR5A` reader - BAR 5 Aperture \\[BAR5A\\]\n\nSpecifies the aperture of the BAR 5\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 5, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
pub type Bar5aR = crate::FieldReader;
#[doc = "Field `BAR5A` writer - BAR 5 Aperture \\[BAR5A\\]\n\nSpecifies the aperture of the BAR 5\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 5, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
pub type Bar5aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BAR5C` reader - BAR 5 Control \\[BAR5C\\]\n\nSpecifies the configuration of BAR5.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
pub type Bar5cR = crate::FieldReader;
#[doc = "Field `BAR5C` writer - BAR 5 Control \\[BAR5C\\]\n\nSpecifies the configuration of BAR5.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
pub type Bar5cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R16` reader - Reserved \\[R16\\]\n\nReserved"]
pub type R16R = crate::FieldReader;
#[doc = "Field `R24` reader - Reserved \\[R24\\]\n\nReserved"]
pub type R24R = crate::FieldReader;
#[doc = "Field `ERBC` reader - Enable Resizable BAR Capability \\[ERBC\\]\n\nSetting this bit to 1 enables the\n\nResizable BAR Capability in the PCI\n\nExpress Configuration Space of the\n\nassociated Function. When the\n\nResizable BAR Capability is enabled,\n\nthe apertures of the memory BARs of\n\nthe corresponding Function are no\n\nlonger selected by the fields in this\n\nregister, but by the setting of the\n\nregisters in the Resizable BAR\n\nCapability Structure."]
pub type ErbcR = crate::BitReader;
#[doc = "Field `ERBC` writer - Enable Resizable BAR Capability \\[ERBC\\]\n\nSetting this bit to 1 enables the\n\nResizable BAR Capability in the PCI\n\nExpress Configuration Space of the\n\nassociated Function. When the\n\nResizable BAR Capability is enabled,\n\nthe apertures of the memory BARs of\n\nthe corresponding Function are no\n\nlonger selected by the fields in this\n\nregister, but by the setting of the\n\nregisters in the Resizable BAR\n\nCapability Structure."]
pub type ErbcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - BAR 4 Aperture \\[BAR4A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 4 or 64bit BAR4-5. For 32-bit\n\nBAR 4, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB\n\nFor64-bit BAR4-5, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 =256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110= 512 MB, 10111 = 1 GB,\n\n11000 = 2 GB, 11001= 4 GB, 11010\n\n= 8 GB, 11011 = 16 GB, 11100 =32\n\nGB, 11101 = 64 GB, 11110 = 128\n\nGB, 11111 = 256 GB"]
    #[inline(always)]
    pub fn bar4a(&self) -> Bar4aR {
        Bar4aR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - BAR 4 Control \\[BAR4C\\]\n\nSpecifies the configuration of BAR4.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    pub fn bar4c(&self) -> Bar4cR {
        Bar4cR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - BAR 5 Aperture \\[BAR5A\\]\n\nSpecifies the aperture of the BAR 5\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 5, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
    #[inline(always)]
    pub fn bar5a(&self) -> Bar5aR {
        Bar5aR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - BAR 5 Control \\[BAR5C\\]\n\nSpecifies the configuration of BAR5.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
    #[inline(always)]
    pub fn bar5c(&self) -> Bar5cR {
        Bar5cR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Reserved \\[R16\\]\n\nReserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Reserved \\[R24\\]\n\nReserved"]
    #[inline(always)]
    pub fn r24(&self) -> R24R {
        R24R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Enable Resizable BAR Capability \\[ERBC\\]\n\nSetting this bit to 1 enables the\n\nResizable BAR Capability in the PCI\n\nExpress Configuration Space of the\n\nassociated Function. When the\n\nResizable BAR Capability is enabled,\n\nthe apertures of the memory BARs of\n\nthe corresponding Function are no\n\nlonger selected by the fields in this\n\nregister, but by the setting of the\n\nregisters in the Resizable BAR\n\nCapability Structure."]
    #[inline(always)]
    pub fn erbc(&self) -> ErbcR {
        ErbcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - BAR 4 Aperture \\[BAR4A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 4 or 64bit BAR4-5. For 32-bit\n\nBAR 4, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB\n\nFor64-bit BAR4-5, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 =256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110= 512 MB, 10111 = 1 GB,\n\n11000 = 2 GB, 11001= 4 GB, 11010\n\n= 8 GB, 11011 = 16 GB, 11100 =32\n\nGB, 11101 = 64 GB, 11110 = 128\n\nGB, 11111 = 256 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar4a(&mut self) -> Bar4aW<PcieLmPhysicalFunctionBarConfiguration1Spec> {
        Bar4aW::new(self, 0)
    }
    #[doc = "Bits 5:7 - BAR 4 Control \\[BAR4C\\]\n\nSpecifies the configuration of BAR4.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    #[must_use]
    pub fn bar4c(&mut self) -> Bar4cW<PcieLmPhysicalFunctionBarConfiguration1Spec> {
        Bar4cW::new(self, 5)
    }
    #[doc = "Bits 8:12 - BAR 5 Aperture \\[BAR5A\\]\n\nSpecifies the aperture of the BAR 5\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 5, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar5a(&mut self) -> Bar5aW<PcieLmPhysicalFunctionBarConfiguration1Spec> {
        Bar5aW::new(self, 8)
    }
    #[doc = "Bits 13:15 - BAR 5 Control \\[BAR5C\\]\n\nSpecifies the configuration of BAR5.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
    #[inline(always)]
    #[must_use]
    pub fn bar5c(&mut self) -> Bar5cW<PcieLmPhysicalFunctionBarConfiguration1Spec> {
        Bar5cW::new(self, 13)
    }
    #[doc = "Bit 31 - Enable Resizable BAR Capability \\[ERBC\\]\n\nSetting this bit to 1 enables the\n\nResizable BAR Capability in the PCI\n\nExpress Configuration Space of the\n\nassociated Function. When the\n\nResizable BAR Capability is enabled,\n\nthe apertures of the memory BARs of\n\nthe corresponding Function are no\n\nlonger selected by the fields in this\n\nregister, but by the setting of the\n\nregisters in the Resizable BAR\n\nCapability Structure."]
    #[inline(always)]
    #[must_use]
    pub fn erbc(&mut self) -> ErbcW<PcieLmPhysicalFunctionBarConfiguration1Spec> {
        ErbcW::new(self, 31)
    }
}
#[doc = "Physical Function BAR Configuration Register 1\n\nSetting this bit to 1 enables the\n\nResizable BAR Capability in the PCI\n\nExpress Configuration Space of the\n\nassociated Function. When the\n\nResizable BAR Capability is enabled,\n\nthe apertures of the memory BARs of\n\nthe corresponding Function are no\n\nlonger selected by the fields in this\n\nregister, but by the setting of the\n\nregisters in the Resizable BAR\n\nCapability Structure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_function_bar_configuration_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_physical_function_bar_configuration_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmPhysicalFunctionBarConfiguration1Spec;
impl crate::RegisterSpec for PcieLmPhysicalFunctionBarConfiguration1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_physical_function_bar_configuration_1::R`](R) reader structure"]
impl crate::Readable for PcieLmPhysicalFunctionBarConfiguration1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_physical_function_bar_configuration_1::W`](W) writer structure"]
impl crate::Writable for PcieLmPhysicalFunctionBarConfiguration1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_1 to value 0x8fc0"]
impl crate::Resettable for PcieLmPhysicalFunctionBarConfiguration1Spec {
    const RESET_VALUE: u32 = 0x8fc0;
}
