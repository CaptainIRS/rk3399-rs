#[doc = "Register `PCIE_CLIENT_BASIC_STATUS1` reader"]
pub type R = crate::R<PcieClientBasicStatus1Spec>;
#[doc = "Field `SYS_PAGE_SIZE` reader - System page size These bits reflect the setting of the System Page Size Register in the SR IOV capability of each PF. Bits \\[15:0\\]
reflect bits \\[15:0\\]
of System Page Size register of PF0"]
pub type SysPageSizeR = crate::FieldReader<u16>;
#[doc = "Field `FC_ST` reader - Function status These outputs indicate the states of the Command Register bits in the PCI configuration space of each Function. These outputs are used to enable requests and completions from the host logic. The assignment of bits is as follows: Bit 0: Function 0 IO Space Enable Bit 1: Function 0 Memory Space Enable Bit 2: Function 0 Bus Master Enable Bit 3: Function 0 INTx Disable and so on depending on the number of functions."]
pub type FcStR = crate::FieldReader;
#[doc = "Field `LINK_ST` reader - Link status Status of the PCI Express link. 2'b00 = No receivers detected. 2'b01 = Link training in progress. 2'b10 = Link up , DL initialization in progress. 2'b11 = Link up, DL initialization completed."]
pub type LinkStR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - System page size These bits reflect the setting of the System Page Size Register in the SR IOV capability of each PF. Bits \\[15:0\\]
reflect bits \\[15:0\\]
of System Page Size register of PF0"]
    #[inline(always)]
    pub fn sys_page_size(&self) -> SysPageSizeR {
        SysPageSizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Function status These outputs indicate the states of the Command Register bits in the PCI configuration space of each Function. These outputs are used to enable requests and completions from the host logic. The assignment of bits is as follows: Bit 0: Function 0 IO Space Enable Bit 1: Function 0 Memory Space Enable Bit 2: Function 0 Bus Master Enable Bit 3: Function 0 INTx Disable and so on depending on the number of functions."]
    #[inline(always)]
    pub fn fc_st(&self) -> FcStR {
        FcStR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Link status Status of the PCI Express link. 2'b00 = No receivers detected. 2'b01 = Link training in progress. 2'b10 = Link up , DL initialization in progress. 2'b11 = Link up, DL initialization completed."]
    #[inline(always)]
    pub fn link_st(&self) -> LinkStR {
        LinkStR::new(((self.bits >> 20) & 3) as u8)
    }
}
#[doc = "Basic status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_basic_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientBasicStatus1Spec;
impl crate::RegisterSpec for PcieClientBasicStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_basic_status1::R`](R) reader structure"]
impl crate::Readable for PcieClientBasicStatus1Spec {}
#[doc = "`reset()` method sets PCIE_CLIENT_BASIC_STATUS1 to value 0x0008_0001"]
impl crate::Resettable for PcieClientBasicStatus1Spec {
    const RESET_VALUE: u32 = 0x0008_0001;
}