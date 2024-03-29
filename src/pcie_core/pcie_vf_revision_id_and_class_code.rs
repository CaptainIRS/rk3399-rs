#[doc = "Register `PCIE_VF_REVISION_ID_AND_CLASS_CODE` reader"]
pub type R = crate::R<PcieVfRevisionIdAndClassCodeSpec>;
#[doc = "Field `RID` reader - Revision ID \\[RID\\]\n\nAssigned by the manufacturer of the\n\ndevice to identify the revision RO\n\nSetting of this field Denali PCIe Core\n\nRegister Specification, PMC-Sierra\n\nVersion 3.4 202 number of the\n\ndevice. This field reflects the setting\n\nof the corresponding register in the\n\nconfiguration space of the\n\nassociated Physical Function."]
pub type RidR = crate::FieldReader;
#[doc = "Field `PIB` reader - Programming Interface Byte \\[PIB\\]\n\nIdentifies the register set layout of\n\nthe device. This field reflects the\n\nsetting of the corresponding register\n\nin the configuration space of the\n\nassociated Physical Function."]
pub type PibR = crate::FieldReader;
#[doc = "Field `SCC` reader - Sub-Class Code \\[SCC\\]\n\nIdentifies a sub-category within the\n\nselected function. This field reflects\n\nthe setting of the corresponding\n\nregister in the configuration space of\n\nthe associated Physical Function."]
pub type SccR = crate::FieldReader;
#[doc = "Field `CC` reader - Class Code \\[CC\\]\n\nIdentifies the function of the device.\n\nThis field reflects the setting of the\n\ncorresponding register in the\n\nconfiguration space of the\n\nassociated Physical Function."]
pub type CcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Revision ID \\[RID\\]\n\nAssigned by the manufacturer of the\n\ndevice to identify the revision RO\n\nSetting of this field Denali PCIe Core\n\nRegister Specification, PMC-Sierra\n\nVersion 3.4 202 number of the\n\ndevice. This field reflects the setting\n\nof the corresponding register in the\n\nconfiguration space of the\n\nassociated Physical Function."]
    #[inline(always)]
    pub fn rid(&self) -> RidR {
        RidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Programming Interface Byte \\[PIB\\]\n\nIdentifies the register set layout of\n\nthe device. This field reflects the\n\nsetting of the corresponding register\n\nin the configuration space of the\n\nassociated Physical Function."]
    #[inline(always)]
    pub fn pib(&self) -> PibR {
        PibR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sub-Class Code \\[SCC\\]\n\nIdentifies a sub-category within the\n\nselected function. This field reflects\n\nthe setting of the corresponding\n\nregister in the configuration space of\n\nthe associated Physical Function."]
    #[inline(always)]
    pub fn scc(&self) -> SccR {
        SccR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Class Code \\[CC\\]\n\nIdentifies the function of the device.\n\nThis field reflects the setting of the\n\ncorresponding register in the\n\nconfiguration space of the\n\nassociated Physical Function."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Revision ID and Class Code Register\n\nIdentifies the function of the device.\n\nThis field reflects the setting of the\n\ncorresponding register in the\n\nconfiguration space of the\n\nassociated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_revision_id_and_class_code::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfRevisionIdAndClassCodeSpec;
impl crate::RegisterSpec for PcieVfRevisionIdAndClassCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_revision_id_and_class_code::R`](R) reader structure"]
impl crate::Readable for PcieVfRevisionIdAndClassCodeSpec {}
#[doc = "`reset()` method sets PCIE_VF_REVISION_ID_AND_CLASS_CODE to value 0"]
impl crate::Resettable for PcieVfRevisionIdAndClassCodeSpec {
    const RESET_VALUE: u32 = 0;
}
