use std::process::Command;

pub fn set_volume(id: String, direction: i8, value: f32) -> Result<(), String> {
    if value == 0.0 {return Err("\"value\" of 0 not allowed".to_owned());}
    
    let modifier = match (direction, value) {
        (0, v) => format!("{}%", v.abs()),
        (d, v) => {
            let x = f32::from(d) * v;
            
            format!("{}%{}", x.abs(), if x.signum() > 0.0 {"+"} else {"-"})
        }
    };
        
    
    let output = Command::new("wpctl")
        .arg("set-volume")
        .arg(id)
        .arg("--limit=1")
        .arg(modifier)
        .output();

    match output {
        Ok(s) => if s.status.success() { Ok(()) } else { Err(format!("Failed to set volume!\nStatus: {}", s.status))},
        Err(err) => Err(format!("Error running command!\nReason: {}", err.kind()))
    }
}