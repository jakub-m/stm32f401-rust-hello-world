#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `LPDS` reader - Low-power deep sleep"]
pub type LpdsR = crate::BitReader;
#[doc = "Field `LPDS` writer - Low-power deep sleep"]
pub type LpdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS` reader - Power down deepsleep"]
pub type PddsR = crate::BitReader;
#[doc = "Field `PDDS` writer - Power down deepsleep"]
pub type PddsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF` reader - Clear wakeup flag"]
pub type CwufR = crate::BitReader;
#[doc = "Field `CWUF` writer - Clear wakeup flag"]
pub type CwufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` reader - Clear standby flag"]
pub type CsbfR = crate::BitReader;
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub type CsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PvdeR = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PVD level selection"]
pub type PlsR = crate::FieldReader;
#[doc = "Field `PLS` writer - PVD level selection"]
pub type PlsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DbpR = crate::BitReader;
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDS` reader - Flash power down in Stop mode"]
pub type FpdsR = crate::BitReader;
#[doc = "Field `FPDS` writer - Flash power down in Stop mode"]
pub type FpdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDC1` reader - ADCDC1"]
pub type Adcdc1R = crate::BitReader;
#[doc = "Field `ADCDC1` writer - ADCDC1"]
pub type Adcdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOS` reader - Regulator voltage scaling output selection"]
pub type VosR = crate::FieldReader;
#[doc = "Field `VOS` writer - Regulator voltage scaling output selection"]
pub type VosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LpdsR {
        LpdsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PddsR {
        PddsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CwufR {
        CwufR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CsbfR {
        CsbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DbpR {
        DbpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash power down in Stop mode"]
    #[inline(always)]
    pub fn fpds(&self) -> FpdsR {
        FpdsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - ADCDC1"]
    #[inline(always)]
    pub fn adcdc1(&self) -> Adcdc1R {
        Adcdc1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Regulator voltage scaling output selection"]
    #[inline(always)]
    pub fn vos(&self) -> VosR {
        VosR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LpdsW<CrSpec> {
        LpdsW::new(self, 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PddsW<CrSpec> {
        PddsW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CwufW<CrSpec> {
        CwufW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CsbfW<CrSpec> {
        CsbfW::new(self, 3)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PvdeW<CrSpec> {
        PvdeW::new(self, 4)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PlsW<CrSpec> {
        PlsW::new(self, 5)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DbpW<CrSpec> {
        DbpW::new(self, 8)
    }
    #[doc = "Bit 9 - Flash power down in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FpdsW<CrSpec> {
        FpdsW::new(self, 9)
    }
    #[doc = "Bit 13 - ADCDC1"]
    #[inline(always)]
    #[must_use]
    pub fn adcdc1(&mut self) -> Adcdc1W<CrSpec> {
        Adcdc1W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Regulator voltage scaling output selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VosW<CrSpec> {
        VosW::new(self, 14)
    }
}
#[doc = "power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
