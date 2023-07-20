#[cfg(target_os = "emscripten")]
mod emscripten_wrappers;
#[cfg(target_os = "emscripten")]
use emscripten_wrappers::emscripten;

use std::path::Path;
use std::time::*;

fn main() {
    #[cfg(target_os = "emscripten")]
    let (window_width, window_height) = emscripten::get_canvas_size();

    // After initializing everything, in the web version, we should delete the spinner from in front of the canvas
    #[cfg(target_os = "emscripten")]
    {
        emscripten::sleep(1500); // Just to demonstrate that the spinner works
                                 //emscripten::exec("let spinner = document.getElementById('spinner'); spinner.remove();");
    }

    'mainloop: loop {
        let t1 = Instant::now();

        let frame_time = t1.elapsed().as_millis();
        if frame_time < u32::MAX as u128 {
            let fps = 1_000u32 / 60u32;
            if fps > frame_time as u32 {
                let mut sleep_for = fps - frame_time as u32;

                let full_sleep_for = sleep_for;
                let mut early_wake = 0;

                loop {
                    let sleep_start = Instant::now();

                    #[cfg(target_os = "emscripten")]
                    emscripten::sleep(sleep_for);

                    #[cfg(not(target_os = "emscripten"))]
                    std::thread::sleep(Duration::from_millis(sleep_for as u64));

                    let elapsed = sleep_start.elapsed().as_millis() as u32;

                    if elapsed < sleep_for {
                        sleep_for -= elapsed;
                        early_wake += 1;
                        continue;
                    }

                    if elapsed > sleep_for {
                        eprintln!(
                            "slept too long! wanted to sleep for {}, but slept for {}",
                            sleep_for, elapsed
                        );
                    }

                    break;
                }

                if early_wake > 2 {
                    eprintln!(
                        "woke up early {} times while trying to sleep {} ms",
                        early_wake, full_sleep_for
                    );
                }
            }
        }

        println!("FPS: {}", 1_000u128 / (t1.elapsed()).as_millis().max(1));
    }
}
