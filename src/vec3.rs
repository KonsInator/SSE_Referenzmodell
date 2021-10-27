#[derive(Copy, Clone)]
pub struct Vek3(pub f32, pub f32, pub f32);

impl Vek3 {
    pub fn alles(n: f32) -> Vek3 { Vek3(n, n, n) }

    // elementweise Operationen
    pub fn add_e(a: &Vek3, b: &Vek3) -> Vek3 { Vek3(a.0+b.0, a.1+b.1, a.2+b.2) }
    pub fn sub_e(a: &Vek3, b: &Vek3) -> Vek3 { Vek3(a.0-b.0, a.1-b.1, a.2-b.2) }
    pub fn mul_e(a: &Vek3, b: &Vek3) -> Vek3 { Vek3(a.0*b.0, a.1*b.1, a.2*b.2) }
    pub fn add3_e(a: &Vek3, b: &Vek3, c: &Vek3) -> Vek3 { Vek3(a.0+b.0+c.0, a.1+b.1+c.1, a.2+b.2+c.2) }

    // skalare Operationen
    pub fn add_s(s: f32, vek: &Vek3) -> Vek3 { Vek3(s+vek.0, s+vek.1, s+vek.2) }
    pub fn sub_s(s: f32, vek: &Vek3) -> Vek3 { Vek3(s-vek.0, s-vek.1, s-vek.2) }
    pub fn mul_s(s: f32, vek: &Vek3) -> Vek3 { Vek3(s*vek.0, s*vek.1, s*vek.2) }

    // nicht-statische Funktionen
    pub fn set_min_max(&mut self, min: &Vek3, max: &Vek3) {
        self.0 = f32::max(min.0, f32::min(max.0, self.0));
        self.1 = f32::max(min.1, f32::min(max.1, self.1));
        self.2 = f32::max(min.2, f32::min(max.2, self.2));
    }
}
