#[doc = "Register `PCIE_PF_SR_IOV_CONTROL_AND_STATUS_S` reader"]
pub type R = crate::R<PciePfSrIovControlAndStatusSSpec>;
#[doc = "Register `PCIE_PF_SR_IOV_CONTROL_AND_STATUS_S` writer"]
pub type W = crate::W<PciePfSrIovControlAndStatusSSpec>;
#[doc = "Field `VFE` reader - VF Enable \\[VFE\\]\n\nThis bit must be set to enable the\n\nVFs associated with this PF."]
pub type VfeR = crate::BitReader;
#[doc = "Field `VFE` writer - VF Enable \\[VFE\\]\n\nThis bit must be set to enable the\n\nVFs associated with this PF."]
pub type VfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFME` reader - VF Migration Enable \\[VFME\\]\n\nNot supported. Hardwired to 0"]
pub type VfmeR = crate::BitReader;
#[doc = "Field `VFMIE` reader - VF Migration Interrupt Enable \\[VFMIE\\]\n\nNot supported. Hardwired to 0"]
pub type VfmieR = crate::BitReader;
#[doc = "Field `VFMSE` reader - VF Memory Space Enable \\[VFMSE\\]\n\nThis bit must be set to allow access\n\nto the memory space of the VFs\n\nassociated with this PF."]
pub type VfmseR = crate::BitReader;
#[doc = "Field `VFMSE` writer - VF Memory Space Enable \\[VFMSE\\]\n\nThis bit must be set to allow access\n\nto the memory space of the VFs\n\nassociated with this PF."]
pub type VfmseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARIE` reader - ARI Capable Hierarchy \\[ARIE\\]\n\nThis bit enables the ARI mode for\n\nVirtual Functions. This bit must be\n\nset when VF Enable is set. Valid only\n\nfor PF0"]
pub type ArieR = crate::BitReader;
#[doc = "Field `ARIE` writer - ARI Capable Hierarchy \\[ARIE\\]\n\nThis bit enables the ARI mode for\n\nVirtual Functions. This bit must be\n\nset when VF Enable is set. Valid only\n\nfor PF0"]
pub type ArieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]\n\nReserved"]
pub type R1R = crate::FieldReader<u16>;
#[doc = "Field `SSR` reader - SRIOV Status Register \\[SSR\\]\n\nNot implemented."]
pub type SsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - VF Enable \\[VFE\\]\n\nThis bit must be set to enable the\n\nVFs associated with this PF."]
    #[inline(always)]
    pub fn vfe(&self) -> VfeR {
        VfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VF Migration Enable \\[VFME\\]\n\nNot supported. Hardwired to 0"]
    #[inline(always)]
    pub fn vfme(&self) -> VfmeR {
        VfmeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VF Migration Interrupt Enable \\[VFMIE\\]\n\nNot supported. Hardwired to 0"]
    #[inline(always)]
    pub fn vfmie(&self) -> VfmieR {
        VfmieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VF Memory Space Enable \\[VFMSE\\]\n\nThis bit must be set to allow access\n\nto the memory space of the VFs\n\nassociated with this PF."]
    #[inline(always)]
    pub fn vfmse(&self) -> VfmseR {
        VfmseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARI Capable Hierarchy \\[ARIE\\]\n\nThis bit enables the ARI mode for\n\nVirtual Functions. This bit must be\n\nset when VF Enable is set. Valid only\n\nfor PF0"]
    #[inline(always)]
    pub fn arie(&self) -> ArieR {
        ArieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15 - Reserved \\[R1\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:31 - SRIOV Status Register \\[SSR\\]\n\nNot implemented."]
    #[inline(always)]
    pub fn ssr(&self) -> SsrR {
        SsrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - VF Enable \\[VFE\\]\n\nThis bit must be set to enable the\n\nVFs associated with this PF."]
    #[inline(always)]
    #[must_use]
    pub fn vfe(&mut self) -> VfeW<PciePfSrIovControlAndStatusSSpec> {
        VfeW::new(self, 0)
    }
    #[doc = "Bit 3 - VF Memory Space Enable \\[VFMSE\\]\n\nThis bit must be set to allow access\n\nto the memory space of the VFs\n\nassociated with this PF."]
    #[inline(always)]
    #[must_use]
    pub fn vfmse(&mut self) -> VfmseW<PciePfSrIovControlAndStatusSSpec> {
        VfmseW::new(self, 3)
    }
    #[doc = "Bit 4 - ARI Capable Hierarchy \\[ARIE\\]\n\nThis bit enables the ARI mode for\n\nVirtual Functions. This bit must be\n\nset when VF Enable is set. Valid only\n\nfor PF0"]
    #[inline(always)]
    #[must_use]
    pub fn arie(&mut self) -> ArieW<PciePfSrIovControlAndStatusSSpec> {
        ArieW::new(self, 4)
    }
}
#[doc = "SR-IOV Control and Status Registers\n\nNot implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_sr_iov_control_and_status_s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_sr_iov_control_and_status_s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfSrIovControlAndStatusSSpec;
impl crate::RegisterSpec for PciePfSrIovControlAndStatusSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_sr_iov_control_and_status_s::R`](R) reader structure"]
impl crate::Readable for PciePfSrIovControlAndStatusSSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_sr_iov_control_and_status_s::W`](W) writer structure"]
impl crate::Writable for PciePfSrIovControlAndStatusSSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_SR_IOV_CONTROL_AND_STATUS_S to value 0"]
impl crate::Resettable for PciePfSrIovControlAndStatusSSpec {
    const RESET_VALUE: u32 = 0;
}
