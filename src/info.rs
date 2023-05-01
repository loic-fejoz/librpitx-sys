use librpitx_sys::*;
use std::{thread, time};

enum ClockKind {
    clk_gnd = 0,
    clk_osc,
    clk_debug0,
    clk_debug1,
    clk_plla,
    clk_pllc,
    clk_plld,
    clk_hdmi,
}

fn info(freq: u64) {
    unsafe {
        let GPPUDCLK0 = (0x98 / 4);
        let mut genpio = generalgpio::new();
        let gpio_pull = genpio._base.gpioreg.add(GPPUDCLK0).read();
        eprintln!("GPIOPULL ={gpio_pull}\n");

        // const PULL_OFF = 0;
        // const PULL_DOWN = 1;
        // const PULL_UP = 2;
        /*genpio.gpioreg[GPPUD] = 0; //PULL_DOWN;
        usleep(150);
        genpio.gpioreg[GPPUDCLK0] = (1 << 4); //GPIO CLK is GPIO 4
        usleep(150);
        genpio.gpioreg[GPPUDCLK0] = (0); //GPIO CLK is GPIO 4
        */
        //genpio.setpulloff(4);

        let mut pad = padgpio::new();
        pad.setlevel(7);

        let mut clk = clkgpio::new();
        clk.print_clock_tree();
        /* // THis fractional works on pi4
        clk.SetPllNumber(clk_plld, 2);
        clk.enableclk(4);
        */
        clk.SetPllNumber(ClockKind::clk_pllc as i32, 2);
        clk.SetAdvancedPllMode(true);
        clk.enableclk(4);
        //clk.SetAdvancedPllMode(true);
        //clk.SetPLLMasterLoop(0,4,0);
        //clk.Setppm(+7.7);
        clk.SetCenterFrequency(freq, 1000);
        clk.SetFrequency(0.0);
        let freqresolution = clk.GetFrequencyResolution();
        let real_freq = clk.GetRealFrequency(0.0);
        eprintln!("Frequency resolution={freqresolution} Error freq={real_freq}");
        let deviation = 0;
        for i in 0..1000 {
            clk.SetFrequency((i as f64) * 100.0);
            let wait_time = time::Duration::from_micros(10000);
            thread::sleep(wait_time);
        }
        let wait_time = time::Duration::from_secs(10);
        thread::sleep(wait_time);
        clk.disableclk(4);
    };
}

fn main() {
    info(144200000u64);
}
