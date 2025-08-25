use crossterm::{cursor, terminal::{size, Clear, ClearType}};
use std::{io::{stdout, Write}};
use crate::sys_info::disk_vec;


pub fn quad(user: &str, os: &str, kv: &str, os_v: &str, uptime: &str, distro: &str, real_name: &str, user_name: &str, architektur: &str, cpu_brand: &str, cpu_usage: &str, total_memory: &f64, used_memory: &f64, free_memory: &f64, gpu_name: &str, gpu_backend: &str){

    let char: String = "§".to_string();
    let long_text = build_text(user, os, kv, os_v, uptime, distro, user_name, architektur, cpu_brand, cpu_usage, total_memory, used_memory, free_memory, gpu_name, gpu_backend);

    let (width, height) = match size() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Terminalgröße konnte nicht ermittelt werden!");
            return;
        }
    };
    
    let mut stdout = stdout();
    crossterm::execute!(stdout, Clear(ClearType::All), cursor::Hide).unwrap();

    // Rechteckgröße und Position
    let rect_width = (width as f32 * 0.9) as u16;
    let rect_height = (height as f32 * 0.8) as u16;
    let start_x = (width - rect_width) / 2;
    let start_y = (height - rect_height) / 2;
    
    // --- Phase 1: Rechteck zeichnen ---
    
    // Zeichne die obere und untere Linie
    let horizontal_line: String = char.repeat(rect_width.into());
    crossterm::execute!(stdout, cursor::MoveTo(start_x, start_y)).unwrap();
    write!(stdout, "{}", horizontal_line).unwrap();
    crossterm::execute!(stdout, cursor::MoveTo(start_x, start_y + rect_height - 1)).unwrap();
    write!(stdout, "{}", horizontal_line).unwrap();

    // Zeichne die vertikalen Linien
    for y in (start_y + 1)..(start_y + rect_height - 1) {
        crossterm::execute!(stdout, cursor::MoveTo(start_x, y)).unwrap();
        write!(stdout, "{char}").unwrap();
        crossterm::execute!(stdout, cursor::MoveTo(start_x + rect_width - 1, y)).unwrap();
        write!(stdout, "{char}").unwrap();
    }
    
    // --- Phase 3: Text in das Rechteck schreiben und zentrieren ---

    let os_text = format!(" {} ", distro);
    let name_text = format!(" {} ", real_name);
    crossterm::execute!(stdout, cursor::MoveTo(start_x + 1, start_y)).unwrap();
    write!(stdout, "{}|{}", os_text, name_text).unwrap();
    
    // Die einzelnen Zeilen des Textes trennen.
    let lines: Vec<&str> = long_text.lines().collect();
    
    let mut line_y = start_y + 2;
    for line in lines {
        if line_y >= start_y + rect_height - 1 {
            break;
        }

        let max_text_width = rect_width.saturating_sub(2);
        let line_len = line.len();

        // Überprüfen, ob die Zeile zu lang ist, bevor Sie sie zentrieren.
        if (line_len as u16) > max_text_width {
            let text_x = start_x + 1;
            crossterm::execute!(stdout, cursor::MoveTo(text_x, line_y)).unwrap();
            write!(stdout, "{}", line).unwrap();

        } else {
            // Text zentrieren, wenn er nicht zu lang ist.
            let text_x = start_x + (rect_width - line_len as u16) / 2;
            crossterm::execute!(stdout, cursor::MoveTo(text_x, line_y)).unwrap();
            write!(stdout, "{}", line).unwrap();
        }

        line_y += 1;
    }
    
    crossterm::execute!(stdout, cursor::Show, cursor::MoveTo(0, height)).unwrap();
}


fn build_text(user: &str, os: &str, kv: &str, os_v: &str, uptime: &str, distro: &str, user_name: &str, architektur: &str, cpu_brand: &str, cpu_usage: &str, total_memory: &f64, used_memory: &f64, free_memory: &f64, gpu_name: &str, gpu_backend: &str) -> String {
    let disks_vec = disk_vec();

    let mut formatted_text = format!(
        "---System Information---\n\n\
         - Operating System: {} ({})\n\
         - Operating System Version: {}\n\
         - Kernel-Version: {}\n\
         - Distro: {}\n\
         - Uptime: {}\n\
         - User: {}\n\
         - Username: {}\n\n\
         ---System Usage---\n\n\
         - Memory: {:.2} GiB total / {:.2} GiB used / {:.2} GiB free\n\
         - CPU: {} ({}%)\n\
         - GPU: {} ({})\n\n\
         ---Disk Information---\n\n",
        os, architektur, os_v, kv, distro, uptime, user, user_name, total_memory, used_memory, free_memory, cpu_brand, cpu_usage, gpu_name, gpu_backend
    );

    for disk in &disks_vec {
        let total_gib = disk.2 as f64 / 1024.0 / 1024.0 / 1024.0;
        let free_gib = disk.3 as f64 / 1024.0 / 1024.0 / 1024.0;
        
        formatted_text.push_str(&format!(
            "- Disk: {} ({}) / Total: {:.2} GiB / Free: {:.2} GiB\n",
            disk.0, disk.1, total_gib, free_gib
        ));
    }
    
    formatted_text.push_str("\n---MORE COOMING SOON---\n\n");
    formatted_text
}
