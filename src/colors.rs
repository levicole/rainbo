pub struct HSV {
    pub h: u16,
    pub s: f64,
    pub v: f64
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl HSV {
    pub fn new(h: u16, s: f64, v: f64) -> Result<HSV, &'static str> {
        if s > 1.0 || v > 1.0 {
            return Err("Saturation and value should be between 0 and 1.0")
        }
        Ok(HSV {
            h: h,
            s: s,
            v: v
        })
    }

    pub fn into_rgb(&self) -> RGB {
        let h = (self.h as f64 / 60.0).floor();
        let f = (self.h as f64 / 360.0) * 6.0 - h;
        let p = self.v * (1.0 - self.s);
        let q = self.v * (1.0 - f * self.s);
        let t = self.v * (1.0 - (1.0 - f) * self.s);

        let (r,g,b) = match h {
            0.0 => (self.v, t, p),
            1.0 => (q, self.v, p),
            2.0 => (p, self.v, t),
            3.0 => (p, q, self.v),
            4.0 => (t, p, self.v),
            5.0 => (self.v, p, q),
            _   => (0.0, 0.0, 0.0)
        };

        RGB {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_into_rgb_for_pink() {
        let pink = HSV::new(300, 0.4, 1.0).unwrap();
        let pink_rgb = pink.into_rgb();

        assert_eq!(pink_rgb.r, 255);
        assert_eq!(pink_rgb.g, 153);
        assert_eq!(pink_rgb.b, 255);
    }

    #[test]
    fn test_hsv_into_rgb_for_teal() {
        let pink = HSV::new(180, 1.0, 0.6784).unwrap();
        let pink_rgb = pink.into_rgb();

        assert_eq!(pink_rgb.r, 0);
        assert_eq!(pink_rgb.g, 172);
        assert_eq!(pink_rgb.b, 172);
    }
}
