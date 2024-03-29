#[doc = "Register `PHY_I2CM_ADDRESS` reader"]
pub type R = crate::R<PhyI2cmAddressSpec>;
#[doc = "Register `PHY_I2CM_ADDRESS` writer"]
pub type W = crate::W<PhyI2cmAddressSpec>;
#[doc = "Field `ADDRESS` reader - Register address for read and write operations"]
pub type AddressR = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - Register address for read and write operations"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Register address for read and write operations"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register address for read and write operations"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<PhyI2cmAddressSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "PHY I2C Address Configuration Register\n\nThis register writes the address for read and write operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmAddressSpec;
impl crate::RegisterSpec for PhyI2cmAddressSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_address::R`](R) reader structure"]
impl crate::Readable for PhyI2cmAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_address::W`](W) writer structure"]
impl crate::Writable for PhyI2cmAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_ADDRESS to value 0"]
impl crate::Resettable for PhyI2cmAddressSpec {
    const RESET_VALUE: u8 = 0;
}
