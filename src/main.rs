use plotters::prelude::*;

use pid::{PID, PIDParameter, SystemParameter};
use vec3::Vek3;

mod vec3;
mod pid;

// Konstanten
const SIGNALLAENGE: usize = 100;
const T_ABTAST: f32 = 0.05;

fn main() {
    let mut pid: PID = setup_pid(T_ABTAST);
    pid.enable_flag = true;

    let mut input: [Vek3; SIGNALLAENGE] = [Vek3(1.0, 0.0, 0.0); SIGNALLAENGE];
    input[0] = Vek3::alles(0.0);

    let mut output: [Vek3; SIGNALLAENGE] = [Vek3::alles(0.0); SIGNALLAENGE];

    regelkreis(&mut pid, &input, &mut output);

    for v in &output {
        println!("[{} {} {}]", v.0, v.1, v.2);
    }

    //plot(&input, &output, "Sprungantwort");
}

fn regelkreis(pid: &mut PID, input: &[Vek3; SIGNALLAENGE], output: &mut [Vek3; SIGNALLAENGE]) {
    let mut u: [Vek3; SIGNALLAENGE] = [Vek3::alles(0.0); SIGNALLAENGE]; // Buffer für Ausgang des PID
    let mut ist: Vek3; // Buffer zur Behandliung der letzten Eingabe im Fall k==0

    // Regelkreis mit Tiefpassfilter
    for k in 0..SIGNALLAENGE {
        ist = if k>0 { output[k-1] } else { Vek3::alles(0.0) };
        u[k] = pid.anwenden(&input[k], &ist);
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
