use std::process::Command;

pub fn add_go() {
    let output = Command::new("go")
        .arg("install")
        .arg("golang.org/x/tools/gopls@latest")
        .output()
        .expect("\nFallo al ejecutar 'go install golang.org/x/tools/gopls@latest'");

    if output.status.success() {
        println!("Instalación de 'go install golang.org/x/tools/gopls@latest' completa");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error al ejecutar el comando:\n{stderr}")
    }
}
