#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<Ahb1rstrSpec>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<Ahb1rstrSpec>;
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub type GpiocrstR = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub type GpiodrstR = crate::BitReader;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub type GpioerstR = crate::BitReader;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub type GpioerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub type GpiohrstR = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub type GpiohrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1RST` reader - DMA2 reset"]
pub type Dma1rstR = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA2 reset"]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub type Dma2rstR = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub type Dma2rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GpioerstR {
        GpioerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GpiohrstR {
        GpiohrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA2 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> Dma2rstR {
        Dma2rstR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GpioarstW<Ahb1rstrSpec> {
        GpioarstW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GpiobrstW<Ahb1rstrSpec> {
        GpiobrstW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GpiocrstW<Ahb1rstrSpec> {
        GpiocrstW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GpiodrstW<Ahb1rstrSpec> {
        GpiodrstW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GpioerstW<Ahb1rstrSpec> {
        GpioerstW::new(self, 4)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GpiohrstW<Ahb1rstrSpec> {
        GpiohrstW::new(self, 7)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CrcrstW<Ahb1rstrSpec> {
        CrcrstW::new(self, 12)
    }
    #[doc = "Bit 21 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> Dma1rstW<Ahb1rstrSpec> {
        Dma1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> Dma2rstW<Ahb1rstrSpec> {
        Dma2rstW::new(self, 22)
    }
}
#[doc = "AHB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstrSpec;
impl crate::RegisterSpec for Ahb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for Ahb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for Ahb1rstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for Ahb1rstrSpec {
    const RESET_VALUE: u32 = 0;
}
