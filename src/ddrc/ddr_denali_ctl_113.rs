#[doc = "Register `DDR_DENALI_CTL_113` reader"]
pub type R = crate::R<DdrDenaliCtl113Spec>;
#[doc = "Register `DDR_DENALI_CTL_113` writer"]
pub type W = crate::W<DdrDenaliCtl113Spec>;
#[doc = "Field `DFS_PHY_REG_WRITE_DATA_F0` reader - Register data which will be written during a frequency change for frequency copy 0."]
pub type DfsPhyRegWriteDataF0R = crate::FieldReader<u32>;
#[doc = "Field `DFS_PHY_REG_WRITE_DATA_F0` writer - Register data which will be written during a frequency change for frequency copy 0."]
pub type DfsPhyRegWriteDataF0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register data which will be written during a frequency change for frequency copy 0."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_data_f0(&self) -> DfsPhyRegWriteDataF0R {
        DfsPhyRegWriteDataF0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register data which will be written during a frequency change for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_data_f0(&mut self) -> DfsPhyRegWriteDataF0W<DdrDenaliCtl113Spec> {
        DfsPhyRegWriteDataF0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_113::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_113::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl113Spec;
impl crate::RegisterSpec for DdrDenaliCtl113Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_113::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl113Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_113::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl113Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_113 to value 0"]
impl crate::Resettable for DdrDenaliCtl113Spec {
    const RESET_VALUE: u32 = 0;
}
