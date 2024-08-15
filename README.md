# Proyecto de Macros en Rust

Este proyecto demuestra cómo usar macros en Rust para generar automáticamente operaciones CRUD (Crear, Leer, Actualizar, Eliminar) para estructuras de datos. La macro `CrudOperations` se define en el crate `my_macro_crate` y se usa en el crate `my_project` para ilustrar cómo funciona en la práctica.

## ¿Qué es una Macro en Rust?

Las macros en Rust son una forma poderosa de generar código automáticamente. A diferencia de las funciones regulares, que son llamadas en tiempo de ejecución, las macros se expanden en tiempo de compilación. Esto permite que el compilador genere código basado en patrones definidos, reduciendo la repetición y simplificando la creación de código repetitivo.

### Ventajas de Usar Macros

- **Generación Automática de Código**: Las macros pueden generar código repetitivo automáticamente, reduciendo la necesidad de escribir el mismo código una y otra vez.
- **Reducción de Errores**: Al automatizar la generación de código, se minimiza el riesgo de errores humanos que pueden ocurrir al escribir manualmente código repetitivo.
- **Flexibilidad y Poder**: Las macros en Rust pueden tomar una gran variedad de formas y pueden ser diseñadas para hacer mucho más que simplemente generar código. Pueden realizar validaciones, transformaciones y más.
- **Mejora de la Legibilidad**: Al utilizar macros para manejar operaciones comunes, el código puede ser más limpio y legible, ya que los detalles repetitivos están encapsulados en la macro.

## Estructura del Proyecto

El proyecto se divide en dos partes principales:

- **`my_macro_crate`**: Un crate que define la macro derivada `CrudOperations`.
- **`my_project`**: Un crate que usa la macro `CrudOperations` para ilustrar cómo agregar automáticamente métodos CRUD a una estructura de datos.

## Contenido del Crate `my_macro_crate`

### `src/lib.rs`

Define la macro `CrudOperations`, que genera métodos CRUD para las estructuras de datos.

```rust
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CrudOperations)]
pub fn crud_operations_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            pub fn create(&self) {
                println!("Creating {:?}", self);
            }

            pub fn read(id: u32) {
                println!("Reading entity with id: {}", id);
            }

            pub fn update(&self) {
                println!("Updating {:?}", self);
            }

            pub fn delete(id: u32) {
                println!("Deleting entity with id: {}", id);
            }
        }
    };

    TokenStream::from(expanded)
}
```

## Contenido del Crate `my_project`

### `src/main.rs`

Utiliza la macro `CrudOperations` para añadir métodos CRUD a la estructura `User`.

```rust
use my_macro_crate::CrudOperations;

#[derive(CrudOperations, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    user.create();
    User::read(1);
    user.update();
    User::delete(1);
}
```

## Comandos de Makefile

El proyecto incluye un `Makefile` para facilitar el trabajo con macros y el código generado. Aquí se describen los comandos disponibles:

### `make expand`

Ejecuta el comando `cargo expand` para mostrar el código generado por las macros. Este comando es útil para ver cómo se expande la macro `CrudOperations` en el código Rust real.

### `make watch`

Monitorea los archivos en el directorio `src/` para detectar cambios y ejecuta automáticamente `cargo expand` cuando se detectan modificaciones. Esto es útil para desarrollar y depurar macros en tiempo real.

### `make clean`

Limpia los archivos compilados generados por Cargo. Este comando es opcional y se puede usar para limpiar el proyecto.

## Cómo Usar el Proyecto

1. **Construir el Crate de Macros**: Asegúrate de que el crate `my_macro_crate` esté construido. Navega al directorio `my_macro_crate` y ejecuta `cargo build`.

2. **Construir el Crate del Proyecto**: Navega al directorio `my_project` y agrega el crate `my_macro_crate` como una dependencia en `Cargo.toml`:

    ```toml
    [dependencies]
    my_macro_crate = { path = "../my_macro_crate" }
    ```

3. **Ejecutar el Proyecto**: Ejecuta `cargo run` en el directorio `my_project` para ver cómo se aplican los métodos CRUD generados a la estructura `User`.

4. **Monitorear Cambios**: Usa `make watch` en el directorio `my_project` para monitorear cambios en los archivos y ver cómo se actualiza el código generado automáticamente.

## Casos de Uso

- **Desarrollo de Bibliotecas**: Puedes usar macros para reducir la repetición en la creación de operaciones comunes como CRUD en bibliotecas y aplicaciones.
- **Automatización de Código**: Las macros permiten automatizar la generación de código boilerplate, lo que facilita el mantenimiento y la evolución del código.
- **Prototipos Rápidos**: Acelera el desarrollo de prototipos al generar automáticamente métodos y estructuras comunes sin escribir manualmente cada detalle.

## Créditos

Este proyecto fue creado por **Alejandro López**.

Este `README.md` proporciona una visión general del proyecto y de cómo interactuar con los comandos del `Makefile`. Asegúrate de seguir los pasos descritos para configurar y ejecutar correctamente el proyecto.
