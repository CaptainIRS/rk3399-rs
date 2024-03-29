#[doc = "Register `PCIE_RC_REVISION_ID_AND_CLASS_CODE` reader"]
pub type R = crate::R<PcieRcRevisionIdAndClassCodeSpec>;
#[doc = "Field `RID` reader - Revision ID \\[RID\\]\n\nAssigned by the manufacturer of the\n\ndevice to identify the revision\n\nnumber of the device. This field can\n\nbe written from the APB bus by\n\nsetting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type RidR = crate::FieldReader;
#[doc = "Field `PIB` reader - Programming Interface Byte \\[PIB\\]\n\nIdentifies the register set layout of\n\nthe device. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite."]
pub type PibR = crate::FieldReader;
#[doc = "Field `SCC` reader - Sub-Class Code \\[SCC\\]\n\nIdentifies a sub-category within the\n\nselected function. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type SccR = crate::FieldReader;
#[doc = "Field `CC` reader - Class Code \\[CC\\]\n\nIdentifies the function of the device.\n\nOn power- up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type CcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Revision ID \\[RID\\]\n\nAssigned by the manufacturer of the\n\ndevice to identify the revision\n\nnumber of the device. This field can\n\nbe written from the APB bus by\n\nsetting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn rid(&self) -> RidR {
        RidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Programming Interface Byte \\[PIB\\]\n\nIdentifies the register set layout of\n\nthe device. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite."]
    #[inline(always)]
    pub fn pib(&self) -> PibR {
        PibR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sub-Class Code \\[SCC\\]\n\nIdentifies a sub-category within the\n\nselected function. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn scc(&self) -> SccR {
        SccR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Class Code \\[CC\\]\n\nIdentifies the function of the device.\n\nOn power- up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Revision ID and Class Code Register\n\nIdentifies the function of the device.\n\nOn power- up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nwritten from the APB bus by setting\n\n\\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_revision_id_and_class_code::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcRevisionIdAndClassCodeSpec;
impl crate::RegisterSpec for PcieRcRevisionIdAndClassCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_revision_id_and_class_code::R`](R) reader structure"]
impl crate::Readable for PcieRcRevisionIdAndClassCodeSpec {}
#[doc = "`reset()` method sets PCIE_RC_REVISION_ID_AND_CLASS_CODE to value 0"]
impl crate::Resettable for PcieRcRevisionIdAndClassCodeSpec {
    const RESET_VALUE: u32 = 0;
}
