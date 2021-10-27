use crate::vec3::Vek3;

pub struct PIDParameter {
    pub(crate) k_p: f32,
    pub(crate) k_i: f32,
    pub(crate) k_d: f32,
    pub(crate) tau: f32,   // = 1/ω mit ω als Grenzfrequenz des Tiefpasses im D-Regleranteil
    pub(crate) i_max: Vek3, // Anti-Windup Maximalwert
    pub(crate) i_min: Vek3  // Anti-Windup Minimalwert
}

struct InternerSpeicher {
    e_vorher: Vek3,
    i_vorher: Vek3,
    d_vorher: Vek3
}

pub struct SystemParameter {
    pub(crate) t_a: f32    // Abtastfrequenz
}

pub struct PID {
    parameter: PIDParameter,
    speicher: InternerSpeicher,
    system_parameter: SystemParameter,
    pub(crate) enable_flag: bool
}

impl PID {
    pub(crate) fn new(pid_params: PIDParameter, system_params: SystemParameter) -> PID {
        PID {
            parameter: pid_params,
            system_parameter: system_params,
            speicher: InternerSpeicher{
                e_vorher: Vek3::alles(0.0),
                i_vorher: Vek3::alles(0.0),
                d_vorher: Vek3::alles(0.0)
            },
            enable_flag: false
        }
    }

    /*
    ############################
            ALGORITHMUS
    ############################
     */

    pub(crate) fn anwenden(&mut self, soll: &Vek3, ist: &Vek3) -> Vek3 {
        if self.enable_flag {
            let e_k = Vek3::sub_e(soll, ist);

            // P-Anteil
            let p = Vek3::mul_s(self.parameter.k_p, &e_k);

            // I-Anteil
            let mut i = Vek3::add_e(
                &Vek3::mul_s(self.parameter.k_i*self.system_parameter.t_a/2.0,
                             &Vek3::add_e(&e_k, &self.speicher.e_vorher)),
                &self.speicher.i_vorher);

            i.set_min_max(&self.parameter.i_min, &self.parameter.i_max);    // Anti-Windup

            // D-Anteil
            let d = Vek3::mul_s(1.0/(2.0*self.parameter.tau + self.system_parameter.t_a),
                                &Vek3::add_e(
                                    &Vek3::mul_s(2.0*self.parameter.k_d,
                                                 &Vek3::sub_e(&e_k, &self.speicher.e_vorher)),
                                    &Vek3::mul_s(2.0*self.parameter.tau - self.system_parameter.t_a, &self.speicher.d_vorher)));

            self.speicher.e_vorher = e_k.clone();
            self.speicher.i_vorher = i.clone();
            self.speicher.d_vorher = d.clone();

            return Vek3::add3_e(&p, &i, &d);
        } else {
            return Vek3::alles(0.0);
        }
    }
}
