use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader, SeekFrom};
use std::path::Path;

// Definimos la estructura de una tarea
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

// Función para crear una nueva tarea
fn create_task(id: u32, description: String, completed: bool) -> Task {
    Task {
        id,
        description,
        completed,
    }
}

// Función para leer todas las tareas desde un archivo JSON
fn read_tasks(file_path: &str) -> Result<Vec<Task>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let tasks: Vec<Task> = serde_json::from_reader(reader)?;

    Ok(tasks)
}

// Función para escribir todas las tareas en un archivo JSON
fn write_tasks(file_path: &str, tasks: &Vec<Task>) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    serde_json::to_writer(file, tasks)?;

    Ok(())
}

fn main() {
    // Definimos el nombre del archivo para almacenar las tareas
    let file_path = "tasks.json";

    let mut tasks = Vec::new();

    // Intentamos leer las tareas desde el archivo
    match read_tasks(&file_path) {
        Ok(t) => tasks = t,
        Err(_) => println!("No se pudieron cargar las tareas existentes."),
    }

    // Agregamos una tarea
    let task1 = create_task(1, String::from("Hacer la compra"), false);
    tasks.push(task1);

    // Actualizamos la tarea (marcándola como completada)
    tasks[0].completed = true;

    // Borramos una tarea
    tasks.retain(|task| task.id != 1);

    // Escribimos las tareas en el archivo
    match write_tasks(&file_path, &tasks) {
        Ok(_) => println!("Tareas guardadas correctamente."),
        Err(_) => println!("Hubo un error al guardar las tareas."),
    }
}
