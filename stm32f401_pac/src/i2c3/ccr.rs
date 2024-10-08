#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)"]
pub type CcrR = crate::FieldReader<u16>;
#[doc = "Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)"]
pub type CcrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DUTY` reader - Fast mode duty cycle"]
pub type DutyR = crate::BitReader;
#[doc = "Field `DUTY` writer - Fast mode duty cycle"]
pub type DutyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_S` reader - I2C master mode selection"]
pub type FSR = crate::BitReader;
#[doc = "Field `F_S` writer - I2C master mode selection"]
pub type FSW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&self) -> CcrR {
        CcrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&self) -> FSR {
        FSR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CcrW<CcrSpec> {
        CcrW::new(self, 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DutyW<CcrSpec> {
        DutyW::new(self, 14)
    }
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn f_s(&mut self) -> FSW<CcrSpec> {
        FSW::new(self, 15)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
