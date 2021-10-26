use std::cmp::max;
use plotters::prelude::*;

struct PIDParameter {
    k_p: f32,
    k_i: f32,
    k_d: f32,
    tau: f32,   // = 1/ω mit ω als Grenzfrequenz des Tiefpasses im D-Regleranteil
    i_max: f32, // Anti-Windup Maximalwert
    i_min: f32  // Anti-Windup Minimalwert
}

struct InternerSpeicher {
    e_vorher: f32,
    i_vorher: f32,
    d_vorher: f32
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
                e_vorher: 0.0,
                i_vorher: 0.0,
                d_vorher: 0.0
            },
            enable_flag: false
        }
    }

    /*
    ############################
            ALGORITHMUS
    ############################
     */

    fn naechster_wert(&mut self, soll: [f32; 3], ist: [f32; 3]) -> [f32;  3] {
        if self.enable_flag {
            let e_k = soll - ist;
            let p = self.parameter.k_p * e_k;                                                                                           // P-Anteil

            let mut i = self.parameter.k_i*self.system_parameter.t_a/2.0 * (e_k + self.speicher.e_vorher) + self.speicher.i_vorher;     // I-Anteil ohne Anti-Windup
            // Anti-Windup
            if i > self.parameter.i_max {
                i = self.parameter.i_max;
            } else if i < self.parameter.i_min {
                i = self.parameter.i_min;
            }

            let d = 1.0/(2.0*self.parameter.tau + self.system_parameter.t_a) * (2.0*self.parameter.k_d*(e_k - self.speicher.e_vorher)
                + (2.0*self.parameter.tau - self.system_parameter.t_a)*self.speicher.d_vorher);                                             // D-Anteil

            self.speicher.e_vorher = e_k;
            self.speicher.i_vorher = i;
            self.speicher.d_vorher = d;

            return p + i + d;
        } else {
            return [0.0; 3];
        }
    }
}

// Konstanten
const SIGNALLAENGE: usize = 1000;
const T_ABTAST: f32 = 0.05;

fn main() {
    let mut pid: PID = setup_pid(T_ABTAST);
    pid.enable_flag = true;

    let mut input: [[f32; 3]; SIGNALLAENGE] = [[1.0; 3]; SIGNALLAENGE];
    input[0] = [0.0; 3];

    let mut output: [[f32; 3]; SIGNALLAENGE] = [[0.0; 3]; SIGNALLAENGE];

    pid.naechster_wert(input[0], [0.0; 3]);

    //regelkreis(&mut pid, input, &mut output);

    //plot(&input, &output, "Sprungantwort");
}

fn regelkreis(pid: &mut PID, input: [f32; SIGNALLAENGE], output: &mut [f32; SIGNALLAENGE]) {
    let mut u: [f32; SIGNALLAENGE] = [0.0; SIGNALLAENGE]; // Buffer für Ausgang des PID
    let mut ist: f32; // Buffer zur Behandliung der letzten Eingabe im Fall k==0

    // Regelkreis mit Tiefpassfilter
    for k in 0..SIGNALLAENGE {
        ist = if k>0 { output[k-1] } else { 0.0 };
        u[k] = pid.naechster_wert(input[k].to_owned(), ist);
        output[k] = if k > 0 {
            (T_ABTAST * (u[k - 1] + u[k]) - (T_ABTAST - 2.0) * output[k - 1]) / (2.0 + T_ABTAST)
        } else {
            T_ABTAST * u[k] / (2.0 + T_ABTAST)
        }
    }
}

fn setup_pid(t_abtast: f32) -> PID {
    PID::new(PIDParameter {
        k_p: 2.0,
        k_i: 2.0,
        k_d: 1.0,
        tau: 0.5,
        i_max: 10.0,
        i_min: -10.0
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
