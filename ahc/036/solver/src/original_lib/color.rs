pub const COLOR_HOTTEST_HSLA: &str = "hsl(349, 100%, 56%, 0.8)"; // #ff1e46 * 0.8
pub const COLOR_COOLEST_HSLA: &str = "hsl(210, 100%, 56%, 0.8)"; // #1e90ff * 0.8

#[derive(Debug, Clone, Copy)]
pub struct HslaColor {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct RGBAColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

impl HslaColor {
    pub fn decode_from(s: &str) -> HslaColor {
        let s2 = s
            .trim_start_matches("hsl(")
            .trim_end_matches(')')
            .split(',')
            .collect::<Vec<_>>();
        let h = s2[0].parse::<f64>().unwrap();
        let s = s2[1].trim().trim_end_matches('%').parse::<f64>().unwrap();
        let l = s2[2].trim().trim_end_matches('%').parse::<f64>().unwrap();
        let a = s2[3].trim().parse::<f64>().unwrap();
        HslaColor { h, s, l, a }
    }

    pub fn encode_from(c: HslaColor) -> String {
        format!("hsla({}, {}%, {}%, {})", c.h, c.s, c.l, c.a)
    }

    pub fn encode(&self) -> String {
        HslaColor::encode_from(*self)
    }

    fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
        let t = if t < 0.0 {
            t + 1.0
        } else if t > 1.0 {
            t - 1.0
        } else {
            t
        };
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 1.0 / 2.0 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    }

    pub fn convert_to_rgba(&self) -> RGBAColor {
        let h = self.h / 360.0;
        let s = self.s / 100.0;
        let l = self.l / 100.0;

        let (r, g, b) = if s == 0.0 {
            let gray = (l * 255.0).round() as u8;
            (gray, gray, gray)
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            let r = Self::hue_to_rgb(p, q, h + 1.0 / 3.0);
            let g = Self::hue_to_rgb(p, q, h);
            let b = Self::hue_to_rgb(p, q, h - 1.0 / 3.0);

            (
                (r * 255.0).round() as u8,
                (g * 255.0).round() as u8,
                (b * 255.0).round() as u8,
            )
        };

        RGBAColor { r, g, b, a: self.a }
    }
}

impl RGBAColor {
    pub fn encode(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

pub fn get_hsla_gradient(cnt: usize) -> Vec<HslaColor> {
    let mut colors = vec![];
    let hottest = HslaColor::decode_from(COLOR_HOTTEST_HSLA);
    let coolest = HslaColor::decode_from(COLOR_COOLEST_HSLA);
    let mut h = coolest.h;
    let mut s = coolest.s;
    let mut l = coolest.l;
    let mut a = coolest.a;
    let dh = (coolest.h - hottest.h + 360.0) / (cnt as f64);
    let ds = (hottest.s - coolest.s) / (cnt as f64);
    let dl = (hottest.l - coolest.l) / (cnt as f64);
    let da = (hottest.a - coolest.a) / (cnt as f64);
    for _ in 0..cnt {
        colors.push(HslaColor { h, s, l, a });
        h = (h - dh) % 360.0;
        s += ds;
        l += dl;
        a += da;
    }
    colors
}

pub fn get_rgba_gradient(cnt: usize) -> Vec<RGBAColor> {
    let hsla_colors = get_hsla_gradient(cnt);
    hsla_colors
        .iter()
        .map(|color| color.convert_to_rgba())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsla_to_rgba() {
        let hsla = HslaColor {
            h: 0.0,
            s: 100.0,
            l: 50.0,
            a: 1.0,
        };
        let rgba = hsla.convert_to_rgba();
        assert_eq!(rgba.r, 255);
        assert_eq!(rgba.g, 0);
        assert_eq!(rgba.b, 0);
        assert_eq!(rgba.a, 1.0);
    }
}
