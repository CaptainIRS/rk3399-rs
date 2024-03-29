#[doc = "Register `RKI2C_RXDATA5` reader"]
pub type R = crate::R<Rki2cRxdata5Spec>;
#[doc = "Field `RXDATA5` reader - data5 received\n\n32 bits data"]
pub type Rxdata5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data5 received\n\n32 bits data"]
    #[inline(always)]
    pub fn rxdata5(&self) -> Rxdata5R {
        Rxdata5R::new(self.bits)
    }
}
#[doc = "I2C rx data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cRxdata5Spec;
impl crate::RegisterSpec for Rki2cRxdata5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_rxdata5::R`](R) reader structure"]
impl crate::Readable for Rki2cRxdata5Spec {}
#[doc = "`reset()` method sets RKI2C_RXDATA5 to value 0"]
impl crate::Resettable for Rki2cRxdata5Spec {
    const RESET_VALUE: u32 = 0;
}
