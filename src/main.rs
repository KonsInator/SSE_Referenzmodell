mod vec3;

use vec3::Vek3;
use plotters::prelude::*;

struct PIDParameter {
    k_p: f32,
    k_i: f32,
    k_d: f32,
    tau: f32,   // = 1/ω mit ω als Grenzfrequenz des Tiefpasses im D-Regleranteil
    i_max: Vek3, // Anti-Windup Maximalwert
    i_min: Vek3  // Anti-Windup Minimalwert
}

struct InternerSpeicher {
    e_vorher: Vek3,
    i_vorher: Vek3,
    d_vorher: Vek3
}

struct SystemParameter {
    t_a: f32    // Abtastfrequenz
}

struct PID {
    parameter: PIDParameter,
    speicher: InternerSpeicher,
    system_parameter: SystemParameter,
    enable_flag: bool
}

impl PID {
    fn new(pid_params: PIDParameter, system_params: SystemParameter) -> PID {
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

    fn naechster_wert(&mut self, soll: &Vek3, ist: &Vek3) -> Vek3 {
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
                                    &Vek3::add_s(2.0*self.parameter.k_d,
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

// Konstanten
const SIGNALLAENGE: usize = 1000;
const T_ABTAST: f32 = 0.05;

fn main() {
    let mut pid: PID = setup_pid(T_ABTAST);
    pid.enable_flag = true;

    let mut input: [Vek3; SIGNALLAENGE] = [Vek3::alles(1.0); SIGNALLAENGE];
    input[0] = Vek3::alles(0.0);

    let mut output: [Vek3; SIGNALLAENGE] = [Vek3::alles(0.0); SIGNALLAENGE];

    pid.naechster_wert(&Vek3::alles(1.0), &Vek3::alles(0.0));

    regelkreis(&mut pid, &input, &mut output);

    output.iter().map(|v: &Vek3| println!("[{} {} {}]", v.0, v.1, v.2));

    //plot(&input, &output, "Sprungantwort");
}

fn regelkreis(pid: &mut PID, input: &[Vek3; SIGNALLAENGE], output: &mut [Vek3; SIGNALLAENGE]) {
    let mut u: [Vek3; SIGNALLAENGE] = [Vek3::alles(0.0); SIGNALLAENGE]; // Buffer für Ausgang des PID
    let mut ist: Vek3; // Buffer zur Behandliung der letzten Eingabe im Fall k==0

    // Regelkreis mit Tiefpassfilter
    for k in 0..SIGNALLAENGE {
        ist = if k>0 { output[k-1] } else { Vek3::alles(0.0) };
        u[k] = pid.naechster_wert(&input[k], &ist);
        output[k] = if k > 0 {
            Vek3::mul_s(1.0 / (2.0 + T_ABTAST),
                        &Vek3::sub_e(
                            &Vek3::mul_s(T_ABTAST,
                                         &Vek3::add_e(&u[k - 1], &u[k])),
                            &Vek3::mul_s(T_ABTAST - 2.0, &output[k - 1])))
        } else {
            Vek3::mul_s(T_ABTAST / (2.0 + T_ABTAST), &u[k])
        }
    }
}

fn setup_pid(t_abtast: f32) -> PID {
    PID::new(PIDParameter {
        k_p: 2.0,
        k_i: 2.0,
        k_d: 1.0,
        tau: 0.5,
        i_max: Vek3::alles(10.0),
        i_min: Vek3::alles(-10.0)
    }, SystemParameter {
        t_a: t_abtast
    })
}

fn plot(input: &[f32; SIGNALLAENGE], output: &[f32; SIGNALLAENGE], name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("PID_PLOT.svg", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption(name, ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_ranged(0f32..(SIGNALLAENGE as f32 - 1.0),
                      0f32..f32::max(
                          input.iter().cloned().fold(input[0],f32::max),
                          output.iter().cloned().fold(output[0],f32::max))*1.2)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    chart.draw_series(LineSeries::new((0..SIGNALLAENGE).map(|k| (k as f32, output[k])), &RED,))?
        .label("output");
    chart.draw_series(LineSeries::new((0..SIGNALLAENGE).map(|k| (k as f32, input[k])), &BLUE,))?
        .label("input");

    Ok(())
}
